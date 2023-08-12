use anyhow::Result;
use std::io::Write;
use std::path::Path;
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

    let component = load_wasi_component(path, &engine)?;

    let mut linker = Linker::new(&engine);

    add_to_linker(&mut linker)?;
    crate::testwasi::add_to_linker(&mut linker, |x| x)?;
    let mut store = Store::new(&engine, Wasi::default());
    let (exports, _) = instantiate(&mut store, &component, &linker)?;

    println!("testing {path:?}");
    test(exports, &mut store)?;

    Ok(())
}

fn load_wasi_component(path: &Path, engine: &Engine) -> Result<Component> {
    let wasi_adapter = std::fs::read(test_artifacts::ADAPTER)?;
    let module = std::fs::read(path)?;

    let wasm = ComponentEncoder::default()
        .module(&module)?
        .validate(true)
        .adapter("wasi_snapshot_preview1", &wasi_adapter)?
        .encode()?;

    Component::from_binary(engine, &wasm)
}
