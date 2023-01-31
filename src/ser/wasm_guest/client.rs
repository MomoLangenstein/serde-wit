use core::fmt;

use scoped_reference::{ScopedBorrow, ScopedReference};
use serde::serde_if_integer128;

wit_bindgen_guest_rust::generate!({ world: "serde-serializer-client", no_std });
export_serde_serializer_client!(GuestsideSerializerClient);

pub struct GuestsideSerializerClient {
    serialize: ScopedBorrow<dyn ErasedSerialize>,
}

impl serialize::Serialize for GuestsideSerializerClient {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> (serde_types::U128, serde_types::Usize) {
        serializer::test(x, y)
    }
}

impl GuestsideSerializerClient {
    pub fn with_new<'a, S: ?Sized + serde::Serialize, F: FnOnce(&Self) -> Q, Q>(
        serialize: &'a S,
        inner: F,
    ) -> Q {
        let scoped_serialize: ScopedReference<dyn ErasedSerialize + 'a> =
            ScopedReference::new(&serialize);

        let result = {
            let serialize: ScopedBorrow<dyn ErasedSerialize + 'a> = scoped_serialize.borrow();
            let serialize: ScopedBorrow<dyn ErasedSerialize + 'static> =
                unsafe { core::mem::transmute(serialize) };

            inner(&Self { serialize })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scoped_serialize);

        result
    }

    fn serialize(&self, serializer: Serializer) -> Result<SerOk, SerError> {
        ErasedSerialize::erased_serialize(
            &*self.serialize,
            SerializerableSerializer::new(serializer),
        )
    }
}

trait ErasedSerialize {
    fn erased_serialize(&self, serializer: SerializerableSerializer) -> Result<SerOk, SerError>;
}

impl<T: ?Sized + serde::Serialize> ErasedSerialize for T {
    fn erased_serialize(&self, serializer: SerializerableSerializer) -> Result<SerOk, SerError> {
        self.serialize(serializer)
    }
}

struct SerializerableSerializer {
    serializer: Serializer,
}

impl SerializerableSerializer {
    fn new(serializer: Serializer) -> Self {
        Self { serializer }
    }
}

impl serde::Serializer for SerializerableSerializer {
    type Ok = SerOk;
    type Error = SerError;

    type SerializeSeq = SerializerableSerializeSeq;
    type SerializeTuple = SerializerableSerializeTuple;
    type SerializeTupleStruct = SerializerableSerializeTupleStruct;
    type SerializeTupleVariant = SerializerableSerializeTupleVariant;
    type SerializeMap = SerializerableSerializeMap;
    type SerializeStruct = SerializerableSerializeStruct;
    type SerializeStructVariant = SerializerableSerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i64(v)
    }

    serde_if_integer128! {
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            let bytes = v.to_le_bytes();

            let le_hi = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let le_lo = [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ];

            self.serializer.serialize_i128(serde_types::S128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            })
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u64(v)
    }

    serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            let bytes = v.to_le_bytes();

            let le_hi = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let le_lo = [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ];

            self.serializer.serialize_u128(serde_types::U128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            })
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_none()
    }

    fn serialize_some<V: ?Sized + serde::Serialize>(
        self,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        GuestsideSerializerClient::with_new(value, |value| self.serializer.serialize_some(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<V: ?Sized + serde::Serialize>(
        self,
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        GuestsideSerializerClient::with_new(value, |value| {
            self.serializer.serialize_newtype_struct(name, value)
        })
    }

    fn serialize_newtype_variant<V: ?Sized + serde::Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        GuestsideSerializerClient::with_new(value, |value| {
            self.serializer
                .serialize_newtype_variant(name, variant_index, variant, value)
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = match len {
            Some(len) => match u32::try_from(len) {
                Ok(len) => Some(serde_types::Usize { val: len }),
                Err(_) => {
                    return Err(serde::ser::Error::custom(
                        "Serializer::serialize_seq len exceeds u32",
                    ))
                }
            },
            None => None,
        };

        self.serializer
            .serialize_seq(len)
            .map(|serialize_seq| SerializerableSerializeSeq {
                serialize_seq: Some(serialize_seq),
            })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_tuple len exceeds u32",
            ));
        };

        self.serializer
            .serialize_tuple(serde_types::Usize { val: len })
            .map(|serialize_tuple| SerializerableSerializeTuple {
                serialize_tuple: Some(serialize_tuple),
            })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_tuple_struct len exceeds u32",
            ));
        };

        self.serializer
            .serialize_tuple_struct(name, serde_types::Usize { val: len })
            .map(
                |serialize_tuple_struct| SerializerableSerializeTupleStruct {
                    serialize_tuple_struct: Some(serialize_tuple_struct),
                },
            )
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_tuple_variant len exceeds u32",
            ));
        };

        self.serializer
            .serialize_tuple_variant(
                name,
                variant_index,
                variant,
                serde_types::Usize { val: len },
            )
            .map(
                |serialize_tuple_variant| SerializerableSerializeTupleVariant {
                    serialize_tuple_variant: Some(serialize_tuple_variant),
                },
            )
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = match len {
            Some(len) => match u32::try_from(len) {
                Ok(len) => Some(serde_types::Usize { val: len }),
                Err(_) => {
                    return Err(serde::ser::Error::custom(
                        "Serializer::serialize_map len exceeds u32",
                    ))
                }
            },
            None => None,
        };

        self.serializer
            .serialize_map(len)
            .map(|serialize_map| SerializerableSerializeMap {
                serialize_map: Some(serialize_map),
            })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_struct len exceeds u32",
            ));
        };

        self.serializer
            .serialize_struct(name, serde_types::Usize { val: len })
            .map(|serialize_struct| SerializerableSerializeStruct {
                serialize_struct: Some(serialize_struct),
            })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_struct_variant len exceeds u32",
            ));
        };

        self.serializer
            .serialize_struct_variant(
                name,
                variant_index,
                variant,
                serde_types::Usize { val: len },
            )
            .map(
                |serialize_struct_variant| SerializerableSerializeStructVariant {
                    serialize_struct_variant: Some(serialize_struct_variant),
                },
            )
    }

    fn is_human_readable(&self) -> bool {
        self.serializer.is_human_readable()
    }
}

struct SerializerableSerializeSeq {
    serialize_seq: Option<SerializeSeq>,
}

impl serde::ser::SerializeSeq for SerializerableSerializeSeq {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_element<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_seq) = self.serialize_seq.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeSeq::serialize_element after free"));
        };

        let (serialize_seq, result) = GuestsideSerializerClient::with_new(value, |value| {
            serialize_seq.serialize_element(value)
        });
        self.serialize_seq = Some(serialize_seq);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_seq) = self.serialize_seq.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeSeq::end after free"));
        };

        serialize_seq.end()
    }
}

struct SerializerableSerializeTuple {
    serialize_tuple: Option<SerializeTuple>,
}

impl serde::ser::SerializeTuple for SerializerableSerializeTuple {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_element<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple) = self.serialize_tuple.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTuple::serialize_element after free"));
        };

        let (serialize_tuple, result) = GuestsideSerializerClient::with_new(value, |value| {
            serialize_tuple.serialize_element(value)
        });
        self.serialize_tuple = Some(serialize_tuple);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple) = self.serialize_tuple.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTuple::end after free"));
        };

        serialize_tuple.end()
    }
}

struct SerializerableSerializeTupleStruct {
    serialize_tuple_struct: Option<SerializeTupleStruct>,
}

impl serde::ser::SerializeTupleStruct for SerializerableSerializeTupleStruct {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple_struct) = self.serialize_tuple_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleStruct::serialize_field after free"));
        };

        let (serialize_tuple_struct, result) =
            GuestsideSerializerClient::with_new(value, |value| {
                serialize_tuple_struct.serialize_field(value)
            });
        self.serialize_tuple_struct = Some(serialize_tuple_struct);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple_struct) = self.serialize_tuple_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleStruct::end after free"));
        };

        serialize_tuple_struct.end()
    }
}

struct SerializerableSerializeTupleVariant {
    serialize_tuple_variant: Option<SerializeTupleVariant>,
}

impl serde::ser::SerializeTupleVariant for SerializerableSerializeTupleVariant {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple_variant) = self.serialize_tuple_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleVariant::serialize_field after free"));
        };

        let (serialize_tuple_variant, result) =
            GuestsideSerializerClient::with_new(value, |value| {
                serialize_tuple_variant.serialize_field(value)
            });
        self.serialize_tuple_variant = Some(serialize_tuple_variant);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple_variant) = self.serialize_tuple_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleVariant::end after free"));
        };

        serialize_tuple_variant.end()
    }
}

struct SerializerableSerializeMap {
    serialize_map: Option<SerializeMap>,
}

impl serde::ser::SerializeMap for SerializerableSerializeMap {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_key<K: ?Sized + serde::Serialize>(&mut self, key: &K) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::serialize_key after free"));
        };

        let (serialize_map, result) =
            GuestsideSerializerClient::with_new(key, |key| serialize_map.serialize_key(key));
        self.serialize_map = Some(serialize_map);

        result
    }

    fn serialize_value<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::serialize_value after free"));
        };

        let (serialize_map, result) = GuestsideSerializerClient::with_new(value, |value| {
            serialize_map.serialize_value(value)
        });
        self.serialize_map = Some(serialize_map);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::end after free"));
        };

        serialize_map.end()
    }

    fn serialize_entry<K: ?Sized + serde::Serialize, V: ?Sized + serde::Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::serialize_map after free"));
        };

        let (serialize_map, result) =
            GuestsideSerializerClient::with_new(key, |key| serialize_map.serialize_key(key));

        if let Err(err) = result {
            self.serialize_map = Some(serialize_map);
            return Err(err);
        }

        let (serialize_map, result) = GuestsideSerializerClient::with_new(value, |value| {
            serialize_map.serialize_value(value)
        });
        self.serialize_map = Some(serialize_map);

        result
    }
}

struct SerializerableSerializeStruct {
    serialize_struct: Option<SerializeStruct>,
}

impl serde::ser::SerializeStruct for SerializerableSerializeStruct {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStruct::serialize_field after free"));
        };

        let (serialize_struct, result) = GuestsideSerializerClient::with_new(value, |value| {
            serialize_struct.serialize_field(key, value)
        });
        self.serialize_struct = Some(serialize_struct);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStruct::end after free"));
        };

        serialize_struct.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStruct::skip_field after free"));
        };

        let (serialize_struct, result) = serialize_struct.skip_field(key);
        self.serialize_struct = Some(serialize_struct);

        result
    }
}

struct SerializerableSerializeStructVariant {
    serialize_struct_variant: Option<SerializeStructVariant>,
}

impl serde::ser::SerializeStructVariant for SerializerableSerializeStructVariant {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStructVariant::serialize_field after free"));
        };

        let (serialize_struct_variant, result) =
            GuestsideSerializerClient::with_new(value, |value| {
                serialize_struct_variant.serialize_field(key, value)
            });
        self.serialize_struct_variant = Some(serialize_struct_variant);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStructVariant::end after free"));
        };

        serialize_struct_variant.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStructVariant::skip_field after free"));
        };

        let (serialize_struct_variant, result) = serialize_struct_variant.skip_field(key);
        self.serialize_struct_variant = Some(serialize_struct_variant);

        result
    }
}

// serializer::SerOk
struct SerOk {
    _private: (),
}

// serializer::SerError
struct SerError {
    _private: (),
}

impl serde::ser::Error for SerError {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        todo!("wit-bindgen")
    }
}

impl serde::ser::StdError for SerError {}

impl fmt::Debug for SerError {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        todo!() // fmt.write_str("SerError")
    }
}

impl fmt::Display for SerError {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        todo!() // fmt.write_str("SerError")
    }
}

struct Serializer {
    _private: (),
}

// TODO: remove
impl Serializer {
    fn serialize_bool(self, _v: bool) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i8(self, _v: i8) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i16(self, _v: i16) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i32(self, _v: i32) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i64(self, _v: i64) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    serde_if_integer128! {
        fn serialize_i128(self, _v: serde_types::S128) -> Result<SerOk, SerError> {
            todo!("wit-bindgen")
        }
    }

    fn serialize_u8(self, _v: u8) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_u16(self, _v: u16) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_u32(self, _v: u32) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_u64(self, _v: u64) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    serde_if_integer128! {
        fn serialize_u128(self, _v: serde_types::U128) -> Result<SerOk, SerError> {
            todo!("wit-bindgen")
        }
    }

    fn serialize_f32(self, _v: f32) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_f64(self, _v: f64) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_char(self, _v: char) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_str(self, _v: &str) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_none(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_some(self, _value: &GuestsideSerializerClient) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_unit(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_unit_struct(self, _name: &str) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_newtype_struct(
        self,
        _name: &str,
        _value: &GuestsideSerializerClient,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_newtype_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _value: &GuestsideSerializerClient,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_seq(self, _len: Option<serde_types::Usize>) -> Result<SerializeSeq, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_tuple(self, _len: serde_types::Usize) -> Result<SerializeTuple, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_tuple_struct(
        self,
        _name: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeTupleStruct, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_tuple_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeTupleVariant, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_map(self, _len: Option<serde_types::Usize>) -> Result<SerializeMap, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_struct(
        self,
        _name: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeStruct, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_struct_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeStructVariant, SerError> {
        todo!("wit-bindgen")
    }

    fn is_human_readable(&self) -> bool {
        todo!("wit-bindgen")
    }
}

struct SerializeSeq {
    _private: (),
}

struct SerializeTuple {
    _private: (),
}

struct SerializeTupleStruct {
    _private: (),
}

struct SerializeTupleVariant {
    _private: (),
}

struct SerializeMap {
    _private: (),
}

struct SerializeStruct {
    _private: (),
}

struct SerializeStructVariant {
    _private: (),
}

impl SerializeSeq {
    fn serialize_element(self, _value: &GuestsideSerializerClient) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeTuple {
    fn serialize_element(self, _value: &GuestsideSerializerClient) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeTupleStruct {
    fn serialize_field(self, _value: &GuestsideSerializerClient) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeTupleVariant {
    fn serialize_field(self, _value: &GuestsideSerializerClient) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeMap {
    fn serialize_key(self, _key: &GuestsideSerializerClient) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn serialize_value(self, _value: &GuestsideSerializerClient) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeStruct {
    fn serialize_field(
        self,
        _key: &str,
        _value: &GuestsideSerializerClient,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn skip_field(self, _key: &str) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }
}

impl SerializeStructVariant {
    fn serialize_field(
        self,
        _key: &str,
        _value: &GuestsideSerializerClient,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn skip_field(self, _key: &str) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }
}
