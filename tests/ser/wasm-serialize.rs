wit_bindgen::generate!({ path: "../../ser/wit", world: "test-serialize", exports: {
    "test:ser/serialize": SerTest,
} });

struct SerTest;

impl self::exports::test::ser::serialize::Serialize for SerTest {
    fn serialize() -> String {
        self::test::ser::serializer::serialize_i32(42)
    }
}

fn main() {}
