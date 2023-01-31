wasmtime_component_macro::bindgen!({ world: "serde-deserializer-client" });

pub struct HostsideDeserializerProvider<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    deserialize: deserialize::Deserialize,
}

impl<S: wasmtime::AsContextMut> deserializer::Deserializer for HostsideDeserializerProvider<S> {
    fn test(
        &mut self,
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> anyhow::Result<Result<(serde_types::U128, serde_types::Usize), serde_de::Unexpected>> {
        self.deserialize.test(&mut self.store, x, y)
    }
}
