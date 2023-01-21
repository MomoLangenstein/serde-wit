wit_bindgen_guest_rust::generate!({ world: "serde-serializer-provider" });
export_serde_serializer_provider!(GuestsideSerializerProvider);

pub struct GuestsideSerializerProvider {
    _private: (),
}

impl serializer::Serializer for GuestsideSerializerProvider {
    fn test(x: serde_ser::S128) -> serde_ser::U128 {
        serialize::test(x)
    }
}
