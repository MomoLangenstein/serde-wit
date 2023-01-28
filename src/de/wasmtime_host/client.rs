wasmtime_component_macro::bindgen!({ world: "serde-deserializer-provider" });

pub struct HostsideDeserializerClient<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    deserializer: deserializer::Deserializer,
}

impl<S: wasmtime::AsContextMut> deserialize::Deserialize for HostsideDeserializerClient<S> {
    fn test(
        &mut self,
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> anyhow::Result<(serde_types::U128, serde_types::Usize)> {
        self.deserializer.test(&mut self.store, x, y)
    }
}
