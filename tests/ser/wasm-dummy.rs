wit_bindgen::generate!({ path: "../../ser/wit", world: "test-dummy", exports: {
    "test:ser/serialize": SerTest,
    "test:ser/serializer": SerTest,
} });

struct SerTest;

impl self::exports::test::ser::serialize::Serialize for SerTest {
    fn serialize() -> String {
        unimplemented!("dummy")
    }
}


impl self::exports::test::ser::serializer::Serializer for SerTest {
    fn serialize_i32(_v: i32) -> String {
        unimplemented!("dummy")
    }
}

fn main() {}
