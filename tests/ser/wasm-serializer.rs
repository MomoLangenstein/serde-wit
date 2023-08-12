mod serializer {
    wit_bindgen::generate!({ path: "../../ser/wit", world: "test-serializer", exports: {
        "test:ser/serializer": super::SerTest,
    } });
}

struct SerTest;

impl self::serializer::exports::test::ser::serializer::Serializer for SerTest {
    fn serialize_i32(v: i32) -> String {
        println!("serialize_i32(v={v}) called from serialize");

        format!("{v}")
    }
}

fn main() {}
