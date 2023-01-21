wasmtime_component_macro::bindgen!({ world: "serde-deserializer-provider" });

pub struct HostsideDeserializerClient<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    deserializer: deserializer::Deserializer,
}

impl<S: wasmtime::AsContextMut> deserialize::Deserialize for HostsideDeserializerClient<S> {
    fn test(&mut self, x: serde_de::S128) -> anyhow::Result<serde_de::U128> {
        self.deserializer.test(&mut self.store, x)
    }
}
