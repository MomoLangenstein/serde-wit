mod serializer {
    wit_bindgen::generate!({ path: "../../ser/wit", world: "test-serializer", exports: {
        "test:ser/serializer": super::SerTest,
        "test:ser/serializer/serializer-resource": super::SerTest,
    } });
}

pub struct SerTest;

impl serializer::exports::test::ser::serializer::SerializerResource for SerTest {
    fn new() -> Self {
        println!("SerializerResource::new called from serialize");
        Self
    }

    fn serialize_i32(_this: serializer::exports::test::ser::serializer::OwnSerializerResource, v: i32) -> String {
        println!("SerializerResource::serialize_i32(v={v}) called from serialize");

        format!("{v}")
    }
}

impl Drop for SerTest {
    fn drop(&mut self) {
        println!("SerializerResource::drop called from serialize");
    }
}

impl serializer::exports::test::ser::serializer::Serializer for SerTest {
    fn serialize_i32(v: i32) -> String {
        println!("serialize_i32(v={v}) called from serialize");

        format!("{v}")
    }
}

// #[export_name = "[resource-drop]serializer-resource"]
// extern "C" fn resource_dtor(handle: i32) {
//     #[link(wasm_import_module = "test:ser/serializer")]
//     extern "C" {
//         #[link_name = "[dtor]serializer-resource"]
//         fn call_drop(handle: i32);
//     }

//     unsafe { call_drop(handle) }
// }

// #[export_name = "[resource-new]serializer-resource"]
// #[link_section = "[export]test:ser/serializer"]
// extern "C" fn resource_new(handle: i32) -> i32 {
//     handle
// }

// #[export_name = "[resource-rep]serializer-resource"]
// #[link_section = "[export]test:ser/serializer"]
// extern "C" fn resource_rep(handle: i32) -> i32 {
//     handle
// }

fn main() {}
