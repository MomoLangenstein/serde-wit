wasmtime_component_macro::bindgen!({ world: "serde-serializer-provider" });

pub struct HostsideSerializerClient<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    serializer: serializer::Serializer,
}

impl<S: wasmtime::AsContextMut> serialize::Serialize for HostsideSerializerClient<S> {
    fn test(&mut self, x: serde_ser::S128) -> anyhow::Result<serde_ser::U128> {
        self.serializer.test(&mut self.store, x)
    }
}
