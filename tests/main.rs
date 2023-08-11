use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use wasmtime::component::{Component, Instance, Linker};
use wasmtime::{Config, Engine, Store};
use wit_component::ComponentEncoder;
use wit_parser::Resolve;

mod demo;
mod ser;

wasmtime::component::bindgen!(in "tests/crates/wasi_snapshot_preview1/wit");

#[derive(Default)]
struct Wasi<T>(T);

impl<T> testwasi::Host for Wasi<T> {
    fn log(&mut self, bytes: Vec<u8>) -> Result<()> {
        std::io::stdout().write_all(&bytes)?;
        Ok(())
    }

    fn log_err(&mut self, bytes: Vec<u8>) -> Result<()> {
        std::io::stderr().write_all(&bytes)?;
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
fn run_test<T, U>(
    name: &str,
    add_to_linker: fn(&mut Linker<Wasi<T>>) -> Result<()>,
    instantiate: fn(&mut Store<Wasi<T>>, &Component, &Linker<Wasi<T>>) -> Result<(U, Instance)>,
    test: fn(U, &mut Store<Wasi<T>>) -> Result<()>,
) -> Result<()>
where
    T: Default,
{
    run_test_from_dir(name, name, add_to_linker, instantiate, test)
}

#[allow(clippy::type_complexity)]
fn run_test_from_dir<T, U>(
    dir_name: &str,
    name: &str,
    add_to_linker: fn(&mut Linker<Wasi<T>>) -> Result<()>,
    instantiate: fn(&mut Store<Wasi<T>>, &Component, &Linker<Wasi<T>>) -> Result<(U, Instance)>,
    test: fn(U, &mut Store<Wasi<T>>) -> Result<()>,
) -> Result<()>
where
    T: Default,
{
    // Create an engine with caching enabled to assist with iteration in this
    // project.
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    for wasm in tests(name, dir_name)? {
        let component = Component::from_file(&engine, &wasm)?;
        let mut linker = Linker::new(&engine);

        add_to_linker(&mut linker)?;
        crate::testwasi::add_to_linker(&mut linker, |x| x)?;
        let mut store = Store::new(&engine, Wasi::default());
        let (exports, _) = instantiate(&mut store, &component, &linker)?;

        println!("testing {wasm:?}");
        test(exports, &mut store)?;
    }

    Ok(())
}

fn tests(name: &str, dir_name: &str) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();

    let mut dir = PathBuf::from("./tests");
    dir.push(dir_name);

    let mut resolve = Resolve::new();
    let (pkg, _files) = resolve.push_dir(&dir).unwrap();
    let _world = resolve.select_world(pkg, None).unwrap();

    let mut rust = Vec::new();
    for file in dir.read_dir()? {
        let path = file?.path();
        if matches!(path.file_name(), Some(name) if name == "wasm.rs") {
            rust.push(path);
        }
    }

    let mut out_dir = std::env::current_exe()?;
    out_dir.pop();
    out_dir.pop();
    out_dir.pop();
    out_dir.push("runtime-tests");
    out_dir.push(name);

    println!("wasi adapter = {:?}", test_artifacts::ADAPTER);
    let wasi_adapter = std::fs::read(test_artifacts::ADAPTER)?;

    drop(std::fs::remove_dir_all(&out_dir));
    std::fs::create_dir_all(&out_dir)?;

    if !rust.is_empty() {
        let core = test_artifacts::WASMS
            .iter()
            .map(PathBuf::from)
            .find(|p| match p.file_stem().and_then(|s| s.to_str()) {
                Some(n) => n == name,
                None => false,
            })
            .unwrap();
        println!("rust core module = {core:?}");
        let module = std::fs::read(&core)?;
        let wasm = ComponentEncoder::default()
            .module(&module)?
            .validate(true)
            .adapter("wasi_snapshot_preview1", &wasi_adapter)?
            .encode()?;

        let dst = out_dir.join("rust.wasm");
        println!("rust component {dst:?}");
        std::fs::write(&dst, wasm)?;
        result.push(dst);
    }

    Ok(result)
}
