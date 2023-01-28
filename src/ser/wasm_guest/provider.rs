wit_bindgen_guest_rust::generate!({ world: "serde-serializer-provider" });
export_serde_serializer_provider!(GuestsideSerializerProvider);

pub struct GuestsideSerializerProvider {
    _private: (),
}

impl serializer::Serializer for GuestsideSerializerProvider {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> (serde_types::U128, serde_types::Usize) {
        serialize::test(x, y)
    }
}
