use alloc::{boxed::Box, vec::Vec, string::String, format};
use core::fmt;

use scoped_reference::{ScopedBorrowMut, ScopedReference};
use serde::serde_if_integer128;

wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-client", no_std });
export_serde_deserializer_client!(GuestsideDeserializerClient);

use crate::any::Any;

pub struct GuestsideDeserializerClient {
    deserialize_seed: Box<dyn ErasedDeserializeSeed>,
    scope: ScopedBorrowMut<()>,
}

impl deserialize::Deserialize for GuestsideDeserializerClient {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> Result<(serde_types::U128, serde_types::Usize), serde_de::Unexpected> {
        deserializer::test(x, y)
    }
}

impl GuestsideDeserializerClient {
    #[must_use]
    pub fn with_new<'a, D: for<'de> serde::de::Deserialize<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        inner: F,
    ) -> Q {
        Self::with_new_seed(core::marker::PhantomData::<D>, inner)
    }

    #[must_use]
    pub fn with_new_seed<
        'a,
        D: for<'de> serde::de::DeserializeSeed<'de> + 'a,
        F: FnOnce(Self) -> Q,
        Q,
    >(
        deserialize_seed: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let deserialize_seed: Box<dyn ErasedDeserializeSeed + 'a> = Box::new(deserialize_seed);
            let deserialize_seed: Box<dyn ErasedDeserializeSeed + 'static> =
                unsafe { core::mem::transmute(deserialize_seed) };

            inner(Self {
                deserialize_seed,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    unsafe fn with_new_seed_unchecked<
        'a,
        'de,
        D: serde::de::DeserializeSeed<'de> + 'a,
        F: FnOnce(Self) -> Q,
        Q,
    >(
        deserialize_seed: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let deserialize_seed: Box<dyn ErasedDeserializeSeed + 'a> = Box::new(deserialize_seed);
            let deserialize_seed: Box<dyn ErasedDeserializeSeed + 'static> =
                unsafe { core::mem::transmute(deserialize_seed) };

            inner(Self {
                deserialize_seed,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    fn deserialize(self, deserializer: Deserializer) -> Result<DeValue, DeError> {
        ErasedDeserializeSeed::erased_deserialize(
            self.deserialize_seed,
            DeserializerableDeserializer { deserializer },
        )
    }
}

trait ErasedDeserializeSeed {
    fn erased_deserialize(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError>;
}

trait ErasedVisitor {
    fn erased_expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
    fn erased_visit_bool(self: Box<Self>, v: bool) -> Result<DeValue, DeError>;
    fn erased_visit_i8(self: Box<Self>, v: i8) -> Result<DeValue, DeError>;
    fn erased_visit_i16(self: Box<Self>, v: i16) -> Result<DeValue, DeError>;
    fn erased_visit_i32(self: Box<Self>, v: i32) -> Result<DeValue, DeError>;
    fn erased_visit_i64(self: Box<Self>, v: i64) -> Result<DeValue, DeError>;
    fn erased_visit_u8(self: Box<Self>, v: u8) -> Result<DeValue, DeError>;
    fn erased_visit_u16(self: Box<Self>, v: u16) -> Result<DeValue, DeError>;
    fn erased_visit_u32(self: Box<Self>, v: u32) -> Result<DeValue, DeError>;
    fn erased_visit_u64(self: Box<Self>, v: u64) -> Result<DeValue, DeError>;
    serde_if_integer128! {
        fn erased_visit_i128(self: Box<Self>, v: i128) -> Result<DeValue, DeError>;
        fn erased_visit_u128(self: Box<Self>, v: u128) -> Result<DeValue, DeError>;
    }
    fn erased_visit_f32(self: Box<Self>, v: f32) -> Result<DeValue, DeError>;
    fn erased_visit_f64(self: Box<Self>, v: f64) -> Result<DeValue, DeError>;
    fn erased_visit_char(self: Box<Self>, v: char) -> Result<DeValue, DeError>;
    fn erased_visit_string(self: Box<Self>, v: String) -> Result<DeValue, DeError>;
    fn erased_visit_byte_buf(self: Box<Self>, v: Vec<u8>) -> Result<DeValue, DeError>;
    fn erased_visit_none(self: Box<Self>) -> Result<DeValue, DeError>;
    fn erased_visit_some(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError>;
    fn erased_visit_unit(self: Box<Self>) -> Result<DeValue, DeError>;
    fn erased_visit_newtype_struct(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError>;
    fn erased_visit_seq(
        self: Box<Self>,
        seq: DeserializerableSeqAccess,
    ) -> Result<DeValue, DeError>;
    fn erased_visit_map(
        self: Box<Self>,
        map: DeserializerableMapAccess,
    ) -> Result<DeValue, DeError>;
    fn erased_visit_enum(
        self: Box<Self>,
        data: DeserializerableEnumAccess,
    ) -> Result<DeValue, DeError>;
}

impl<'de, T: serde::de::DeserializeSeed<'de>> ErasedDeserializeSeed for T {
    fn erased_deserialize(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError> {
        self.deserialize(deserializer).map(DeValue::wrap)
    }
}

impl<'de, T: serde::de::Visitor<'de>> ErasedVisitor for T {
    fn erased_expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.expecting(formatter)
    }

    fn erased_visit_bool(self: Box<Self>, v: bool) -> Result<DeValue, DeError> {
        self.visit_bool(v).map(DeValue::wrap)
    }

    fn erased_visit_i8(self: Box<Self>, v: i8) -> Result<DeValue, DeError> {
        self.visit_i8(v).map(DeValue::wrap)
    }

    fn erased_visit_i16(self: Box<Self>, v: i16) -> Result<DeValue, DeError> {
        self.visit_i16(v).map(DeValue::wrap)
    }

    fn erased_visit_i32(self: Box<Self>, v: i32) -> Result<DeValue, DeError> {
        self.visit_i32(v).map(DeValue::wrap)
    }

    fn erased_visit_i64(self: Box<Self>, v: i64) -> Result<DeValue, DeError> {
        self.visit_i64(v).map(DeValue::wrap)
    }

    serde_if_integer128! {
        fn erased_visit_i128(self: Box<Self>, v: i128) -> Result<DeValue, DeError> {
            self.visit_i128(v).map(DeValue::wrap)
        }
    }

    fn erased_visit_u8(self: Box<Self>, v: u8) -> Result<DeValue, DeError> {
        self.visit_u8(v).map(DeValue::wrap)
    }

    fn erased_visit_u16(self: Box<Self>, v: u16) -> Result<DeValue, DeError> {
        self.visit_u16(v).map(DeValue::wrap)
    }

    fn erased_visit_u32(self: Box<Self>, v: u32) -> Result<DeValue, DeError> {
        self.visit_u32(v).map(DeValue::wrap)
    }

    fn erased_visit_u64(self: Box<Self>, v: u64) -> Result<DeValue, DeError> {
        self.visit_u64(v).map(DeValue::wrap)
    }

    serde_if_integer128! {
        fn erased_visit_u128(self: Box<Self>, v: u128) -> Result<DeValue, DeError> {
            self.visit_u128(v).map(DeValue::wrap)
        }
    }

    fn erased_visit_f32(self: Box<Self>, v: f32) -> Result<DeValue, DeError> {
        self.visit_f32(v).map(DeValue::wrap)
    }

    fn erased_visit_f64(self: Box<Self>, v: f64) -> Result<DeValue, DeError> {
        self.visit_f64(v).map(DeValue::wrap)
    }

    fn erased_visit_char(self: Box<Self>, v: char) -> Result<DeValue, DeError> {
        self.visit_char(v).map(DeValue::wrap)
    }

    fn erased_visit_string(self: Box<Self>, v: String) -> Result<DeValue, DeError> {
        self.visit_string(v).map(DeValue::wrap)
    }

    fn erased_visit_byte_buf(self: Box<Self>, v: Vec<u8>) -> Result<DeValue, DeError> {
        self.visit_byte_buf(v).map(DeValue::wrap)
    }

    fn erased_visit_none(self: Box<Self>) -> Result<DeValue, DeError> {
        self.visit_none().map(DeValue::wrap)
    }

    fn erased_visit_some(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError> {
        self.visit_some(deserializer).map(DeValue::wrap)
    }

    fn erased_visit_unit(self: Box<Self>) -> Result<DeValue, DeError> {
        self.visit_unit().map(DeValue::wrap)
    }

    fn erased_visit_newtype_struct(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError> {
        self.visit_newtype_struct(deserializer).map(DeValue::wrap)
    }

    fn erased_visit_seq(
        self: Box<Self>,
        seq: DeserializerableSeqAccess,
    ) -> Result<DeValue, DeError> {
        self.visit_seq(seq).map(DeValue::wrap)
    }

    fn erased_visit_map(
        self: Box<Self>,
        map: DeserializerableMapAccess,
    ) -> Result<DeValue, DeError> {
        self.visit_map(map).map(DeValue::wrap)
    }

    fn erased_visit_enum(
        self: Box<Self>,
        data: DeserializerableEnumAccess,
    ) -> Result<DeValue, DeError> {
        self.visit_enum(data).map(DeValue::wrap)
    }
}

struct DeserializerableDeserializer {
    deserializer: Deserializer,
}

impl Visitor {
    fn with_new<'de, V: serde::de::Visitor<'de>, F: FnOnce(Self) -> Result<DeValue, DeError>>(
        visitor: V,
        inner: F,
    ) -> Result<V::Value, DeError> {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let visitor: Box<dyn ErasedVisitor + '_> = Box::new(visitor);
            let visitor: Box<dyn ErasedVisitor + 'static> =
                unsafe { core::mem::transmute(visitor) };

            inner(Self {
                visitor,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        match result {
            Ok(value) => {
                if let Some(value) = value.take() {
                    return Ok(value);
                }
            }
            Err(err) => return Err(err),
        };

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn expecting(&self) -> String {
        struct Expecting<'a> {
            visitor: &'a Visitor,
        }

        impl<'a> fmt::Display for Expecting<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.visitor.visitor.erased_expecting(f)
            }
        }

        format!("{}", Expecting { visitor: self })
    }

    fn visit_bool(self, v: bool) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_bool(v)
    }

    fn visit_i8(self, v: i8) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_i8(v)
    }

    fn visit_i16(self, v: i16) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_i16(v)
    }

    fn visit_i32(self, v: i32) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_i32(v)
    }

    fn visit_i64(self, v: i64) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_i64(v)
    }

    serde_if_integer128! {
        fn visit_i128(self, v: i128) -> Result<DeValue, DeError> {
            self.visitor.erased_visit_i128(v)
        }
    }

    fn visit_u8(self, v: u8) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_u8(v)
    }

    fn visit_u16(self, v: u16) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_u16(v)
    }

    fn visit_u32(self, v: u32) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_u32(v)
    }

    fn visit_u64(self, v: u64) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_u64(v)
    }

    serde_if_integer128! {
        fn visit_u128(self, v: u128) -> Result<DeValue, DeError> {
            self.visitor.erased_visit_u128(v)
        }
    }

    fn visit_f32(self, v: f32) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_f32(v)
    }

    fn visit_f64(self, v: f64) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_f64(v)
    }

    fn visit_char(self, v: char) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_char(v)
    }

    fn visit_string(self, v: String) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_string(v)
    }

    fn visit_byte_buf(self, v: Vec<u8>) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_byte_buf(v)
    }

    fn visit_none(self) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_none()
    }

    fn visit_some(self, deserializer: Deserializer) -> Result<DeValue, DeError> {
        self.visitor
            .erased_visit_some(DeserializerableDeserializer { deserializer })
    }

    fn visit_unit(self) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_unit()
    }

    fn visit_newtype_struct(self, deserializer: Deserializer) -> Result<DeValue, DeError> {
        self.visitor
            .erased_visit_newtype_struct(DeserializerableDeserializer { deserializer })
    }

    fn visit_seq(self, seq: SeqAccess) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_seq(DeserializerableSeqAccess {
            seq_access: Some(seq),
        })
    }

    fn visit_map(self, map: MapAccess) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_map(DeserializerableMapAccess {
            map_access: Some(map),
        })
    }

    fn visit_enum(self, data: EnumAccess) -> Result<DeValue, DeError> {
        self.visitor
            .erased_visit_enum(DeserializerableEnumAccess { enum_access: data })
    }
}

impl<'de> serde::Deserializer<'de> for DeserializerableDeserializer {
    type Error = DeError;

    fn deserialize_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_any(visitor)
        })
    }

    fn deserialize_bool<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_bool(visitor)
        })
    }

    fn deserialize_i8<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| self.deserializer.deserialize_i8(visitor))
    }

    fn deserialize_i16<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_i16(visitor)
        })
    }

    fn deserialize_i32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_i32(visitor)
        })
    }

    fn deserialize_i64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_i64(visitor)
        })
    }

    serde_if_integer128! {
        fn deserialize_i128<V: serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            Visitor::with_new(visitor, |visitor| self.deserializer.deserialize_i128(visitor))
        }
    }

    fn deserialize_u8<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| self.deserializer.deserialize_u8(visitor))
    }

    fn deserialize_u16<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_u16(visitor)
        })
    }

    fn deserialize_u32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_u32(visitor)
        })
    }

    fn deserialize_u64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_u64(visitor)
        })
    }

    serde_if_integer128! {
        fn deserialize_u128<V: serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            Visitor::with_new(visitor, |visitor| self.deserializer.deserialize_u128(visitor))
        }
    }

    fn deserialize_f32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_f32(visitor)
        })
    }

    fn deserialize_f64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_f64(visitor)
        })
    }

    fn deserialize_char<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_char(visitor)
        })
    }

    fn deserialize_str<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_str(visitor)
        })
    }

    fn deserialize_string<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_string(visitor)
        })
    }

    fn deserialize_bytes<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_bytes(visitor)
        })
    }

    fn deserialize_byte_buf<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_byte_buf(visitor)
        })
    }

    fn deserialize_option<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_option(visitor)
        })
    }

    fn deserialize_unit<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_unit(visitor)
        })
    }

    fn deserialize_unit_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_unit_struct(name, visitor)
        })
    }

    fn deserialize_newtype_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_newtype_struct(name, visitor)
        })
    }

    fn deserialize_seq<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_seq(visitor)
        })
    }

    fn deserialize_tuple<V: serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_tuple(len, visitor)
        })
    }

    fn deserialize_tuple_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer
                .deserialize_tuple_struct(name, len, visitor)
        })
    }

    fn deserialize_map<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_map(visitor)
        })
    }

    fn deserialize_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_struct(name, fields, visitor)
        })
    }

    fn deserialize_enum<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_enum(name, variants, visitor)
        })
    }

    fn deserialize_identifier<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_identifier(visitor)
        })
    }

    fn deserialize_ignored_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.deserializer.deserialize_ignored_any(visitor)
        })
    }

    fn is_human_readable(&self) -> bool {
        self.deserializer.is_human_readable()
    }
}

impl Deserializer {
    fn deserialize_any(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_bool(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_i8(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_i16(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_i32(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_i64(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_i128(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_u8(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_u16(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_u32(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_u64(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_u128(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_f32(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_f64(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_char(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_str(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_string(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_bytes(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_byte_buf(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_option(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_unit(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_unit_struct(self, _name: &str, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_newtype_struct(
        self,
        _name: &str,
        _visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_seq(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_tuple(self, _len: usize, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_tuple_struct(
        self,
        _name: &str,
        _len: usize,
        _visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_map(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_struct(
        self,
        _name: &str,
        _fields: &[&str],
        _visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_enum(
        self,
        _name: &str,
        _variants: &[&str],
        _visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_identifier(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_ignored_any(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn is_human_readable(&self) -> bool {
        todo!("wit-bindgen")
    }
}

struct Visitor {
    visitor: Box<dyn ErasedVisitor>,
    scope: ScopedBorrowMut<()>,
}

struct SeqAccess {
    _private: (),
}

impl SeqAccess {
    fn next_element_seed(
        self,
        _seed: GuestsideDeserializerClient,
    ) -> (Self, Result<Option<DeValue>, DeError>) {
        todo!("wit-bindgen")
    }

    fn size_hint(&self) -> Option<usize> {
        todo!("wit-bindgen")
    }
}

struct DeserializerableSeqAccess {
    seq_access: Option<SeqAccess>,
}

impl<'de> serde::de::SeqAccess<'de> for DeserializerableSeqAccess {
    type Error = DeError;

    fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        let Some(seq_access) = self.seq_access.take() else {
            return Err(serde::de::Error::custom("bug: SeqAccess::next_element_seed after free"));
        };

        let (seq_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                seq_access.next_element_seed(seed)
            })
        };
        self.seq_access = Some(seq_access);

        match result {
            Ok(Some(value)) => {
                if let Some(value) = value.take() {
                    return Ok(Some(value));
                }
            }
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn size_hint(&self) -> Option<usize> {
        self.seq_access
            .as_ref()
            .and_then(|seq_access| seq_access.size_hint())
    }
}

struct MapAccess {
    _private: (),
}

impl MapAccess {
    fn next_key_seed(
        self,
        _seed: GuestsideDeserializerClient,
    ) -> (Self, Result<Option<DeValue>, DeError>) {
        todo!("wit-bindgen")
    }

    fn next_value_seed(
        self,
        _seed: GuestsideDeserializerClient,
    ) -> (Self, Result<DeValue, DeError>) {
        todo!("wit-bindgen")
    }

    fn size_hint(&self) -> Option<usize> {
        todo!("wit-bindgen")
    }
}

struct DeserializerableMapAccess {
    map_access: Option<MapAccess>,
}

impl<'de> serde::de::MapAccess<'de> for DeserializerableMapAccess {
    type Error = DeError;

    fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(serde::de::Error::custom("bug: MapAccess::next_key_seed after free"));
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                map_access.next_key_seed(seed)
            })
        };
        self.map_access = Some(map_access);

        match result {
            Ok(Some(value)) => {
                if let Some(value) = value.take() {
                    return Ok(Some(value));
                }
            }
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(serde::de::Error::custom("bug: MapAccess::next_key_seed after free"));
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                map_access.next_value_seed(seed)
            })
        };
        self.map_access = Some(map_access);

        match result {
            Ok(value) => {
                if let Some(value) = value.take() {
                    return Ok(value);
                }
            }
            Err(err) => return Err(err),
        };

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn next_entry_seed<K: serde::de::DeserializeSeed<'de>, V: serde::de::DeserializeSeed<'de>>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(serde::de::Error::custom("bug: MapAccess::next_entry_seed after free"));
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(kseed, |seed| {
                map_access.next_key_seed(seed)
            })
        };

        let key = match result {
            Ok(Some(key)) => key,
            Ok(None) => {
                self.map_access = Some(map_access);
                return Ok(None);
            }
            Err(err) => {
                self.map_access = Some(map_access);
                return Err(err);
            }
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(vseed, |seed| {
                map_access.next_value_seed(seed)
            })
        };
        self.map_access = Some(map_access);

        match result {
            Ok(value) => {
                if let (Some(key), Some(value)) = (key.take(), value.take()) {
                    return Ok(Some((key, value)));
                }
            }
            Err(err) => return Err(err),
        };

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn size_hint(&self) -> Option<usize> {
        self.map_access
            .as_ref()
            .and_then(|map_access| map_access.size_hint())
    }
}

struct EnumAccess {
    _private: (),
}

impl EnumAccess {
    fn variant_seed(
        self,
        _seed: GuestsideDeserializerClient,
    ) -> Result<(DeValue, VariantAccess), DeError> {
        todo!("wit-bindgen")
    }
}

struct DeserializerableEnumAccess {
    enum_access: EnumAccess,
}

impl<'de> serde::de::EnumAccess<'de> for DeserializerableEnumAccess {
    type Error = DeError;
    type Variant = DeserializerableVariantAccess;

    fn variant_seed<V: serde::de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let result = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                self.enum_access.variant_seed(seed)
            })
        };

        match result {
            Ok((value, variant_access)) => {
                if let Some(value) = value.take() {
                    return Ok((value, DeserializerableVariantAccess { variant_access }));
                }
            }
            Err(err) => return Err(err),
        }

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }
}

struct VariantAccess {
    _private: (),
}

impl VariantAccess {
    fn unit_variant(self) -> Result<(), DeError> {
        todo!("wit-bindgen")
    }

    fn newtype_variant_seed(self, _seed: GuestsideDeserializerClient) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn tuple_variant(self, _len: usize, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn struct_variant(self, _fields: &[&str], _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }
}

struct DeserializerableVariantAccess {
    variant_access: VariantAccess,
}

impl<'de> serde::de::VariantAccess<'de> for DeserializerableVariantAccess {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.variant_access.unit_variant()
    }

    fn newtype_variant_seed<T: serde::de::DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        let result = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                self.variant_access.newtype_variant_seed(seed)
            })
        };

        match result {
            Ok(value) => {
                if let Some(value) = value.take() {
                    return Ok(value);
                }
            }
            Err(err) => return Err(err),
        }

        Err(serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn tuple_variant<V: serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.variant_access.tuple_variant(len, visitor)
        })
    }

    fn struct_variant<V: serde::de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            self.variant_access.struct_variant(fields, visitor)
        })
    }
}

struct DeValue {
    value: Any,
}

impl DeValue {
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

struct DeError {
    _private: (),
}

fn translate_serde_de_unexpected(unexp: serde::de::Unexpected) -> serde_de::Unexpected {
    match unexp {
        serde::de::Unexpected::Bool(v) => serde_de::Unexpected::Bool(v),
        serde::de::Unexpected::Unsigned(v) => serde_de::Unexpected::Unsigned(v),
        serde::de::Unexpected::Signed(v) => serde_de::Unexpected::Signed(v),
        serde::de::Unexpected::Float(v) => serde_de::Unexpected::Float(v),
        serde::de::Unexpected::Char(v) => serde_de::Unexpected::Char(v),
        serde::de::Unexpected::Str(v) => serde_de::Unexpected::Str(String::from(v)),
        serde::de::Unexpected::Bytes(v) => serde_de::Unexpected::Bytes(Vec::from(v)),
        serde::de::Unexpected::Unit => serde_de::Unexpected::Unit,
        serde::de::Unexpected::Option => serde_de::Unexpected::Option,
        serde::de::Unexpected::NewtypeStruct => serde_de::Unexpected::NewtypeStruct,
        serde::de::Unexpected::Seq => serde_de::Unexpected::Seq,
        serde::de::Unexpected::Map => serde_de::Unexpected::Map,
        serde::de::Unexpected::Enum => serde_de::Unexpected::Enum,
        serde::de::Unexpected::UnitVariant => serde_de::Unexpected::UnitVariant,
        serde::de::Unexpected::NewtypeVariant => serde_de::Unexpected::NewtypeVariant,
        serde::de::Unexpected::TupleVariant => serde_de::Unexpected::TupleVariant,
        serde::de::Unexpected::StructVariant => serde_de::Unexpected::StructVariant,
        serde::de::Unexpected::Other(v) => serde_de::Unexpected::Other(String::from(v)),
    }
}

impl serde::de::Error for DeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::custom(&format!("{msg}"))
    }

    fn invalid_type(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        Self::invalid_type(translate_serde_de_unexpected(unexp), &format!("{exp}"))
    }

    fn invalid_value(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        Self::invalid_value(translate_serde_de_unexpected(unexp), &format!("{exp}"))
    }

    fn invalid_length(len: usize, exp: &dyn serde::de::Expected) -> Self {
        Self::invalid_length(len, &format!("{exp}"))
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self::unknown_variant(variant, expected)
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::unknown_field(field, expected)
    }

    fn missing_field(field: &'static str) -> Self {
        Self::missing_field(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self::duplicate_field(field)
    }
}

impl DeError {
    fn display(&self) -> String {
        todo!("wit-bindgen")
    }

    fn debug(&self) -> String {
        todo!("wit-bindgen")
    }

    fn custom(_msg: &str) -> Self {
        todo!("wit-bindgen")
    }

    fn invalid_type(_unexp: serde_de::Unexpected, _exp: &str) -> Self {
        todo!("wit-bindgen")
    }

    fn invalid_value(_unexp: serde_de::Unexpected, _exp: &str) -> Self {
        todo!("wit-bindgen")
    }

    fn invalid_length(_len: usize, _exp: &str) -> Self {
        todo!("wit-bindgen")
    }

    fn unknown_variant(_variant: &str, _expected: &[&str]) -> Self {
        todo!("wit-bindgen")
    }

    fn unknown_field(_field: &str, _expected: &[&str]) -> Self {
        todo!("wit-bindgen")
    }

    fn missing_field(_field: &str) -> Self {
        todo!("wit-bindgen")
    }

    fn duplicate_field(_field: &str) -> Self {
        todo!("wit-bindgen")
    }
}

impl serde::de::StdError for DeError {}

impl fmt::Debug for DeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.debug())
    }
}

impl fmt::Display for DeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.display())
    }
}

struct Deserializer {
    _private: (),
}
