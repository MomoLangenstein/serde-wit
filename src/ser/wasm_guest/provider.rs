use alloc::{string::String, vec::Vec};

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    serde_if_integer128,
};

wit_bindgen_guest_rust::generate!({ world: "serde-serializer-provider" });
// export_serde_serializer_provider!(GuestsideSerializerProvider);

use crate::{any::Any, intern::intern_string};

pub struct GuestsideSerializerProvider<S: serde::Serializer> {
    serializer: S,
}

impl<S: serde::Serializer> serializer::Serializer for GuestsideSerializerProvider<S> {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> (serde_types::U128, serde_types::Usize) {
        serialize::test(x, y)
    }
}

impl<S: serde::Serializer> GuestsideSerializerProvider<S> {
    #[must_use]
    pub fn new(serializer: S) -> Self {
        Self { serializer }
    }

    fn wrap_result(result: Result<S::Ok, S::Error>) -> Result<SerOk, SerError> {
        result.map(SerOk::wrap).map_err(SerError::wrap)
    }

    fn serialize_bool(self, v: bool) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_i8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_i16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_i32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_i64(v))
    }

    serde_if_integer128! {
        fn serialize_i128(self, v: serde_types::S128) -> Result<SerOk, SerError> {
            let le_hi = v.le_hi.to_le_bytes();
            let le_lo = v.le_lo.to_le_bytes();

            let bytes = [
                le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
                le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
            ];

            Self::wrap_result(self.serializer.serialize_i128(i128::from_le_bytes(bytes)))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_u8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_u16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_u32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_u64(v))
    }

    serde_if_integer128! {
        fn serialize_u128(self, v: serde_types::U128) -> Result<SerOk, SerError> {
            let le_hi = v.le_hi.to_le_bytes();
            let le_lo = v.le_lo.to_le_bytes();

            let bytes = [
                le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
                le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
            ];

            Self::wrap_result(self.serializer.serialize_u128(u128::from_le_bytes(bytes)))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_f32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_f64(v))
    }

    fn serialize_char(self, v: char) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_char(v))
    }

    fn serialize_str(self, v: String) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_str(&v))
    }

    fn serialize_bytes(self, v: Vec<u8>) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_bytes(&v))
    }

    fn serialize_none(self) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_none())
    }

    fn serialize_some(self, value: &Serialize) -> Result<SerOk, SerError> {
        Self::wrap_result(
            self.serializer
                .serialize_some(&SerializableSerialize::new(value)),
        )
    }

    fn serialize_unit(self) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_unit())
    }

    fn serialize_unit_struct(self, name: String) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_unit_struct(intern_string(name)))
    }

    fn serialize_unit_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
    ) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_unit_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
        ))
    }

    fn serialize_newtype_struct(self, name: String, value: &Serialize) -> Result<SerOk, SerError> {
        Self::wrap_result(
            self.serializer
                .serialize_newtype_struct(intern_string(name), &SerializableSerialize::new(value)),
        )
    }

    fn serialize_newtype_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        value: &Serialize,
    ) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_newtype_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
            &SerializableSerialize::new(value),
        ))
    }

    fn serialize_seq(
        self,
        len: Option<serde_types::Usize>,
    ) -> Result<GuestsideSerializeSeqProvider<S>, SerError> {
        Ok(GuestsideSerializeSeqProvider {
            serialize_seq: self
                .serializer
                .serialize_seq(len.map(|len| len.val as usize))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_tuple(
        self,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeTupleProvider<S>, SerError> {
        Ok(GuestsideSerializeTupleProvider {
            serialize_tuple: self
                .serializer
                .serialize_tuple(len.val as usize)
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeTupleStructProvider<S>, SerError> {
        Ok(GuestsideSerializeTupleStructProvider {
            serialize_tuple_struct: self
                .serializer
                .serialize_tuple_struct(intern_string(name), len.val as usize)
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeTupleVariantProvider<S>, SerError> {
        Ok(GuestsideSerializeTupleVariantProvider {
            serialize_tuple_variant: self
                .serializer
                .serialize_tuple_variant(
                    intern_string(name),
                    variant_index,
                    intern_string(variant),
                    len.val as usize,
                )
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_map(
        self,
        len: Option<serde_types::Usize>,
    ) -> Result<GuestsideSerializeMapProvider<S>, SerError> {
        Ok(GuestsideSerializeMapProvider {
            serialize_map: self
                .serializer
                .serialize_map(len.map(|len| len.val as usize))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_struct(
        self,
        name: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeStructProvider<S>, SerError> {
        Ok(GuestsideSerializeStructProvider {
            serialize_struct: self
                .serializer
                .serialize_struct(intern_string(name), len.val as usize)
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_struct_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeStructVariantProvider<S>, SerError> {
        Ok(GuestsideSerializeStructVariantProvider {
            serialize_struct_variant: self
                .serializer
                .serialize_struct_variant(
                    intern_string(name),
                    variant_index,
                    intern_string(variant),
                    len.val as usize,
                )
                .map_err(SerError::wrap)?,
        })
    }

    fn is_human_readable(&self) -> bool {
        self.serializer.is_human_readable()
    }
}

struct SerOk {
    value: Any,
}

impl SerOk {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: unsafe { Any::new(value) },
        }
    }

    fn take<T>(self) -> Option<T> {
        // Safety: TODO
        unsafe { self.value.take() }
    }
}

struct SerError {
    inner: SerErrorOrCustom,
}

enum SerErrorOrCustom {
    Error(Any),
    Custom(String),
}

impl SerError {
    fn wrap<T: serde::ser::Error>(value: T) -> Self {
        // Safety: TODO
        Self {
            inner: SerErrorOrCustom::Error(unsafe { Any::new(value) }),
        }
    }

    fn take<T: serde::ser::Error>(self) -> Option<T> {
        match self.inner {
            // Safety: TODO
            SerErrorOrCustom::Error(err) => unsafe { err.take() },
            SerErrorOrCustom::Custom(msg) => Some(serde::ser::Error::custom(msg)),
        }
    }

    fn custom(msg: String) -> Self {
        Self {
            inner: SerErrorOrCustom::Custom(msg),
        }
    }
}

struct GuestsideSerializeSeqProvider<S: serde::Serializer> {
    serialize_seq: S::SerializeSeq,
}

struct GuestsideSerializeTupleProvider<S: serde::Serializer> {
    serialize_tuple: S::SerializeTuple,
}

struct GuestsideSerializeTupleStructProvider<S: serde::Serializer> {
    serialize_tuple_struct: S::SerializeTupleStruct,
}

struct GuestsideSerializeTupleVariantProvider<S: serde::Serializer> {
    serialize_tuple_variant: S::SerializeTupleVariant,
}

struct GuestsideSerializeMapProvider<S: serde::Serializer> {
    serialize_map: S::SerializeMap,
}

struct GuestsideSerializeStructProvider<S: serde::Serializer> {
    serialize_struct: S::SerializeStruct,
}

struct GuestsideSerializeStructVariantProvider<S: serde::Serializer> {
    serialize_struct_variant: S::SerializeStructVariant,
}

impl<S: serde::Serializer> GuestsideSerializeSeqProvider<S> {
    fn serialize_element(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_seq
            .serialize_element(&SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_seq
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }
}

impl<S: serde::Serializer> GuestsideSerializeTupleProvider<S> {
    fn serialize_element(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple
            .serialize_element(&SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_tuple
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }
}

impl<S: serde::Serializer> GuestsideSerializeTupleStructProvider<S> {
    fn serialize_field(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple_struct
            .serialize_field(&SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_tuple_struct
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }
}

impl<S: serde::Serializer> GuestsideSerializeTupleVariantProvider<S> {
    fn serialize_field(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple_variant
            .serialize_field(&SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_tuple_variant
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }
}

impl<S: serde::Serializer> GuestsideSerializeMapProvider<S> {
    fn serialize_key(mut self, key: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_map
            .serialize_key(&SerializableSerialize::new(key))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn serialize_value(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_map
            .serialize_value(&SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_map
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }
}

impl<S: serde::Serializer> GuestsideSerializeStructProvider<S> {
    fn serialize_field(mut self, key: String, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct
            .serialize_field(intern_string(key), &SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_struct
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn skip_field(mut self, key: String) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct
            .skip_field(intern_string(key))
            .map_err(SerError::wrap);

        (self, result)
    }
}

impl<S: serde::Serializer> GuestsideSerializeStructVariantProvider<S> {
    fn serialize_field(mut self, key: String, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct_variant
            .serialize_field(intern_string(key), &SerializableSerialize::new(value))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_struct_variant
            .end()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn skip_field(mut self, key: String) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct_variant
            .skip_field(intern_string(key))
            .map_err(SerError::wrap);

        (self, result)
    }
}

struct SerializableSerialize<'a> {
    serialize: &'a Serialize,
}

impl<'a> SerializableSerialize<'a> {
    fn new(serialize: &'a Serialize) -> Self {
        Self { serialize }
    }
}

impl<'a> serde::Serialize for SerializableSerialize<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let result = self
            .serialize
            .serialize(GuestsideSerializerProvider::new(serializer));

        match result {
            Ok(value) => {
                if let Some(value) = value.take() {
                    return Ok(value);
                }
            }
            Err(err) => {
                if let Some(err) = err.take() {
                    return Err(err);
                }
            }
        };

        Err(serde::ser::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }
}

struct Serialize {
    _private: (),
}

// TODO: remove
impl Serialize {
    fn serialize<S: serde::Serializer>(
        &self,
        _serializer: GuestsideSerializerProvider<S>,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}
