#[export_name = "[resource-drop]serializer-resource"]
extern "C" fn resource_dtor(handle: i32) {
    #[link(wasm_import_module = "test:ser/serializer")]
    extern "C" {
        #[link_name = "[dtor]serializer-resource"]
        fn call_drop(handle: i32);
    }

    unsafe { call_drop(handle) }
}

#[export_name = "[resource-new]serializer-resource"]
extern "C" fn resource_new(handle: i32) -> i32 {
    handle
}

#[export_name = "[resource-rep]serializer-resource"]
extern "C" fn resource_rep(handle: i32) -> i32 {
    handle
}
