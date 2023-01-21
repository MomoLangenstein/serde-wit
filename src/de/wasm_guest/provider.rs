wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-provider" });
export_serde_deserializer_provider!(GuestsideDeserializerProvider);

pub struct GuestsideDeserializerProvider {
    _private: (),
}

impl deserializer::Deserializer for GuestsideDeserializerProvider {
    fn test(x: serde_de::S128) -> serde_de::U128 {
        deserialize::test(x)
    }
}
