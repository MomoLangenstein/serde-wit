use std::path::Path;

use anyhow::Result;
use wasmtime::Store;

wasmtime::component::bindgen!({
    path: "tests/ser",
});

#[derive(Default)]
pub struct MyImports;

#[test]
fn run() -> Result<()> {
    crate::run_test(
        Path::new(test_artifacts::WASM_SER),
        |_linker| Ok(()), // Ser::add_to_linker(linker, |x| &mut x.0),
        |store, component, linker| Test::instantiate(store, component, linker),
        run_test,
    )
}

fn run_test(test: Test, store: &mut Store<crate::Wasi<MyImports>>) -> Result<()> {
    let result = test.call_run(&mut *store)?;
    assert_eq!(result, Err(String::from("Hello world!")));
    Ok(())
}
