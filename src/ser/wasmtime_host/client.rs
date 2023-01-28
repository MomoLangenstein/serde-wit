wasmtime_component_macro::bindgen!({ world: "serde-serializer-provider" });

pub struct HostsideSerializerClient<S: wasmtime::AsContextMut> {
    _private: (),
    store: S,
    serializer: serializer::Serializer,
}

impl<S: wasmtime::AsContextMut> serialize::Serialize for HostsideSerializerClient<S> {
    fn test(
        &mut self,
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> anyhow::Result<(serde_types::U128, serde_types::Usize)> {
        self.serializer.test(&mut self.store, x, y)
    }
}
