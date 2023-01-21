wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-client" });
export_serde_deserializer_client!(GuestsideDeserializerClient);

pub struct GuestsideDeserializerClient {
    _private: (),
}

impl deserialize::Deserialize for GuestsideDeserializerClient {
    fn test(x: serde_de::S128) -> serde_de::U128 {
        deserializer::test(x)
    }
}
