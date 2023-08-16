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

        let serializer = serialize::test::ser::serializer::SerializerResource::new();
        // std::mem::forget(serializer);

        // Ok(self::serialize::test::ser::serializer::serialize_i32(42))

        let my_serialize = Serialize { value: 2899 };

        println!("{:p}", serialize as *const ());

        let serialize = serialize::test::ser::serializer::Serialize::new(
            unsafe { std::mem::transmute::<&Serialize, isize>(&my_serialize) } as i32,
            serialize::test::ser::serializer::SerializeVtable {
                serialize: unsafe { std::mem::transmute::<fn(i32, i32) -> String, isize>(serialize) }as i32,
            },
        );

        println!("{serialize:?}");

        let result = serialize::test::ser::serializer::SerializerResource::serialize_some(serializer, &serialize);

        std::mem::drop(my_serialize);

        Ok(result)
    }
}

struct Serialize {
    value: i32,
}

impl Serialize {
    fn serialize(&self, serializer: serialize::test::ser::serializer::SerializerResource) -> String {
        serialize::test::ser::serializer::SerializerResource::serialize_i32(serializer, self.value)
    }
}

fn serialize(serialize: i32, serializer: i32) -> String {
    let serialize: &Serialize = unsafe { &*((serialize as isize) as *const Serialize) };
    let serializer = unsafe { serialize::test::ser::serializer::SerializerResource::from_handle(serializer, true) };

    serialize.serialize(serializer)
}

fn main() {}
