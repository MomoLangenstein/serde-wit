use std::path::Path;

use anyhow::Result;
use wasmtime::Store;

wasmtime::component::bindgen!({
    path: "tests/ser/wit", world: "test",
});

#[derive(Default)]
pub struct MyImports;

#[test]
fn run() -> Result<()> {
    crate::run_test(
        Path::new(test_artifacts::WASM_SER_SERIALIZE_TEST),
        &[Path::new(test_artifacts::WASM_SER_SERIALIZER)],
        |_linker| Ok(()), // Test::add_to_linker(linker, |x| &mut x.0),
        |store, component, linker| Test::instantiate(store, component, linker),
        run_test,
    )
}

fn run_test(test: Test, store: &mut Store<crate::Wasi<MyImports>>) -> Result<()> {
    let result = test.call_run(&mut *store)?;
    assert_eq!(result, Ok(String::from("42")));
    Ok(())
}
