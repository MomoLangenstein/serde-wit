mod test {
    wit_bindgen::generate!({ path: "../../ser/wit", world: "test", exports: {
        world: super::SerTest,
        "test:ser/test": super::SerTest,
    } });
}

mod serialize {
    wit_bindgen::generate!({ path: "../../ser/wit", world: "test-serialize" });
}

struct SerTest;

impl test::Test for SerTest {
    fn run() -> Result<String, String> {
        println!("run() called from host");

        let serializer = self::serialize::test::ser::serializer::SerializerResource::new();
        // std::mem::forget(serializer);

        // Ok(self::serialize::test::ser::serializer::serialize_i32(42))

        Ok(self::serialize::test::ser::serializer::SerializerResource::serialize_i32(serializer, 42))
    }
}

fn main() {}
