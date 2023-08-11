wit_bindgen::generate!({ path: "../../ser/demo", world: "demo", exports: {
    world: DemoTest,
    "test:demo/test": DemoTest,
} });

struct DemoTest;

impl Demo for DemoTest {
    fn run() -> Result<(), String> {
        Err(String::from("Hello world!"))
    }
}

impl self::exports::test::demo::test::Test for DemoTest {
    fn string_roundtrip(a: String) -> String {
        a
    }
}

fn main() {}
