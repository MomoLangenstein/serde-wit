use anyhow::Result;
use std::io::Write;
use std::path::Path;
use temp_file::TempFile;
use wasm_compose::composer::ComponentComposer;
use wasm_compose::config::Config as ComposerConfig;
use wasmtime::component::{Component, Instance, Linker};
use wasmtime::{Config, Engine, Store};
use wit_component::ComponentEncoder;

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
    path: &Path,
    dependencies: &[&Path],
    add_to_linker: fn(&mut Linker<Wasi<T>>) -> Result<()>,
    instantiate: fn(&mut Store<Wasi<T>>, &Component, &Linker<Wasi<T>>) -> Result<(U, Instance)>,
    test: fn(U, &mut Store<Wasi<T>>) -> Result<()>,
) -> Result<()>
where
    T: Default,
{
    // Create an engine with caching enabled to assist with iteration in this
    //  project.
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);

    add_to_linker(&mut linker)?;
    crate::testwasi::add_to_linker(&mut linker, |x| x)?;
    let mut store = Store::new(&engine, Wasi::default());

    println!("loading root {path:?}");
    let root_component = compose_wasi_components(path, dependencies, &engine)?;
    println!("instantiating root {path:?}");
    let (exports, _) = instantiate(&mut store, &root_component, &linker)?;

    println!("testing {path:?}");
    test(exports, &mut store)?;

    Ok(())
}

fn compile_wasi_component(path: &Path) -> Result<Vec<u8>> {
    let wasi_adapter = std::fs::read(test_artifacts::ADAPTER)?;
    let resource_adapter = std::fs::read(test_artifacts::RESOURCE)?;
    let module = std::fs::read(path)?;

    // let module = replace(&module, b"[export]", b"_export_");

    ComponentEncoder::default()
        .module(&module)?
        .validate(false)
        .adapter("wasi_snapshot_preview1", &wasi_adapter)?
        // .adapter("_export_test:ser/serializer", &resource_adapter)?
        .encode()
}

fn replace<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq
{
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();

    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            i += to_len;
        } else {
            i += 1;
        }
    }

    result
}

fn compile_wasi_component_to_file(path: &Path) -> Result<TempFile> {
    let wasm = compile_wasi_component(path)?;

    TempFile::new()?
        .panic_on_cleanup_error()
        .with_contents(&wasm)
        .map_err(anyhow::Error::from)
}

fn compose_wasi_components(
    path: &Path,
    dependencies: &[&Path],
    engine: &Engine,
) -> Result<Component> {
    let wasm = if dependencies.is_empty() {
        compile_wasi_component(path)?
    } else {
        let root = compile_wasi_component_to_file(path)?;

        let dependencies = dependencies
            .iter()
            .copied()
            .map(compile_wasi_component_to_file)
            .collect::<Result<Vec<_>, _>>()?;

        let config = ComposerConfig {
            dir: root
                .path()
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_default(),
            definitions: dependencies
                .iter()
                .map(|dependency| dependency.path().to_path_buf())
                .collect(),
            ..Default::default()
        };

        ComponentComposer::new(root.path(), &config).compose()?
    };

    Component::from_binary(engine, &wasm)
}
