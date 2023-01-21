wasmtime_component_macro::bindgen!({ world: "serde-serializer-client" });

pub struct HostsideSerializerProvider<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    serialize: serialize::Serialize,
}

impl<S: wasmtime::AsContextMut> serializer::Serializer for HostsideSerializerProvider<S> {
    fn test(&mut self, x: serde_ser::S128) -> anyhow::Result<serde_ser::U128> {
        self.serialize.test(&mut self.store, x)
    }
}
