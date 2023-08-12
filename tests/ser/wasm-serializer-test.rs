mod test {
    wit_bindgen::generate!({ path: "../../ser/wit", world: "test", exports: {
        world: super::SerTest,
        "test:ser/test": super::SerTest,
    } });
}

mod serializer {
    wit_bindgen::generate!({ path: "../../ser/wit", world: "test-serializer", exports: {
        "test:ser/serializer": super::SerTest,
    } });
}

struct SerTest;

impl test::Test for SerTest {
    fn run() -> Result<String, String> {
        Ok(self::serializer::test::ser::serialize::serialize())
    }
}

impl self::serializer::exports::test::ser::serializer::Serializer for SerTest {
    fn serialize_i32(v: i32) -> String {
        format!("{v}")
    }
}

fn main() {}
