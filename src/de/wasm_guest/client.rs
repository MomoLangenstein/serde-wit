use alloc::{boxed::Box, vec::Vec};
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
        d: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError>;
    fn erased_visit_unit(self: Box<Self>) -> Result<DeValue, DeError>;
    fn erased_visit_newtype_struct(
        self: Box<Self>,
        d: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError>;
    // fn erased_visit_seq(self: Box<Self>, s: &mut dyn SeqAccess<'de>) -> Result<DeValue, DeError>;
    // fn erased_visit_map(self: Box<Self>, m: &mut dyn MapAccess<'de>) -> Result<DeValue, DeError>;
    // fn erased_visit_enum(self: Box<Self>, e: &mut dyn EnumAccess<'de>) -> Result<DeValue, Error>;
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
        d: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError> {
        self.visit_some(d).map(DeValue::wrap)
    }

    fn erased_visit_unit(self: Box<Self>) -> Result<DeValue, DeError> {
        self.visit_unit().map(DeValue::wrap)
    }

    fn erased_visit_newtype_struct(
        self: Box<Self>,
        d: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError> {
        self.visit_newtype_struct(d).map(DeValue::wrap)
    }
}

struct DeserializerableDeserializer {
    deserializer: Deserializer,
}

fn unwrap_visitor_value<'de, V: serde::de::Visitor<'de>>(
    result: Result<DeValue, DeError>,
) -> Result<V::Value, DeError> {
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

unsafe fn erase_visitor_lifetime<'a>(
    visitor: Box<dyn ErasedVisitor + 'a>,
) -> Box<dyn ErasedVisitor + 'static> {
    core::mem::transmute(visitor)
}

fn with_wrapped_visitor<
    'de,
    V: serde::de::Visitor<'de>,
    F: FnOnce(Visitor) -> Result<DeValue, DeError>,
>(
    visitor: V,
    inner: F,
) -> Result<V::Value, DeError> {
    let visitor: Box<dyn ErasedVisitor + '_> = Box::new(visitor);
    let visitor: Box<dyn ErasedVisitor + 'static> = unsafe { erase_visitor_lifetime(visitor) };

    unwrap_visitor_value::<'de, V>(inner(Visitor { visitor }))
}

impl<'de> serde::Deserializer<'de> for DeserializerableDeserializer {
    type Error = DeError;

    fn deserialize_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_any(visitor)
        })
    }

    fn deserialize_bool<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_bool(visitor)
        })
    }

    fn deserialize_i8<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| self.deserializer.deserialize_i8(visitor))
    }

    fn deserialize_i16<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_i16(visitor)
        })
    }

    fn deserialize_i32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_i32(visitor)
        })
    }

    fn deserialize_i64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_i64(visitor)
        })
    }

    serde_if_integer128! {
        fn deserialize_i128<V: serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            with_wrapped_visitor(visitor, |visitor| self.deserializer.deserialize_i128(visitor))
        }
    }

    fn deserialize_u8<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| self.deserializer.deserialize_u8(visitor))
    }

    fn deserialize_u16<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_u16(visitor)
        })
    }

    fn deserialize_u32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_u32(visitor)
        })
    }

    fn deserialize_u64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_u64(visitor)
        })
    }

    serde_if_integer128! {
        fn deserialize_u128<V: serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            with_wrapped_visitor(visitor, |visitor| self.deserializer.deserialize_u128(visitor))
        }
    }

    fn deserialize_f32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_f32(visitor)
        })
    }

    fn deserialize_f64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_f64(visitor)
        })
    }

    fn deserialize_char<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_char(visitor)
        })
    }

    fn deserialize_str<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_str(visitor)
        })
    }

    fn deserialize_string<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_string(visitor)
        })
    }

    fn deserialize_bytes<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_bytes(visitor)
        })
    }

    fn deserialize_byte_buf<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_byte_buf(visitor)
        })
    }

    fn deserialize_option<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_option(visitor)
        })
    }

    fn deserialize_unit<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_unit(visitor)
        })
    }

    fn deserialize_unit_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_unit_struct(name, visitor)
        })
    }

    fn deserialize_newtype_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_newtype_struct(name, visitor)
        })
    }

    fn deserialize_seq<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_seq(visitor)
        })
    }

    fn deserialize_tuple<V: serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_tuple(len, visitor)
        })
    }

    fn deserialize_tuple_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer
                .deserialize_tuple_struct(name, len, visitor)
        })
    }

    fn deserialize_map<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_map(visitor)
        })
    }

    fn deserialize_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_struct(name, fields, visitor)
        })
    }

    fn deserialize_enum<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_enum(name, variants, visitor)
        })
    }

    fn deserialize_identifier<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
            self.deserializer.deserialize_identifier(visitor)
        })
    }

    fn deserialize_ignored_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_wrapped_visitor(visitor, |visitor| {
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

    fn deserialize_i128(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn deserialize_u128(self, _visitor: Visitor) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn is_human_readable(&self) -> bool {
        todo!("wit-bindgen")
    }
}

struct Visitor {
    visitor: Box<dyn ErasedVisitor>,
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
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        todo!() // fmt.write_str("DeError")
    }
}

impl fmt::Display for DeError {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        todo!() // fmt.write_str("DeError")
    }
}

struct Deserializer {
    _private: (),
}
