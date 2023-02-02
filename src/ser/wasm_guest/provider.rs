use alloc::{string::String, vec::Vec};

use scoped_reference::{ScopedBorrowMut, ScopedReference};
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    serde_if_integer128,
};

wit_bindgen_guest_rust::generate!({ world: "serde-serializer-provider", no_std });
export_serde_serializer_provider!(GuestsideSerializerProvider);

use crate::{any::Any, intern::intern_string};

pub struct GuestsideSerializerProvider {
    serializer: Box<dyn ErasedSerializer>,
    scope: ScopedBorrowMut<()>,
}

impl serializer::Serializer for GuestsideSerializerProvider {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> (serde_types::U128, serde_types::Usize) {
        serialize::test(x, y)
    }
}

impl GuestsideSerializerProvider {
    #[must_use]
    pub fn with_new<'a, S: serde::Serializer + 'a, F: FnOnce(Self) -> Q, Q>(
        serializer: S,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let serializer: Box<dyn ErasedSerializer + 'a> = Box::new(serializer);
            let serializer: Box<dyn ErasedSerializer + 'static> =
                unsafe { core::mem::transmute(serializer) };

            inner(Self {
                serializer,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    fn serialize_bool(self, v: bool) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_i64(v)
    }

    serde_if_integer128! {
        fn serialize_i128(self, v: serde_types::S128) -> Result<SerOk, SerError> {
            let le_hi = v.le_hi.to_le_bytes();
            let le_lo = v.le_lo.to_le_bytes();

            let bytes = [
                le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
                le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
            ];

            self.serializer.erased_serialize_i128(i128::from_le_bytes(bytes))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_u64(v)
    }

    serde_if_integer128! {
        fn serialize_u128(self, v: serde_types::U128) -> Result<SerOk, SerError> {
            let le_hi = v.le_hi.to_le_bytes();
            let le_lo = v.le_lo.to_le_bytes();

            let bytes = [
                le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
                le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
            ];

            self.serializer.erased_serialize_u128(u128::from_le_bytes(bytes))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_char(v)
    }

    fn serialize_str(self, v: String) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_str(&v)
    }

    fn serialize_bytes(self, v: Vec<u8>) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_bytes(&v)
    }

    fn serialize_none(self) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_none()
    }

    fn serialize_some(self, value: &Serialize) -> Result<SerOk, SerError> {
        self.serializer
            .erased_serialize_some(&SerializableSerialize::new(value))
    }

    fn serialize_unit(self) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_unit()
    }

    fn serialize_unit_struct(self, name: String) -> Result<SerOk, SerError> {
        self.serializer
            .erased_serialize_unit_struct(intern_string(name))
    }

    fn serialize_unit_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
    ) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_unit_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
        )
    }

    fn serialize_newtype_struct(self, name: String, value: &Serialize) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_newtype_struct(
            intern_string(name),
            &SerializableSerialize::new(value),
        )
    }

    fn serialize_newtype_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        value: &Serialize,
    ) -> Result<SerOk, SerError> {
        self.serializer.erased_serialize_newtype_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
            &SerializableSerialize::new(value),
        )
    }

    fn serialize_seq(
        self,
        len: Option<serde_types::Usize>,
    ) -> Result<GuestsideSerializeSeqProvider, SerError> {
        let serialize_seq = self
            .serializer
            .erased_serialize_seq(len.map(|len| len.val as usize))?;

        Ok(GuestsideSerializeSeqProvider {
            serialize_seq,
            scope: self.scope,
        })
    }

    fn serialize_tuple(
        self,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeTupleProvider, SerError> {
        let serialize_tuple = self.serializer.erased_serialize_tuple(len.val as usize)?;

        Ok(GuestsideSerializeTupleProvider {
            serialize_tuple,
            scope: self.scope,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeTupleStructProvider, SerError> {
        let serialize_tuple_struct = self
            .serializer
            .erased_serialize_tuple_struct(intern_string(name), len.val as usize)?;

        Ok(GuestsideSerializeTupleStructProvider {
            serialize_tuple_struct,
            scope: self.scope,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeTupleVariantProvider, SerError> {
        let serialize_tuple_variant = self.serializer.erased_serialize_tuple_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
            len.val as usize,
        )?;

        Ok(GuestsideSerializeTupleVariantProvider {
            serialize_tuple_variant,
            scope: self.scope,
        })
    }

    fn serialize_map(
        self,
        len: Option<serde_types::Usize>,
    ) -> Result<GuestsideSerializeMapProvider, SerError> {
        let serialize_map = self
            .serializer
            .erased_serialize_map(len.map(|len| len.val as usize))?;

        Ok(GuestsideSerializeMapProvider {
            serialize_map,
            scope: self.scope,
        })
    }

    fn serialize_struct(
        self,
        name: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeStructProvider, SerError> {
        let serialize_struct = self
            .serializer
            .erased_serialize_struct(intern_string(name), len.val as usize)?;

        Ok(GuestsideSerializeStructProvider {
            serialize_struct,
            scope: self.scope,
        })
    }

    fn serialize_struct_variant(
        self,
        name: String,
        variant_index: u32,
        variant: String,
        len: serde_types::Usize,
    ) -> Result<GuestsideSerializeStructVariantProvider, SerError> {
        let serialize_struct_variant = self.serializer.erased_serialize_struct_variant(
            intern_string(name),
            variant_index,
            intern_string(variant),
            len.val as usize,
        )?;

        Ok(GuestsideSerializeStructVariantProvider {
            serialize_struct_variant,
            scope: self.scope,
        })
    }

    fn is_human_readable(&self) -> bool {
        self.serializer.erased_is_human_readable()
    }
}

trait ErasedSerializer {
    fn erased_serialize_bool(self: Box<Self>, v: bool) -> Result<SerOk, SerError>;
    fn erased_serialize_i8(self: Box<Self>, v: i8) -> Result<SerOk, SerError>;
    fn erased_serialize_i16(self: Box<Self>, v: i16) -> Result<SerOk, SerError>;
    fn erased_serialize_i32(self: Box<Self>, v: i32) -> Result<SerOk, SerError>;
    fn erased_serialize_i64(self: Box<Self>, v: i64) -> Result<SerOk, SerError>;
    fn erased_serialize_u8(self: Box<Self>, v: u8) -> Result<SerOk, SerError>;
    fn erased_serialize_u16(self: Box<Self>, v: u16) -> Result<SerOk, SerError>;
    fn erased_serialize_u32(self: Box<Self>, v: u32) -> Result<SerOk, SerError>;
    fn erased_serialize_u64(self: Box<Self>, v: u64) -> Result<SerOk, SerError>;
    serde_if_integer128! {
        fn erased_serialize_i128(self: Box<Self>, v: i128) -> Result<SerOk, SerError>;
        fn erased_serialize_u128(self: Box<Self>, v: u128) -> Result<SerOk, SerError>;
    }
    fn erased_serialize_f32(self: Box<Self>, v: f32) -> Result<SerOk, SerError>;
    fn erased_serialize_f64(self: Box<Self>, v: f64) -> Result<SerOk, SerError>;
    fn erased_serialize_char(self: Box<Self>, v: char) -> Result<SerOk, SerError>;
    fn erased_serialize_str(self: Box<Self>, v: &str) -> Result<SerOk, SerError>;
    fn erased_serialize_bytes(self: Box<Self>, v: &[u8]) -> Result<SerOk, SerError>;
    fn erased_serialize_none(self: Box<Self>) -> Result<SerOk, SerError>;
    fn erased_serialize_some(self: Box<Self>, v: &SerializableSerialize)
        -> Result<SerOk, SerError>;
    fn erased_serialize_unit(self: Box<Self>) -> Result<SerOk, SerError>;
    fn erased_serialize_unit_struct(self: Box<Self>, name: &'static str)
        -> Result<SerOk, SerError>;
    fn erased_serialize_unit_variant(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<SerOk, SerError>;
    fn erased_serialize_newtype_struct(
        self: Box<Self>,
        name: &'static str,
        v: &SerializableSerialize,
    ) -> Result<SerOk, SerError>;
    fn erased_serialize_newtype_variant(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        v: &SerializableSerialize,
    ) -> Result<SerOk, SerError>;
    fn erased_serialize_seq<'a>(
        self: Box<Self>,
        len: Option<usize>,
    ) -> Result<Box<dyn ErasedSerializeSeq + 'a>, SerError>
    where
        Self: 'a;
    fn erased_serialize_tuple<'a>(
        self: Box<Self>,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeTuple + 'a>, SerError>
    where
        Self: 'a;
    fn erased_serialize_tuple_struct<'a>(
        self: Box<Self>,
        name: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeTupleStruct + 'a>, SerError>
    where
        Self: 'a;
    fn erased_serialize_tuple_variant<'a>(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeTupleVariant + 'a>, SerError>
    where
        Self: 'a;
    fn erased_serialize_map<'a>(
        self: Box<Self>,
        len: Option<usize>,
    ) -> Result<Box<dyn ErasedSerializeMap + 'a>, SerError>
    where
        Self: 'a;
    fn erased_serialize_struct<'a>(
        self: Box<Self>,
        name: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeStruct + 'a>, SerError>
    where
        Self: 'a;
    fn erased_serialize_struct_variant<'a>(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeStructVariant + 'a>, SerError>
    where
        Self: 'a;
    fn erased_is_human_readable(&self) -> bool;
}

trait ErasedSerializeSeq {
    fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
}

trait ErasedSerializeTuple {
    fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
}

trait ErasedSerializeTupleStruct {
    fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
}

trait ErasedSerializeTupleVariant {
    fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
}

trait ErasedSerializeMap {
    fn erased_serialize_key(&mut self, key: &SerializableSerialize) -> Result<(), SerError>;
    fn erased_serialize_value(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
}

trait ErasedSerializeStruct {
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &SerializableSerialize,
    ) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError>;
}

trait ErasedSerializeStructVariant {
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &SerializableSerialize,
    ) -> Result<(), SerError>;
    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError>;
}

impl<T: serde::Serializer> ErasedSerializer for T {
    fn erased_serialize_bool(self: Box<Self>, v: bool) -> Result<SerOk, SerError> {
        self.serialize_bool(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_i8(self: Box<Self>, v: i8) -> Result<SerOk, SerError> {
        self.serialize_i8(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_i16(self: Box<Self>, v: i16) -> Result<SerOk, SerError> {
        self.serialize_i16(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_i32(self: Box<Self>, v: i32) -> Result<SerOk, SerError> {
        self.serialize_i32(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_i64(self: Box<Self>, v: i64) -> Result<SerOk, SerError> {
        self.serialize_i64(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    serde_if_integer128! {
        fn erased_serialize_i128(self:Box<Self>,v:i128) -> Result<SerOk,SerError> {
            self.serialize_i128(v).map(SerOk::wrap).map_err(SerError::wrap)
        }
    }

    fn erased_serialize_u8(self: Box<Self>, v: u8) -> Result<SerOk, SerError> {
        self.serialize_u8(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_u16(self: Box<Self>, v: u16) -> Result<SerOk, SerError> {
        self.serialize_u16(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_u32(self: Box<Self>, v: u32) -> Result<SerOk, SerError> {
        self.serialize_u32(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_u64(self: Box<Self>, v: u64) -> Result<SerOk, SerError> {
        self.serialize_u64(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    serde_if_integer128! {
        fn erased_serialize_u128(self:Box<Self>,v:u128) -> Result<SerOk,SerError> {
            self.serialize_u128(v).map(SerOk::wrap).map_err(SerError::wrap)
        }
    }

    fn erased_serialize_f32(self: Box<Self>, v: f32) -> Result<SerOk, SerError> {
        self.serialize_f32(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_f64(self: Box<Self>, v: f64) -> Result<SerOk, SerError> {
        self.serialize_f64(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_char(self: Box<Self>, v: char) -> Result<SerOk, SerError> {
        self.serialize_char(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_str(self: Box<Self>, v: &str) -> Result<SerOk, SerError> {
        self.serialize_str(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_bytes(self: Box<Self>, v: &[u8]) -> Result<SerOk, SerError> {
        self.serialize_bytes(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_none(self: Box<Self>) -> Result<SerOk, SerError> {
        self.serialize_none()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_some(
        self: Box<Self>,
        v: &SerializableSerialize,
    ) -> Result<SerOk, SerError> {
        self.serialize_some(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_unit(self: Box<Self>) -> Result<SerOk, SerError> {
        self.serialize_unit()
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_unit_struct(
        self: Box<Self>,
        name: &'static str,
    ) -> Result<SerOk, SerError> {
        self.serialize_unit_struct(name)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_unit_variant(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<SerOk, SerError> {
        self.serialize_unit_variant(name, variant_index, variant)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_newtype_struct(
        self: Box<Self>,
        name: &'static str,
        v: &SerializableSerialize,
    ) -> Result<SerOk, SerError> {
        self.serialize_newtype_struct(name, v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_newtype_variant(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        v: &SerializableSerialize,
    ) -> Result<SerOk, SerError> {
        self.serialize_newtype_variant(name, variant_index, variant, v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
    }

    fn erased_serialize_seq<'a>(
        self: Box<Self>,
        len: Option<usize>,
    ) -> Result<Box<dyn ErasedSerializeSeq + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_seq(len)
            .map(|serialize_seq| {
                let serialize_seq: Box<dyn ErasedSerializeSeq + 'a> = Box::new(serialize_seq);
                serialize_seq
            })
            .map_err(SerError::wrap)
    }

    fn erased_serialize_tuple<'a>(
        self: Box<Self>,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeTuple + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_tuple(len)
            .map(|serialize_tuple| {
                let serialize_tuple: Box<dyn ErasedSerializeTuple + 'a> = Box::new(serialize_tuple);
                serialize_tuple
            })
            .map_err(SerError::wrap)
    }

    fn erased_serialize_tuple_struct<'a>(
        self: Box<Self>,
        name: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeTupleStruct + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_tuple_struct(name, len)
            .map(|serialize_tuple_struct| {
                let serialize_tuple_struct: Box<dyn ErasedSerializeTupleStruct + 'a> =
                    Box::new(serialize_tuple_struct);
                serialize_tuple_struct
            })
            .map_err(SerError::wrap)
    }

    fn erased_serialize_tuple_variant<'a>(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeTupleVariant + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_tuple_variant(name, variant_index, variant, len)
            .map(|serialize_tuple_variant| {
                let serialize_tuple_variant: Box<dyn ErasedSerializeTupleVariant + 'a> =
                    Box::new(serialize_tuple_variant);
                serialize_tuple_variant
            })
            .map_err(SerError::wrap)
    }

    fn erased_serialize_map<'a>(
        self: Box<Self>,
        len: Option<usize>,
    ) -> Result<Box<dyn ErasedSerializeMap + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_map(len)
            .map(|serialize_map| {
                let serialize_map: Box<dyn ErasedSerializeMap + 'a> = Box::new(serialize_map);
                serialize_map
            })
            .map_err(SerError::wrap)
    }

    fn erased_serialize_struct<'a>(
        self: Box<Self>,
        name: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeStruct + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_struct(name, len)
            .map(|serialize_struct| {
                let serialize_struct: Box<dyn ErasedSerializeStruct + 'a> =
                    Box::new(serialize_struct);
                serialize_struct
            })
            .map_err(SerError::wrap)
    }

    fn erased_serialize_struct_variant<'a>(
        self: Box<Self>,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Box<dyn ErasedSerializeStructVariant + 'a>, SerError>
    where
        Self: 'a,
    {
        self.serialize_struct_variant(name, variant_index, variant, len)
            .map(|serialize_struct_variant| {
                let serialize_struct_variant: Box<dyn ErasedSerializeStructVariant + 'a> =
                    Box::new(serialize_struct_variant);
                serialize_struct_variant
            })
            .map_err(SerError::wrap)
    }

    fn erased_is_human_readable(&self) -> bool {
        self.is_human_readable()
    }
}

impl<T: SerializeSeq> ErasedSerializeSeq for T {
    fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
        self.serialize_element(value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }
}

impl<T: SerializeTuple> ErasedSerializeTuple for T {
    fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
        self.serialize_element(value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }
}

impl<T: SerializeTupleStruct> ErasedSerializeTupleStruct for T {
    fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
        self.serialize_field(value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }
}

impl<T: SerializeTupleVariant> ErasedSerializeTupleVariant for T {
    fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
        self.serialize_field(value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }
}

impl<T: SerializeMap> ErasedSerializeMap for T {
    fn erased_serialize_key(&mut self, key: &SerializableSerialize) -> Result<(), SerError> {
        self.serialize_key(key).map_err(SerError::wrap)
    }

    fn erased_serialize_value(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
        self.serialize_value(value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }
}

impl<T: SerializeStruct> ErasedSerializeStruct for T {
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &SerializableSerialize,
    ) -> Result<(), SerError> {
        self.serialize_field(key, value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }

    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError> {
        self.skip_field(key).map_err(SerError::wrap)
    }
}

impl<T: SerializeStructVariant> ErasedSerializeStructVariant for T {
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &SerializableSerialize,
    ) -> Result<(), SerError> {
        self.serialize_field(key, value).map_err(SerError::wrap)
    }

    fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
        self.end().map(SerOk::wrap).map_err(SerError::wrap)
    }

    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError> {
        self.skip_field(key).map_err(SerError::wrap)
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
            SerErrorOrCustom::Custom(msg) => format!("serde_wit::ser::Error {{ custom: {msg} }}"),
        }
    }

    fn custom(msg: &str) -> Self {
        Self {
            inner: SerErrorOrCustom::Custom(String::from(msg)),
        }
    }
}

struct GuestsideSerializeSeqProvider {
    serialize_seq: Box<dyn ErasedSerializeSeq>,
    scope: ScopedBorrowMut<()>,
}

struct GuestsideSerializeTupleProvider {
    serialize_tuple: Box<dyn ErasedSerializeTuple>,
    scope: ScopedBorrowMut<()>,
}

struct GuestsideSerializeTupleStructProvider {
    serialize_tuple_struct: Box<dyn ErasedSerializeTupleStruct>,
    scope: ScopedBorrowMut<()>,
}

struct GuestsideSerializeTupleVariantProvider {
    serialize_tuple_variant: Box<dyn ErasedSerializeTupleVariant>,
    scope: ScopedBorrowMut<()>,
}

struct GuestsideSerializeMapProvider {
    serialize_map: Box<dyn ErasedSerializeMap>,
    scope: ScopedBorrowMut<()>,
}

struct GuestsideSerializeStructProvider {
    serialize_struct: Box<dyn ErasedSerializeStruct>,
    scope: ScopedBorrowMut<()>,
}

struct GuestsideSerializeStructVariantProvider {
    serialize_struct_variant: Box<dyn ErasedSerializeStructVariant>,
    scope: ScopedBorrowMut<()>,
}

impl GuestsideSerializeSeqProvider {
    fn serialize_element(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_seq
            .erased_serialize_element(&SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_seq.erased_end()
    }
}

impl GuestsideSerializeTupleProvider {
    fn serialize_element(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple
            .erased_serialize_element(&SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_tuple.erased_end()
    }
}

impl GuestsideSerializeTupleStructProvider {
    fn serialize_field(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple_struct
            .erased_serialize_field(&SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_tuple_struct.erased_end()
    }
}

impl GuestsideSerializeTupleVariantProvider {
    fn serialize_field(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_tuple_variant
            .erased_serialize_field(&SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_tuple_variant.erased_end()
    }
}

impl GuestsideSerializeMapProvider {
    fn serialize_key(mut self, key: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_map
            .erased_serialize_key(&SerializableSerialize::new(key));

        (self, result)
    }

    fn serialize_value(mut self, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_map
            .erased_serialize_value(&SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_map.erased_end()
    }
}

impl GuestsideSerializeStructProvider {
    fn serialize_field(mut self, key: String, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct
            .erased_serialize_field(intern_string(key), &SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_struct.erased_end()
    }

    fn skip_field(mut self, key: String) -> (Self, Result<(), SerError>) {
        let result = self.serialize_struct.erased_skip_field(intern_string(key));

        (self, result)
    }
}

impl GuestsideSerializeStructVariantProvider {
    fn serialize_field(mut self, key: String, value: &Serialize) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct_variant
            .erased_serialize_field(intern_string(key), &SerializableSerialize::new(value));

        (self, result)
    }

    fn end(self) -> Result<SerOk, SerError> {
        self.serialize_struct_variant.erased_end()
    }

    fn skip_field(mut self, key: String) -> (Self, Result<(), SerError>) {
        let result = self
            .serialize_struct_variant
            .erased_skip_field(intern_string(key));

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
        let result = GuestsideSerializerProvider::with_new(serializer, |serializer| {
            self.serialize.serialize(serializer)
        });

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
    fn serialize(&self, _serializer: GuestsideSerializerProvider) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}
