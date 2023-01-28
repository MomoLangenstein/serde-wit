wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-provider" });
export_serde_deserializer_provider!(GuestsideDeserializerProvider);

pub struct GuestsideDeserializerProvider {
    _private: (),
}

impl deserializer::Deserializer for GuestsideDeserializerProvider {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> (serde_types::U128, serde_types::Usize) {
        deserialize::test(x, y)
    }
}
