wasmtime_component_macro::bindgen!({ world: "serde-deserializer-client" });

pub struct HostsideDeserializerProvider<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    deserialize: deserialize::Deserialize,
}

impl<S: wasmtime::AsContextMut> deserializer::Deserializer for HostsideDeserializerProvider<S> {
    fn test(&mut self, x: serde_de::S128) -> anyhow::Result<serde_de::U128> {
        self.deserialize.test(&mut self.store, x)
    }
}
