#![allow(clippy::cast_possible_truncation)] // FIXME: make safe wrapper for borrowed handles

use alloc::format;
use core::fmt;

use ::serde::serde_if_integer128;
use scoped_reference::{ScopedBorrow, ScopedReference};

wit_bindgen::generate!({ world: "serde-serializer-client", exports: {
    "serde:serde/serde-serialize/serialize": GuestsideSerializerClient,
}, std_feature });

pub struct GuestsideSerializerClient {
    serialize: ScopedBorrow<dyn ErasedSerialize>,
}

impl self::exports::serde::serde::serde_serialize::Serialize for GuestsideSerializerClient {
    fn serialize(
        &self,
        serializer: self::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
    ) -> Result<
        self::exports::serde::serde::serde_serialize::OwnedSerOkHandle,
        self::exports::serde::serde::serde_serialize::OwnedSerErrorHandle,
    > {
        let result = ErasedSerialize::erased_serialize(
            &*self.serialize,
            SerializerableSerializer::new(serializer),
        );
        match result {
            Ok(ok) => Ok(
                self::exports::serde::serde::serde_serialize::OwnedSerOkHandle {
                    owned_handle: ok.ok.into_handle(),
                },
            ),
            Err(err) => Err(
                self::exports::serde::serde::serde_serialize::OwnedSerErrorHandle {
                    owned_handle: err.error.into_handle(),
                },
            ),
        }
    }
}

impl GuestsideSerializerClient {
    pub fn with_new<'a, S: ?Sized + ::serde::Serialize, F: FnOnce(&Self) -> Q, Q>(
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
}

trait ErasedSerialize {
    fn erased_serialize(&self, serializer: SerializerableSerializer) -> Result<SerOk, SerError>;
}

impl<T: ?Sized + ::serde::Serialize> ErasedSerialize for T {
    fn erased_serialize(&self, serializer: SerializerableSerializer) -> Result<SerOk, SerError> {
        self.serialize(serializer)
    }
}

struct SerializerableSerializer {
    serializer: self::serde::serde::serde_serializer::Serializer,
}

impl SerializerableSerializer {
    fn new(
        serializer: self::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
    ) -> Self {
        // Safety: both serializers should be the same on the other side, from owned to owned
        let serializer = unsafe {
            self::serde::serde::serde_serializer::Serializer::from_handle(
                serializer.owned_handle,
                true,
            )
        };

        Self { serializer }
    }
}

trait WrapSerResult {
    type Ok;

    fn wrap(self) -> Result<Self::Ok, SerError>;
}

impl WrapSerResult
    for Result<
        self::serde::serde::serde_serializer::SerOk,
        self::serde::serde::serde_serializer::SerError,
    >
{
    type Ok = SerOk;

    fn wrap(self) -> Result<SerOk, SerError> {
        match self {
            Ok(ok) => Ok(SerOk { ok }),
            Err(error) => Err(SerError { error }),
        }
    }
}

impl WrapSerResult for Result<(), self::serde::serde::serde_serializer::SerError> {
    type Ok = ();

    fn wrap(self) -> Result<(), SerError> {
        self.map_err(|error| SerError { error })
    }
}

impl ::serde::Serializer for SerializerableSerializer {
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
        self::serde::serde::serde_serializer::Serializer::serialize_bool(self.serializer, v).wrap()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_i8(self.serializer, v).wrap()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_i16(self.serializer, v).wrap()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_i32(self.serializer, v).wrap()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_i64(self.serializer, v).wrap()
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

            self::serde::serde::serde_serializer::Serializer::serialize_i128(self.serializer, self::serde::serde::serde_types::S128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            }).wrap()
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_u8(self.serializer, v).wrap()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_u16(self.serializer, v).wrap()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_u32(self.serializer, v).wrap()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_u64(self.serializer, v).wrap()
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

            self::serde::serde::serde_serializer::Serializer::serialize_u128(self.serializer, self::serde::serde::serde_types::U128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            }).wrap()
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_f32(self.serializer, v).wrap()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_f64(self.serializer, v).wrap()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_char(self.serializer, v).wrap()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_str(self.serializer, v).wrap()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_bytes(self.serializer, v).wrap()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_none(self.serializer).wrap()
    }

    fn serialize_some<V: ?Sized + ::serde::Serialize>(
        self,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::Serializer::serialize_some(self.serializer, value)
        })
        .wrap()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_unit(self.serializer).wrap()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_unit_struct(
            self.serializer,
            name,
        )
        .wrap()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self::serde::serde::serde_serializer::Serializer::serialize_unit_variant(
            self.serializer,
            name,
            variant_index,
            variant,
        )
        .wrap()
    }

    fn serialize_newtype_struct<V: ?Sized + ::serde::Serialize>(
        self,
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::Serializer::serialize_newtype_struct(
                self.serializer,
                name,
                value,
            )
        })
        .wrap()
    }

    fn serialize_newtype_variant<V: ?Sized + ::serde::Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::Serializer::serialize_newtype_variant(
                self.serializer,
                name,
                variant_index,
                variant,
                value,
            )
        })
        .wrap()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = match len {
            Some(len) => match u32::try_from(len) {
                Ok(len) => Some(self::serde::serde::serde_types::Usize { val: len }),
                Err(_) => {
                    return Err(::serde::ser::Error::custom(
                        "Serializer::serialize_seq len exceeds u32",
                    ))
                }
            },
            None => None,
        };

        match self::serde::serde::serde_serializer::Serializer::serialize_seq(self.serializer, len)
        {
            Ok(serialize_seq) => Ok(SerializerableSerializeSeq {
                serialize_seq: Some(serialize_seq),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(::serde::ser::Error::custom(
                "Serializer::serialize_tuple len exceeds u32",
            ));
        };

        let len = self::serde::serde::serde_types::Usize { val: len };

        match self::serde::serde::serde_serializer::Serializer::serialize_tuple(
            self.serializer,
            len,
        ) {
            Ok(serialize_tuple) => Ok(SerializerableSerializeTuple {
                serialize_tuple: Some(serialize_tuple),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(::serde::ser::Error::custom(
                "Serializer::serialize_tuple_struct len exceeds u32",
            ));
        };

        let len = self::serde::serde::serde_types::Usize { val: len };

        match self::serde::serde::serde_serializer::Serializer::serialize_tuple_struct(
            self.serializer,
            name,
            len,
        ) {
            Ok(serialize_tuple_struct) => Ok(SerializerableSerializeTupleStruct {
                serialize_tuple_struct: Some(serialize_tuple_struct),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(::serde::ser::Error::custom(
                "Serializer::serialize_tuple_variant len exceeds u32",
            ));
        };

        let len = self::serde::serde::serde_types::Usize { val: len };

        match self::serde::serde::serde_serializer::Serializer::serialize_tuple_variant(
            self.serializer,
            name,
            variant_index,
            variant,
            len,
        ) {
            Ok(serialize_tuple_variant) => Ok(SerializerableSerializeTupleVariant {
                serialize_tuple_variant: Some(serialize_tuple_variant),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = match len {
            Some(len) => match u32::try_from(len) {
                Ok(len) => Some(self::serde::serde::serde_types::Usize { val: len }),
                Err(_) => {
                    return Err(::serde::ser::Error::custom(
                        "Serializer::serialize_map len exceeds u32",
                    ))
                }
            },
            None => None,
        };

        match self::serde::serde::serde_serializer::Serializer::serialize_map(self.serializer, len)
        {
            Ok(serialize_map) => Ok(SerializerableSerializeMap {
                serialize_map: Some(serialize_map),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(::serde::ser::Error::custom(
                "Serializer::serialize_struct len exceeds u32",
            ));
        };

        let len = self::serde::serde::serde_types::Usize { val: len };

        match self::serde::serde::serde_serializer::Serializer::serialize_struct(
            self.serializer,
            name,
            len,
        ) {
            Ok(serialize_struct) => Ok(SerializerableSerializeStruct {
                serialize_struct: Some(serialize_struct),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(::serde::ser::Error::custom(
                "Serializer::serialize_struct_variant len exceeds u32",
            ));
        };

        let len = self::serde::serde::serde_types::Usize { val: len };

        match self::serde::serde::serde_serializer::Serializer::serialize_struct_variant(
            self.serializer,
            name,
            variant_index,
            variant,
            len,
        ) {
            Ok(serialize_struct_variant) => Ok(SerializerableSerializeStructVariant {
                serialize_struct_variant: Some(serialize_struct_variant),
            }),
            Err(error) => Err(SerError { error }),
        }
    }

    fn is_human_readable(&self) -> bool {
        self::serde::serde::serde_serializer::Serializer::is_human_readable(&self.serializer)
    }
}

struct SerializerableSerializeSeq {
    serialize_seq: Option<self::serde::serde::serde_serializer::SerializeSeq>,
}

impl ::serde::ser::SerializeSeq for SerializerableSerializeSeq {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_element<V: ?Sized + ::serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_seq) = self.serialize_seq.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeSeq::serialize_element after free"));
        };

        let (serialize_seq, result) = GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeSeq::serialize_element(
                serialize_seq,
                value,
            )
        });
        self.serialize_seq = Some(serialize_seq);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_seq) = self.serialize_seq.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeSeq::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeSeq::end(serialize_seq).wrap()
    }
}

struct SerializerableSerializeTuple {
    serialize_tuple: Option<self::serde::serde::serde_serializer::SerializeTuple>,
}

impl ::serde::ser::SerializeTuple for SerializerableSerializeTuple {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_element<V: ?Sized + ::serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple) = self.serialize_tuple.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeTuple::serialize_element after free"));
        };

        let (serialize_tuple, result) = GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeTuple::serialize_element(
                serialize_tuple,
                value,
            )
        });
        self.serialize_tuple = Some(serialize_tuple);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple) = self.serialize_tuple.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeTuple::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeTuple::end(serialize_tuple).wrap()
    }
}

struct SerializerableSerializeTupleStruct {
    serialize_tuple_struct: Option<self::serde::serde::serde_serializer::SerializeTupleStruct>,
}

impl ::serde::ser::SerializeTupleStruct for SerializerableSerializeTupleStruct {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + ::serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple_struct) = self.serialize_tuple_struct.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeTupleStruct::serialize_field after free"));
        };

        let (serialize_tuple_struct, result) =
            GuestsideSerializerClient::with_new(value, |value| {
                let borrowed_handle =
                    unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) }
                        as i32;
                let value =
                    serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
                self::serde::serde::serde_serializer::SerializeTupleStruct::serialize_field(
                    serialize_tuple_struct,
                    value,
                )
            });
        self.serialize_tuple_struct = Some(serialize_tuple_struct);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple_struct) = self.serialize_tuple_struct.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeTupleStruct::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeTupleStruct::end(serialize_tuple_struct)
            .wrap()
    }
}

struct SerializerableSerializeTupleVariant {
    serialize_tuple_variant: Option<self::serde::serde::serde_serializer::SerializeTupleVariant>,
}

impl ::serde::ser::SerializeTupleVariant for SerializerableSerializeTupleVariant {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + ::serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple_variant) = self.serialize_tuple_variant.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeTupleVariant::serialize_field after free"));
        };

        let (serialize_tuple_variant, result) =
            GuestsideSerializerClient::with_new(value, |value| {
                let borrowed_handle =
                    unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) }
                        as i32;
                let value =
                    serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
                self::serde::serde::serde_serializer::SerializeTupleVariant::serialize_field(
                    serialize_tuple_variant,
                    value,
                )
            });
        self.serialize_tuple_variant = Some(serialize_tuple_variant);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple_variant) = self.serialize_tuple_variant.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeTupleVariant::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeTupleVariant::end(serialize_tuple_variant)
            .wrap()
    }
}

struct SerializerableSerializeMap {
    serialize_map: Option<self::serde::serde::serde_serializer::SerializeMap>,
}

impl ::serde::ser::SerializeMap for SerializerableSerializeMap {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_key<K: ?Sized + ::serde::Serialize>(
        &mut self,
        key: &K,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeMap::serialize_key after free"));
        };

        let (serialize_map, result) = GuestsideSerializerClient::with_new(key, |key| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(key) } as i32;
            let key = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeMap::serialize_key(serialize_map, key)
        });
        self.serialize_map = Some(serialize_map);

        result.wrap()
    }

    fn serialize_value<V: ?Sized + ::serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeMap::serialize_value after free"));
        };

        let (serialize_map, result) = GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeMap::serialize_value(
                serialize_map,
                value,
            )
        });
        self.serialize_map = Some(serialize_map);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeMap::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeMap::end(serialize_map).wrap()
    }

    fn serialize_entry<K: ?Sized + ::serde::Serialize, V: ?Sized + ::serde::Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeMap::serialize_map after free"));
        };

        let (serialize_map, result) = GuestsideSerializerClient::with_new(key, |key| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(key) } as i32;
            let key = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeMap::serialize_key(serialize_map, key)
        });

        if let Err(err) = result.wrap() {
            self.serialize_map = Some(serialize_map);
            return Err(err);
        }

        let (serialize_map, result) = GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeMap::serialize_value(
                serialize_map,
                value,
            )
        });
        self.serialize_map = Some(serialize_map);

        result.wrap()
    }
}

struct SerializerableSerializeStruct {
    serialize_struct: Option<self::serde::serde::serde_serializer::SerializeStruct>,
}

impl ::serde::ser::SerializeStruct for SerializerableSerializeStruct {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + ::serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeStruct::serialize_field after free"));
        };

        let (serialize_struct, result) = GuestsideSerializerClient::with_new(value, |value| {
            let borrowed_handle =
                unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) } as i32;
            let value = serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
            self::serde::serde::serde_serializer::SerializeStruct::serialize_field(
                serialize_struct,
                key,
                value,
            )
        });
        self.serialize_struct = Some(serialize_struct);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeStruct::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeStruct::end(serialize_struct).wrap()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeStruct::skip_field after free"));
        };

        let (serialize_struct, result) =
            self::serde::serde::serde_serializer::SerializeStruct::skip_field(
                serialize_struct,
                key,
            );
        self.serialize_struct = Some(serialize_struct);

        result.wrap()
    }
}

struct SerializerableSerializeStructVariant {
    serialize_struct_variant: Option<self::serde::serde::serde_serializer::SerializeStructVariant>,
}

impl ::serde::ser::SerializeStructVariant for SerializerableSerializeStructVariant {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + ::serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeStructVariant::serialize_field after free"));
        };

        let (serialize_struct_variant, result) =
            GuestsideSerializerClient::with_new(value, |value| {
                let borrowed_handle =
                    unsafe { core::mem::transmute::<&GuestsideSerializerClient, isize>(value) }
                        as i32;
                let value =
                    serde::serde::serde_serializer::BorrowedSerializeHandle { borrowed_handle };
                self::serde::serde::serde_serializer::SerializeStructVariant::serialize_field(
                    serialize_struct_variant,
                    key,
                    value,
                )
            });
        self.serialize_struct_variant = Some(serialize_struct_variant);

        result.wrap()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeStructVariant::end after free"));
        };

        self::serde::serde::serde_serializer::SerializeStructVariant::end(serialize_struct_variant)
            .wrap()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(::serde::ser::Error::custom("bug: SerializeStructVariant::skip_field after free"));
        };

        let (serialize_struct_variant, result) =
            self::serde::serde::serde_serializer::SerializeStructVariant::skip_field(
                serialize_struct_variant,
                key,
            );
        self.serialize_struct_variant = Some(serialize_struct_variant);

        result.wrap()
    }
}

struct SerOk {
    ok: self::serde::serde::serde_serializer::SerOk,
}

struct SerError {
    error: self::serde::serde::serde_serializer::SerError,
}

impl ::serde::ser::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            error: self::serde::serde::serde_serializer::SerError::custom(&format!("{msg}")),
        }
    }
}

impl ::serde::ser::StdError for SerError {}

impl fmt::Debug for SerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.error.debug())
    }
}

impl fmt::Display for SerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.error.display())
    }
}
