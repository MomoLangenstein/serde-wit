wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-client" });
export_serde_deserializer_client!(GuestsideDeserializerClient);

pub struct GuestsideDeserializerClient {
    _private: (),
}

impl deserialize::Deserialize for GuestsideDeserializerClient {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> (serde_types::U128, serde_types::Usize) {
        deserializer::test(x, y)
    }
}
