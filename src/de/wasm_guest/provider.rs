use core::fmt;

use scoped_reference::{ScopedBorrowMut, ScopedReference};
use serde::serde_if_integer128;

wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-provider", no_std });
export_serde_deserializer_provider!(GuestsideDeserializerProvider);

use crate::{
    any::Any,
    intern::{intern_str_list, intern_string},
};

pub struct GuestsideDeserializerProvider {
    deserializer: Box<dyn ErasedDeserializer>,
    scope: ScopedBorrowMut<()>,
}

impl deserializer::Deserializer for GuestsideDeserializerProvider {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> Result<(serde_types::U128, serde_types::Usize), serde_de::Unexpected> {
        deserialize::test(x, y)
    }
}

impl GuestsideDeserializerProvider {
    #[must_use]
    pub fn with_new<'a, 'de, D: serde::de::Deserializer<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        deserializer: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let deserializer: Box<dyn ErasedDeserializer + 'a> = Box::new(deserializer);
            let deserializer: Box<dyn ErasedDeserializer + 'static> =
                unsafe { core::mem::transmute(deserializer) };

            inner(Self {
                deserializer,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    fn deserialize_any(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_any(VisitableVisitor { visitor })
    }

    fn deserialize_bool(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_bool(VisitableVisitor { visitor })
    }

    fn deserialize_i8(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_i8(VisitableVisitor { visitor })
    }

    fn deserialize_i16(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_i16(VisitableVisitor { visitor })
    }

    fn deserialize_i32(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_i32(VisitableVisitor { visitor })
    }

    fn deserialize_i64(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_i64(VisitableVisitor { visitor })
    }

    fn deserialize_i128(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_i128(VisitableVisitor { visitor })
    }

    fn deserialize_u8(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_u8(VisitableVisitor { visitor })
    }

    fn deserialize_u16(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_u16(VisitableVisitor { visitor })
    }

    fn deserialize_u32(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_u32(VisitableVisitor { visitor })
    }

    fn deserialize_u64(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_u64(VisitableVisitor { visitor })
    }

    fn deserialize_u128(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_u128(VisitableVisitor { visitor })
    }

    fn deserialize_f32(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_f32(VisitableVisitor { visitor })
    }

    fn deserialize_f64(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_f64(VisitableVisitor { visitor })
    }

    fn deserialize_char(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_char(VisitableVisitor { visitor })
    }

    fn deserialize_str(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_str(VisitableVisitor { visitor })
    }

    fn deserialize_string(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_string(VisitableVisitor { visitor })
    }

    fn deserialize_bytes(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_bytes(VisitableVisitor { visitor })
    }

    fn deserialize_byte_buf(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_byte_buf(VisitableVisitor { visitor })
    }

    fn deserialize_option(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_option(VisitableVisitor { visitor })
    }

    fn deserialize_unit(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_unit(VisitableVisitor { visitor })
    }

    fn deserialize_unit_struct(self, name: &str, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer.erased_deserialize_unit_struct(
            intern_string(String::from(name)),
            VisitableVisitor { visitor },
        )
    }

    fn deserialize_newtype_struct(self, name: &str, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer.erased_deserialize_newtype_struct(
            intern_string(String::from(name)),
            VisitableVisitor { visitor },
        )
    }

    fn deserialize_seq(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_seq(VisitableVisitor { visitor })
    }

    fn deserialize_tuple(self, len: usize, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_tuple(len, VisitableVisitor { visitor })
    }

    fn deserialize_tuple_struct(
        self,
        name: &str,
        len: usize,
        visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        self.deserializer.erased_deserialize_tuple_struct(
            intern_string(String::from(name)),
            len,
            VisitableVisitor { visitor },
        )
    }

    fn deserialize_map(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_map(VisitableVisitor { visitor })
    }

    fn deserialize_struct(
        self,
        name: &str,
        fields: &[&str],
        visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        let fields = fields
            .iter()
            .map(|f| intern_string(String::from(*f)))
            .collect();

        self.deserializer.erased_deserialize_struct(
            intern_string(String::from(name)),
            intern_str_list(fields),
            VisitableVisitor { visitor },
        )
    }

    fn deserialize_enum(
        self,
        name: &str,
        variants: &[&str],
        visitor: Visitor,
    ) -> Result<DeValue, DeError> {
        let variants = variants
            .iter()
            .map(|v| intern_string(String::from(*v)))
            .collect();

        self.deserializer.erased_deserialize_enum(
            intern_string(String::from(name)),
            intern_str_list(variants),
            VisitableVisitor { visitor },
        )
    }

    fn deserialize_identifier(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_identifier(VisitableVisitor { visitor })
    }

    fn deserialize_ignored_any(self, visitor: Visitor) -> Result<DeValue, DeError> {
        self.deserializer
            .erased_deserialize_ignored_any(VisitableVisitor { visitor })
    }

    fn is_human_readable(&self) -> bool {
        self.deserializer.erased_is_human_readable()
    }
}

trait ErasedDeserializer {
    fn erased_deserialize_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_bool(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    serde_if_integer128! {
        fn erased_deserialize_u128(self: Box<Self>, visitor: VisitableVisitor) -> Result<DeValue, DeError>;
    }
    fn erased_deserialize_i8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_i16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_i32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_i64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    serde_if_integer128! {
        fn erased_deserialize_i128(self: Box<Self>, visitor: VisitableVisitor) -> Result<DeValue, DeError>;
    }
    fn erased_deserialize_f32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_f64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_char(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_str(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_string(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_bytes(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_byte_buf(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_option(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_unit(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_unit_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_newtype_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_seq(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_tuple(
        self: Box<Self>,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_tuple_struct(
        self: Box<Self>,
        name: &'static str,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_map(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_struct(
        self: Box<Self>,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_identifier(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_enum(
        self: Box<Self>,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_ignored_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_is_human_readable(&self) -> bool;
}

impl<'de, T: serde::de::Deserializer<'de>> ErasedDeserializer for T {
    fn erased_deserialize_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_any(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_bool(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_bool(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u8(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u16(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u32(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u64(visitor).map_err(DeError::wrap)
    }

    serde_if_integer128! {
        fn erased_deserialize_u128(self:Box<Self>,visitor:VisitableVisitor) -> Result<DeValue,DeError> {
            self.deserialize_u128(visitor).map_err(DeError::wrap)
        }
    }

    fn erased_deserialize_i8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i8(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_i16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i16(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_i32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i32(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_i64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i64(visitor).map_err(DeError::wrap)
    }

    serde_if_integer128! {
        fn erased_deserialize_i128(self:Box<Self>,visitor:VisitableVisitor) -> Result<DeValue,DeError> {
            self.deserialize_i128(visitor).map_err(DeError::wrap)
        }
    }

    fn erased_deserialize_f32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_f32(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_f64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_f64(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_char(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_char(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_str(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_str(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_string(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_string(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_bytes(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_bytes(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_byte_buf(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_byte_buf(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_option(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_option(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_unit(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_unit(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_unit_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_unit_struct(name, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_newtype_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_newtype_struct(name, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_seq(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_seq(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_tuple(
        self: Box<Self>,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_tuple(len, visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_tuple_struct(
        self: Box<Self>,
        name: &'static str,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_tuple_struct(name, len, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_map(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_map(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_struct(
        self: Box<Self>,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_struct(name, fields, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_identifier(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_identifier(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_enum(
        self: Box<Self>,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_enum(name, variants, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_ignored_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_ignored_any(visitor).map_err(DeError::wrap)
    }

    fn erased_is_human_readable(&self) -> bool {
        self.is_human_readable()
    }
}

struct Visitor {
    _private: (),
}

impl Visitor {
    fn expecting(&self) -> String {
        todo!("wit-bindgen")
    }

    fn visit_bool(self, _v: bool) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_i8(self, _v: i8) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_i16(self, _v: i16) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_i32(self, _v: i32) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_i64(self, _v: i64) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    serde_if_integer128! {
        fn visit_i128(self, _v: i128) -> Result<DeValue, DeError> {
            todo!("wit-bindgen")
        }
    }

    fn visit_u8(self, _v: u8) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_u16(self, _v: u16) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_u32(self, _v: u32) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_u64(self, _v: u64) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    serde_if_integer128! {
        fn visit_u128(self, _v: u128) -> Result<DeValue, DeError> {
            todo!("wit-bindgen")
        }
    }

    fn visit_f32(self, _v: f32) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_f64(self, _v: f64) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_char(self, _v: char) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_string(self, _v: String) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_byte_buf(self, _v: Vec<u8>) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_none(self) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_some(self, _deserializer: GuestsideDeserializerProvider) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_unit(self) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_newtype_struct(
        self,
        _deserializer: GuestsideDeserializerProvider,
    ) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_seq(self, _seq: SeqAccess) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_map(self, _map: MapAccess) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }

    fn visit_enum(self, _data: EnumAccess) -> Result<DeValue, DeError> {
        todo!("wit-bindgen")
    }
}

struct SeqAccess {
    // TOOD
}

struct MapAccess {
    // TOOD
}

struct EnumAccess {
    // TOOD
}

struct VisitableVisitor {
    visitor: Visitor,
}

fn unwrap_de_error<E: serde::de::Error>(result: Result<DeValue, DeError>) -> Result<DeValue, E> {
    match result {
        Ok(value) => Ok(value),
        Err(err) => match err.take() {
            Some(err) => Err(err),
            None => Err(serde::de::Error::custom(
                "bug: type mismatch across the wit boundary",
            )),
        },
    }
}

impl<'de> serde::de::Visitor<'de> for VisitableVisitor {
    type Value = DeValue;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.visitor.expecting())
    }

    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_bool(v))
    }

    fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_i8(v))
    }

    fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_i16(v))
    }

    fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_i32(v))
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_i64(v))
    }

    serde_if_integer128! {
        fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
            unwrap_de_error(self.visitor.visit_i128(v))
        }
    }

    fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_u8(v))
    }

    fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_u16(v))
    }

    fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_u32(v))
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_u64(v))
    }

    serde_if_integer128! {
        fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
            unwrap_de_error(self.visitor.visit_u128(v))
        }
    }

    fn visit_f32<E: serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_f32(v))
    }

    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_f64(v))
    }

    fn visit_char<E: serde::de::Error>(self, v: char) -> Result<Self::Value, E> {
        unwrap_de_error(self.visitor.visit_char(v))
    }

    //     fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Str(v), &self))
    //     }

    //     fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         self.visit_str(v)
    //     }

    //     fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         self.visit_str(&v)
    //     }

    //     fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         let _ = v;
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Bytes(v), &self))
    //     }

    //     fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         self.visit_bytes(v)
    //     }

    //     fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         self.visit_bytes(&v)
    //     }

    //     fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Option, &self))
    //     }

    //     fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    //     where
    //         D: serde::Deserializer<'de>,
    //     {
    //         let _ = deserializer;
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Option, &self))
    //     }

    //     fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error,
    //     {
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Unit, &self))
    //     }

    //     fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    //     where
    //         D: serde::Deserializer<'de>,
    //     {
    //         let _ = deserializer;
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::NewtypeStruct, &self))
    //     }

    //     fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    //     where
    //         A: serde::de::SeqAccess<'de>,
    //     {
    //         let _ = seq;
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))
    //     }

    //     fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    //     where
    //         A: serde::de::MapAccess<'de>,
    //     {
    //         let _ = map;
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Map, &self))
    //     }

    //     fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    //     where
    //         A: serde::de::EnumAccess<'de>,
    //     {
    //         let _ = data;
    //         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Enum, &self))
    //     }

    //     fn __private_visit_untagged_option<D>(self, _: D) -> Result<Self::Value, ()>
    //     where
    //         D: serde::Deserializer<'de>,
    //     {
    //         Err(())
    //     }
}

struct DeValue {
    _private: (),
}

struct DeError {
    inner: DeErrorVariants,
}

enum DeErrorVariants {
    Error {
        err: Any,
        display: String,
        debug: String,
    },
    Custom(String),
    InvalidType {
        unexp: serde_de::Unexpected,
        exp: String,
    },
    InvalidValue {
        unexp: serde_de::Unexpected,
        exp: String,
    },
    InvalidLength {
        len: usize,
        exp: String,
    },
    UnknownVariant {
        variant: String,
        expected: &'static [&'static str],
    },
    UnknownField {
        field: String,
        expected: &'static [&'static str],
    },
    MissingField {
        field: &'static str,
    },
    DuplicateField {
        field: &'static str,
    },
}

fn translate_serde_de_unexpected(unexp: &serde_de::Unexpected) -> serde::de::Unexpected {
    match unexp {
        serde_de::Unexpected::Bool(v) => serde::de::Unexpected::Bool(*v),
        serde_de::Unexpected::Unsigned(v) => serde::de::Unexpected::Unsigned(*v),
        serde_de::Unexpected::Signed(v) => serde::de::Unexpected::Signed(*v),
        serde_de::Unexpected::Float(v) => serde::de::Unexpected::Float(*v),
        serde_de::Unexpected::Char(v) => serde::de::Unexpected::Char(*v),
        serde_de::Unexpected::Str(v) => serde::de::Unexpected::Str(v),
        serde_de::Unexpected::Bytes(v) => serde::de::Unexpected::Bytes(v),
        serde_de::Unexpected::Unit => serde::de::Unexpected::Unit,
        serde_de::Unexpected::Option => serde::de::Unexpected::Option,
        serde_de::Unexpected::NewtypeStruct => serde::de::Unexpected::NewtypeStruct,
        serde_de::Unexpected::Seq => serde::de::Unexpected::Seq,
        serde_de::Unexpected::Map => serde::de::Unexpected::Map,
        serde_de::Unexpected::Enum => serde::de::Unexpected::Enum,
        serde_de::Unexpected::UnitVariant => serde::de::Unexpected::UnitVariant,
        serde_de::Unexpected::NewtypeVariant => serde::de::Unexpected::NewtypeVariant,
        serde_de::Unexpected::TupleVariant => serde::de::Unexpected::TupleVariant,
        serde_de::Unexpected::StructVariant => serde::de::Unexpected::StructVariant,
        serde_de::Unexpected::Other(v) => serde::de::Unexpected::Other(v),
    }
}

impl DeError {
    fn wrap<T: serde::de::Error>(err: T) -> Self {
        let display = format!("{err}");
        let debug = format!("{err:?}");

        // Safety: TODO
        Self {
            inner: DeErrorVariants::Error {
                err: unsafe { Any::new(err) },
                display,
                debug,
            },
        }
    }

    fn take<T: serde::de::Error>(self) -> Option<T> {
        match self.inner {
            // Safety: TODO
            DeErrorVariants::Error { err, .. } => unsafe { err.take() },
            DeErrorVariants::Custom(msg) => Some(serde::de::Error::custom(msg)),
            DeErrorVariants::InvalidType { unexp, exp } => Some(serde::de::Error::invalid_type(
                translate_serde_de_unexpected(&unexp),
                &&*exp,
            )),
            DeErrorVariants::InvalidValue { unexp, exp } => Some(serde::de::Error::invalid_value(
                translate_serde_de_unexpected(&unexp),
                &&*exp,
            )),
            DeErrorVariants::InvalidLength { len, exp } => {
                Some(serde::de::Error::invalid_length(len, &&*exp))
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                Some(serde::de::Error::unknown_variant(&variant, expected))
            }
            DeErrorVariants::UnknownField { field, expected } => {
                Some(serde::de::Error::unknown_field(&field, expected))
            }
            DeErrorVariants::MissingField { field } => Some(serde::de::Error::missing_field(field)),
            DeErrorVariants::DuplicateField { field } => {
                Some(serde::de::Error::duplicate_field(field))
            }
        }
    }

    fn display(&self) -> String {
        match &self.inner {
            DeErrorVariants::Error { display, .. } => String::from(display),
            DeErrorVariants::Custom(msg) => String::from(msg),
            DeErrorVariants::InvalidType { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("invalid type: {unexp}, expected {exp}")
            }
            DeErrorVariants::InvalidValue { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("invalid value: {unexp}, expected {exp}")
            }
            DeErrorVariants::InvalidLength { len, exp } => {
                format!("invalid length {len}, expected {exp}")
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                format!(
                    "unknown variant `{variant}`, {}",
                    ExpectedOneOf {
                        expected,
                        kinds: "variants"
                    }
                )
            }
            DeErrorVariants::UnknownField { field, expected } => {
                format!(
                    "unknown field `{field}`, {}",
                    ExpectedOneOf {
                        expected,
                        kinds: "fields"
                    }
                )
            }
            DeErrorVariants::MissingField { field } => {
                format!("missing field `{field}`")
            }
            DeErrorVariants::DuplicateField { field } => {
                format!("duplicate field `{field}`")
            }
        }
    }

    fn debug(&self) -> String {
        match &self.inner {
            DeErrorVariants::Error { debug, .. } => {
                format!("serde_wit::de::Error {{ err: {debug} }}")
            }
            DeErrorVariants::Custom(msg) => {
                format!("serde_wit::de::Error {{ err: Custom({msg}) }}")
            }
            DeErrorVariants::InvalidType { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("serde_wit::de::Error {{ err: InvalidType {{ unexp: {unexp:?}, exp: {exp:?} }} }}")
            }
            DeErrorVariants::InvalidValue { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("serde_wit::de::Error {{ err: InvalidValue {{ unexp: {unexp:?}, exp: {exp:?} }} }}")
            }
            DeErrorVariants::InvalidLength { len, exp } => {
                format!(
                    "serde_wit::de::Error {{ err: InvalidLength {{ len: {len}, exp: {exp:?} }} }}"
                )
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                format!(
                    "serde_wit::de::Error {{ err: UnknownVariant {{ variant: {variant:?}, expected: {expected:?} }} }}"
                )
            }
            DeErrorVariants::UnknownField { field, expected } => {
                format!(
                    "serde_wit::de::Error {{ err: UnknownField {{ field: {field:?}, expected: {expected:?} }} }}"
                )
            }
            DeErrorVariants::MissingField { field } => {
                format!("serde_wit::de::Error {{ err: MissingField {{ field: {field:?} }} }}")
            }
            DeErrorVariants::DuplicateField { field } => {
                format!("serde_wit::de::Error {{ err: DuplicateField {{ field: {field:?} }} }}")
            }
        }
    }

    fn custom(msg: &str) -> Self {
        Self {
            inner: DeErrorVariants::Custom(String::from(msg)),
        }
    }

    fn invalid_type(unexp: serde_de::Unexpected, exp: &str) -> Self {
        Self {
            inner: DeErrorVariants::InvalidType {
                unexp,
                exp: String::from(exp),
            },
        }
    }

    fn invalid_value(unexp: serde_de::Unexpected, exp: &str) -> Self {
        Self {
            inner: DeErrorVariants::InvalidValue {
                unexp,
                exp: String::from(exp),
            },
        }
    }

    fn invalid_length(len: usize, exp: &str) -> Self {
        Self {
            inner: DeErrorVariants::InvalidLength {
                len,
                exp: String::from(exp),
            },
        }
    }

    fn unknown_variant(variant: &str, expected: &[&str]) -> Self {
        let expected = expected
            .iter()
            .map(|e| intern_string(String::from(*e)))
            .collect();

        Self {
            inner: DeErrorVariants::UnknownVariant {
                variant: String::from(variant),
                expected: intern_str_list(expected),
            },
        }
    }

    fn unknown_field(field: &str, expected: &[&str]) -> Self {
        let expected = expected
            .iter()
            .map(|e| intern_string(String::from(*e)))
            .collect();

        Self {
            inner: DeErrorVariants::UnknownField {
                field: String::from(field),
                expected: intern_str_list(expected),
            },
        }
    }

    fn missing_field(field: &str) -> Self {
        Self {
            inner: DeErrorVariants::MissingField {
                field: intern_string(String::from(field)),
            },
        }
    }

    fn duplicate_field(field: &str) -> Self {
        Self {
            inner: DeErrorVariants::DuplicateField {
                field: intern_string(String::from(field)),
            },
        }
    }
}

struct ExpectedOneOf {
    expected: &'static [&'static str],
    kinds: &'static str,
}

impl fmt::Display for ExpectedOneOf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.expected {
            [] => write!(fmt, "there are no {}", self.kinds),
            [one] => write!(fmt, "expected `{one}`"),
            [one, two] => write!(fmt, "expected `{one}` or `{two}`"),
            _ => {
                write!(fmt, "expected one of ")?;
                for (i, alt) in self.expected.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, ", ")?;
                    }
                    write!(fmt, "`{alt}`")?;
                }
                Ok(())
            }
        }
    }
}
