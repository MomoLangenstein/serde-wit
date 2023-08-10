use alloc::{string::String, vec::Vec};
use core::cell::RefCell;

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    serde_if_integer128,
};

wasmtime_component_macro::bindgen!({ world: "serde-serializer-client" });

use crate::{any::Any, intern::intern_string};

pub struct HostsideSerializerProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    serializer: S,
    store: T,
}

impl<S: serde::Serializer, T: wasmtime::AsContextMut> serializer::Serializer
    for HostsideSerializerProvider<S, T>
{
    fn test(
        &mut self,
        _x: serde_types::S128,
        _y: serde_types::Usize,
    ) -> anyhow::Result<(serde_types::U128, serde_ser::Usize)> {
        todo!() // self.serialize.test(&mut self.store, x, y)
    }
}

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializerProvider<S, T> {
    #[must_use]
    pub fn new(serializer: S, store: T) -> Self {
        Self { serializer, store }
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

    fn serialize_some(self, value: &serialize::Serialize) -> Result<SerOk, SerError> {
        Self::wrap_result(
            self.serializer
                .serialize_some(&SerializableSerialize::new(value, self.store)),
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

    fn serialize_newtype_struct(
        self,
        name: String,
        value: &serialize::Serialize,
    ) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_newtype_struct(
            intern_string(name),
            &SerializableSerialize::new(value, self.store),
        ))
    }

    fn serialize_newtype_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        value: &serialize::Serialize,
    ) -> Result<SerOk, SerError> {
        Self::wrap_result(self.serializer.serialize_newtype_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
            &SerializableSerialize::new(value, self.store),
        ))
    }

    fn serialize_seq(
        self,
        len: Option<serde_types::Usize>,
    ) -> Result<HostsideSerializeSeqProvider<S, T>, SerError> {
        Ok(HostsideSerializeSeqProvider {
            store: self.store,
            serialize_seq: self
                .serializer
                .serialize_seq(len.map(|len| wit_to_usize(len.val)))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_tuple(
        self,
        len: serde_types::Usize,
    ) -> Result<HostsideSerializeTupleProvider<S, T>, SerError> {
        Ok(HostsideSerializeTupleProvider {
            store: self.store,
            serialize_tuple: self
                .serializer
                .serialize_tuple(wit_to_usize(len.val))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: String,
        len: serde_types::Usize,
    ) -> Result<HostsideSerializeTupleStructProvider<S, T>, SerError> {
        Ok(HostsideSerializeTupleStructProvider {
            store: self.store,
            serialize_tuple_struct: self
                .serializer
                .serialize_tuple_struct(intern_string(name), wit_to_usize(len.val))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        len: serde_types::Usize,
    ) -> Result<HostsideSerializeTupleVariantProvider<S, T>, SerError> {
        Ok(HostsideSerializeTupleVariantProvider {
            store: self.store,
            serialize_tuple_variant: self
                .serializer
                .serialize_tuple_variant(
                    intern_string(name),
                    variant_index,
                    intern_string(variant),
                    wit_to_usize(len.val),
                )
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_map(
        self,
        len: Option<serde_types::Usize>,
    ) -> Result<HostsideSerializeMapProvider<S, T>, SerError> {
        Ok(HostsideSerializeMapProvider {
            store: self.store,
            serialize_map: self
                .serializer
                .serialize_map(len.map(|len| wit_to_usize(len.val)))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_struct(
        self,
        name: String,
        len: serde_types::Usize,
    ) -> Result<HostsideSerializeStructProvider<S, T>, SerError> {
        Ok(HostsideSerializeStructProvider {
            store: self.store,
            serialize_struct: self
                .serializer
                .serialize_struct(intern_string(name), wit_to_usize(len.val))
                .map_err(SerError::wrap)?,
        })
    }

    fn serialize_struct_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        len: serde_types::Usize,
    ) -> Result<HostsideSerializeStructVariantProvider<S, T>, SerError> {
        Ok(HostsideSerializeStructVariantProvider {
            store: self.store,
            serialize_struct_variant: self
                .serializer
                .serialize_struct_variant(
                    intern_string(name),
                    variant_index,
                    intern_string(variant),
                    wit_to_usize(len.val),
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
    Error {
        err: Any,
        display: String,
        debug: String,
    },
    Custom(String),
}

impl SerError {
    fn wrap<T: serde::ser::Error>(err: T) -> Self {
        let display = format!("{err}");
        let debug = format!("{err:?}");

        // Safety: TODO
        Self {
            inner: SerErrorOrCustom::Error {
                err: unsafe { Any::new(err) },
                display,
                debug,
            },
        }
    }

    fn take<T: serde::ser::Error>(self) -> Option<T> {
        match self.inner {
            // Safety: TODO
            SerErrorOrCustom::Error { err, .. } => unsafe { err.take() },
            SerErrorOrCustom::Custom(msg) => Some(serde::ser::Error::custom(msg)),
        }
    }

    fn display(&self) -> String {
        match &self.inner {
            SerErrorOrCustom::Error { display, .. } => String::from(display),
            SerErrorOrCustom::Custom(msg) => String::from(msg),
        }
    }

    fn debug(&self) -> String {
        match &self.inner {
            SerErrorOrCustom::Error { debug, .. } => {
                format!("serde_wit::ser::Error {{ err: {debug} }}")
            }
            SerErrorOrCustom::Custom(msg) => {
                format!("serde_wit::ser::Error {{ err: Custom({msg}) }}")
            }
        }
    }

    fn custom(msg: &str) -> Self {
        Self {
            inner: SerErrorOrCustom::Custom(String::from(msg)),
        }
    }
}

struct HostsideSerializeSeqProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_seq: S::SerializeSeq,
}

struct HostsideSerializeTupleProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_tuple: S::SerializeTuple,
}

struct HostsideSerializeTupleStructProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_tuple_struct: S::SerializeTupleStruct,
}

struct HostsideSerializeTupleVariantProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_tuple_variant: S::SerializeTupleVariant,
}

struct HostsideSerializeMapProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_map: S::SerializeMap,
}

struct HostsideSerializeStructProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_struct: S::SerializeStruct,
}

struct HostsideSerializeStructVariantProvider<S: serde::Serializer, T: wasmtime::AsContextMut> {
    store: T,
    serialize_struct_variant: S::SerializeStructVariant,
}

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeSeqProvider<S, T> {
    fn serialize_element(mut self, value: &serialize::Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_seq
            .serialize_element(&SerializableSerialize::new(value, &mut self.store))
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

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeTupleProvider<S, T> {
    fn serialize_element(mut self, value: &serialize::Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple
            .serialize_element(&SerializableSerialize::new(value, &mut self.store))
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

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeTupleStructProvider<S, T> {
    fn serialize_field(mut self, value: &serialize::Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple_struct
            .serialize_field(&SerializableSerialize::new(value, &mut self.store))
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

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeTupleVariantProvider<S, T> {
    fn serialize_field(mut self, value: &serialize::Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple_variant
            .serialize_field(&SerializableSerialize::new(value, &mut self.store))
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

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeMapProvider<S, T> {
    fn serialize_key(mut self, key: &serialize::Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_map
            .serialize_key(&SerializableSerialize::new(key, &mut self.store))
            .map_err(SerError::wrap);

        (self, result)
    }

    fn serialize_value(mut self, value: &serialize::Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_map
            .serialize_value(&SerializableSerialize::new(value, &mut self.store))
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

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeStructProvider<S, T> {
    fn serialize_field(
        mut self,
        key: String,
        value: &serialize::Serialize,
    ) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct
            .serialize_field(
                intern_string(key),
                &SerializableSerialize::new(value, &mut self.store),
            )
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

impl<S: serde::Serializer, T: wasmtime::AsContextMut> HostsideSerializeStructVariantProvider<S, T> {
    fn serialize_field(
        mut self,
        key: String,
        value: &serialize::Serialize,
    ) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct_variant
            .serialize_field(
                intern_string(key),
                &SerializableSerialize::new(value, &mut self.store),
            )
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

struct SerializableSerialize<'a, T: wasmtime::AsContextMut> {
    serialize: &'a serialize::Serialize,
    store: RefCell<T>,
}

impl<'a, T: wasmtime::AsContextMut> SerializableSerialize<'a, T> {
    fn new(serialize: &'a serialize::Serialize, store: T) -> Self {
        Self {
            serialize,
            store: RefCell::new(store),
        }
    }
}

impl<'a, T: wasmtime::AsContextMut> serde::Serialize for SerializableSerialize<'a, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Try to obtain mutable access to the store
        let Ok(mut store) = self.store.try_borrow_mut() else {
            return Err(serde::ser::Error::custom("bug: double Serialize::serialize"));
        };

        let result = self
            .serialize
            .serialize(HostsideSerializerProvider::new(serializer, &mut *store));

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

// TODO: remove
impl serialize::Serialize {
    fn serialize<S: serde::Serializer, T: wasmtime::AsContextMut>(
        &self,
        _serializer: HostsideSerializerProvider<S, T>,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}
