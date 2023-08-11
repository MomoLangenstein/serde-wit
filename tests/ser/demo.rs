use anyhow::Result;
use wasmtime::Store;

wasmtime::component::bindgen!({
    path: "tests/ser/demo",
});

#[derive(Default)]
pub struct MyImports;

#[test]
fn run() -> Result<()> {
    crate::run_test(
        "demo",
        |_linker| Ok(()), // Demo::add_to_linker(linker, |x| &mut x.0),
        |store, component, linker| Demo::instantiate(store, component, linker),
        run_test,
    )
}

fn run_test(demo: Demo, store: &mut Store<crate::Wasi<MyImports>>) -> Result<()> {
    let result = demo.call_run(&mut *store)?;
    assert_eq!(result, Err(String::from("Hello world!")));
    Ok(())
}
