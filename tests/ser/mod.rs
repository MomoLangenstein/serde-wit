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
        "ser",
        |_linker| Ok(()), // Ser::add_to_linker(linker, |x| &mut x.0),
        |store, component, linker| Ser::instantiate(store, component, linker),
        run_test,
    )
}

fn run_test(ser: Ser, store: &mut Store<crate::Wasi<MyImports>>) -> Result<()> {
    let result = ser.call_run(&mut *store)?;
    assert_eq!(result, Err(String::from("Hello world!")));
    Ok(())
}
