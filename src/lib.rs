#![cfg_attr(not(feature = "wasmtime-host"), no_std)]

extern crate alloc;

pub(crate) mod any;
pub(crate) mod intern;

// pub mod de;
pub mod ser;

// wit_bindgen::generate!({ world: "serde-demo", exports: {
//     "serde:serde/serialize/serialize": GuestsideSerializerClient,
//     "serde:serde/raw-serialize/serialize": GuestsideSerializerClient,
// } });

// pub struct GuestsideSerializerClient;

// impl self::exports::serde::serde::serialize::Serialize for GuestsideSerializerClient {
//     fn serialize(&self, serializer: self::serde::serde::raw_serializer::Serializer) {
//         todo!()
//     }
// }
