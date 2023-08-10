use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::{cell::RefCell, fmt};

use ::serde::serde_if_integer128;
use scoped_reference::{ScopedBorrowMut, ScopedReference};

wit_bindgen::generate!({ world: "serde-deserializer-client", exports: {
    "serde:serde/serde-deserialize/deserialize-seed": GuestsideDeserializerClient,
    "serde:serde/serde-deserialize/de-value": DeValue,
    "serde:serde/serde-deserialize/visitor": Visitor,
} });

use crate::any::Any;

pub struct GuestsideDeserializerClient {
    deserialize_seed: RefCell<Option<Box<dyn ErasedDeserializeSeed>>>,
    _scope: ScopedBorrowMut<()>,
}

impl GuestsideDeserializerClient {
    #[must_use]
    pub fn with_new<'a, D: for<'de> ::serde::de::Deserialize<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        inner: F,
    ) -> Q {
        Self::with_new_seed(core::marker::PhantomData::<D>, inner)
    }

    #[must_use]
    pub fn with_new_seed<
        'a,
        D: for<'de> ::serde::de::DeserializeSeed<'de> + 'a,
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
                deserialize_seed: RefCell::new(Some(deserialize_seed)),
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    unsafe fn with_new_seed_unchecked<
        'a,
        'de,
        D: ::serde::de::DeserializeSeed<'de> + 'a,
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
                deserialize_seed: RefCell::new(Some(deserialize_seed)),
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }
}

impl self::exports::serde::serde::serde_deserialize::DeserializeSeed
    for GuestsideDeserializerClient
{
    fn deserialize(
        this: self::exports::serde::serde::serde_deserialize::OwnDeserializeSeed,
        deserializer: self::exports::serde::serde::serde_deserialize::OwnedDeserializerHandle,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Ok(mut deserialize_seed) = this.deserialize_seed.try_borrow_mut() else {
            let error = <DeError as ::serde::de::Error>::custom("bug: could not mutably borrow the owned DeserializeSeed in DeserializeSeed::deserialize");
            return Err(error.into_handle());
        };
        let Some(deserialize_seed) = deserialize_seed.take() else {
            let error = <DeError as ::serde::de::Error>::custom("bug: DeserializeSeed::deserialize after free");
            return Err(error.into_handle());
        };

        // TODO: Safety
        let deserializer = unsafe {
            self::serde::serde::serde_deserializer::Deserializer::from_handle(
                deserializer.owned_handle,
                true,
            )
        };

        ErasedDeserializeSeed::erased_deserialize(
            deserialize_seed,
            DeserializerableDeserializer { deserializer },
        )
        .wrap()
    }
}

trait WrapDeResult {
    type Ok;

    fn wrap(
        self,
    ) -> Result<Self::Ok, self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle>;
}

impl WrapDeResult for Result<DeValue, DeError> {
    type Ok = self::exports::serde::serde::serde_deserialize::OwnDeValue;

    fn wrap(
        self,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        match self {
            Ok(ok) => Ok(self::exports::serde::serde::serde_deserialize::OwnDeValue::new(ok)),
            Err(error) => Err(error.into_handle()),
        }
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
    fn erased_visit_i128(self: Box<Self>, v: i128) -> Result<DeValue, DeError>;
    fn erased_visit_u128(self: Box<Self>, v: u128) -> Result<DeValue, DeError>;
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

impl<'de, T: ::serde::de::DeserializeSeed<'de>> ErasedDeserializeSeed for T {
    fn erased_deserialize(
        self: Box<Self>,
        deserializer: DeserializerableDeserializer,
    ) -> Result<DeValue, DeError> {
        self.deserialize(deserializer).map(DeValue::wrap)
    }
}

impl<'de, T: ::serde::de::Visitor<'de>> ErasedVisitor for T {
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

    fn erased_visit_i128(self: Box<Self>, v: i128) -> Result<DeValue, DeError> {
        self.visit_i128(v).map(DeValue::wrap)
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

    fn erased_visit_u128(self: Box<Self>, v: u128) -> Result<DeValue, DeError> {
        self.visit_u128(v).map(DeValue::wrap)
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
    deserializer: self::serde::serde::serde_deserializer::Deserializer,
}

fn unwrap_deserializer_result<V>(
    result: Result<
        self::serde::serde::serde_deserializer::OwnedDeValueHandle,
        self::serde::serde::serde_deserializer::DeError,
    >,
    value_ty: &'static str,
) -> Result<V, DeError> {
    match result {
        Ok(value) => {
            // TODO: Safety
            let value = unsafe {
                self::exports::serde::serde::serde_deserialize::OwnDeValue::from_handle(
                    value.owned_handle,
                )
            };

            let Ok(mut value) = value.value.try_borrow_mut() else {
                return Err(::serde::de::Error::custom(
                    format!("bug: could not mutably borrow the owned {value_ty} result"),
                ));
            };
            let Some(value) = value.take() else {
                return Err(::serde::de::Error::custom(
                    format!("bug: use of {value_ty} after free"),
                ));
            };
            // TODO: Safety
            let Some(value): Option<V> = (unsafe { value.take() }) else {
                return Err(::serde::de::Error::custom(
                    format!("bug: {value_ty} type mismatch across the wit boundary"),
                ))
            };
            Ok(value)
        }
        Err(error) => Err(DeError { error }),
    }
}

fn unwrap_deserializer_optional_result<V>(
    result: Result<
        Option<self::serde::serde::serde_deserializer::OwnedDeValueHandle>,
        self::serde::serde::serde_deserializer::DeError,
    >,
    value_ty: &'static str,
) -> Result<Option<V>, DeError> {
    let non_optional_result = match result {
        Ok(None) => return Ok(None),
        Ok(Some(ok)) => Ok(ok),
        Err(err) => Err(err),
    };

    unwrap_deserializer_result(non_optional_result, value_ty).map(Some)
}

impl Visitor {
    fn with_new_old<
        'de,
        V: ::serde::de::Visitor<'de>,
        F: FnOnce(Self) -> Result<DeValue, DeError>,
    >(
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

            struct Expecting<'a> {
                visitor: &'a dyn ErasedVisitor,
            }

            impl<'a> fmt::Display for Expecting<'a> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.visitor.erased_expecting(f)
                }
            }

            let expecting = format!("{}", Expecting { visitor: &*visitor });

            inner(Self {
                visitor: RefCell::new(Some(visitor)),
                expecting,
                _scope: scope.borrow_mut(),
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

        Err(::serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn with_new<
        'de,
        V: ::serde::de::Visitor<'de>,
        F: FnOnce(
            Self,
        ) -> Result<
            self::serde::serde::serde_deserializer::OwnedDeValueHandle,
            self::serde::serde::serde_deserializer::DeError,
        >,
    >(
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

            struct Expecting<'a> {
                visitor: &'a dyn ErasedVisitor,
            }

            impl<'a> fmt::Display for Expecting<'a> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.visitor.erased_expecting(f)
                }
            }

            let expecting = format!("{}", Expecting { visitor: &*visitor });

            inner(Self {
                visitor: RefCell::new(Some(visitor)),
                expecting,
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        unwrap_deserializer_result(result, "Visitor::Value")
    }

    fn try_extract_visitor(
        &self,
        method: &'static str,
    ) -> Result<
        Box<dyn ErasedVisitor>,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Ok(mut visitor) = self.visitor.try_borrow_mut() else {
            let error = <DeError as ::serde::de::Error>::custom(format!("bug: could not mutably borrow the owned Visitor in Visitor::{method}"));
            return Err(error.into_handle());
        };
        let Some(visitor) = visitor.take() else {
            let error = <DeError as ::serde::de::Error>::custom(format!("bug: Visitor::{method} after free"));
            return Err(error.into_handle());
        };
        Ok(visitor)
    }

    /*fn visit_map(self, map: MapAccess) -> Result<DeValue, DeError> {
        self.visitor.erased_visit_map(DeserializerableMapAccess {
            map_access: Some(map),
        })
    }

    fn visit_enum(self, data: EnumAccess) -> Result<DeValue, DeError> {
        self.visitor
            .erased_visit_enum(DeserializerableEnumAccess { enum_access: data })
    }*/
}

impl self::exports::serde::serde::serde_deserialize::Visitor for Visitor {
    fn expecting(this: &Self) -> String {
        this.expecting.clone()
    }

    fn visit_bool(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: bool,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_bool")?
            .erased_visit_bool(v)
            .wrap()
    }

    fn visit_i8(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: i8,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_i8")?
            .erased_visit_i8(v)
            .wrap()
    }

    fn visit_i16(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: i16,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_i16")?
            .erased_visit_i16(v)
            .wrap()
    }

    fn visit_i32(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: i32,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_i32")?
            .erased_visit_i32(v)
            .wrap()
    }

    fn visit_i64(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: i64,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_i64")?
            .erased_visit_i64(v)
            .wrap()
    }

    fn visit_i128(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: self::serde::serde::serde_types::S128,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        this.try_extract_visitor("visit_i128")?
            .erased_visit_i128(i128::from_le_bytes(bytes))
            .wrap()
    }

    fn visit_u8(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: u8,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_u8")?
            .erased_visit_u8(v)
            .wrap()
    }

    fn visit_u16(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: u16,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_u16")?
            .erased_visit_u16(v)
            .wrap()
    }

    fn visit_u32(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: u32,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_u32")?
            .erased_visit_u32(v)
            .wrap()
    }

    fn visit_u64(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: u64,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_u64")?
            .erased_visit_u64(v)
            .wrap()
    }

    fn visit_u128(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: self::serde::serde::serde_types::U128,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        this.try_extract_visitor("visit_u128")?
            .erased_visit_u128(u128::from_le_bytes(bytes))
            .wrap()
    }

    fn visit_f32(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: f32,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_f32")?
            .erased_visit_f32(v)
            .wrap()
    }

    fn visit_f64(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: f64,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_f64")?
            .erased_visit_f64(v)
            .wrap()
    }

    fn visit_char(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: char,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_char")?
            .erased_visit_char(v)
            .wrap()
    }

    fn visit_string(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: String,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_string")?
            .erased_visit_string(v)
            .wrap()
    }

    fn visit_byte_buf(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        v: Vec<u8>,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_byte_buf")?
            .erased_visit_byte_buf(v)
            .wrap()
    }

    fn visit_none(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_none")?
            .erased_visit_none()
            .wrap()
    }

    fn visit_some(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        deserializer: self::exports::serde::serde::serde_deserialize::OwnedDeserializerHandle,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let deserializer = unsafe {
            self::serde::serde::serde_deserializer::Deserializer::from_handle(
                deserializer.owned_handle,
                true,
            )
        };

        this.try_extract_visitor("visit_some")?
            .erased_visit_some(DeserializerableDeserializer { deserializer })
            .wrap()
    }

    fn visit_unit(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        this.try_extract_visitor("visit_unit")?
            .erased_visit_unit()
            .wrap()
    }

    fn visit_newtype_struct(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        deserializer: self::exports::serde::serde::serde_deserialize::OwnedDeserializerHandle,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let deserializer = unsafe {
            self::serde::serde::serde_deserializer::Deserializer::from_handle(
                deserializer.owned_handle,
                true,
            )
        };

        this.try_extract_visitor("visit_newtype_struct")?
            .erased_visit_newtype_struct(DeserializerableDeserializer { deserializer })
            .wrap()
    }

    fn visit_seq(
        this: self::exports::serde::serde::serde_deserialize::OwnVisitor,
        seq: self::exports::serde::serde::serde_deserialize::OwnedSeqAccessHandle,
    ) -> Result<
        self::exports::serde::serde::serde_deserialize::OwnDeValue,
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let seq = unsafe {
            self::serde::serde::serde_deserializer::SeqAccess::from_handle(seq.owned_handle, true)
        };

        this.try_extract_visitor("visit_seq")?
            .erased_visit_seq(DeserializerableSeqAccess {
                seq_access: Some(seq),
            })
            .wrap()
    }
}

impl<'de> ::serde::Deserializer<'de> for DeserializerableDeserializer {
    type Error = DeError;

    fn deserialize_any<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_any(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_bool<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_bool(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_i8<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_i8(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_i16<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_i16(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_i32<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_i32(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_i64<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_i64(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    serde_if_integer128! {
        fn deserialize_i128<V: ::serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            Visitor::with_new(visitor, |visitor| {
                let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

                self::serde::serde::serde_deserializer::Deserializer::deserialize_i128(
                    self.deserializer,
                    self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                        owned_handle: visitor.into_handle(),
                    },
                )
            })
        }
    }

    fn deserialize_u8<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_u8(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_u16<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_u16(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_u32<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_u32(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_u64<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_u64(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    serde_if_integer128! {
        fn deserialize_u128<V: ::serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            Visitor::with_new(visitor, |visitor| {
                let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

                self::serde::serde::serde_deserializer::Deserializer::deserialize_u128(
                    self.deserializer,
                    self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                        owned_handle: visitor.into_handle(),
                    },
                )
            })
        }
    }

    fn deserialize_f32<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_f32(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_f64<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_f64(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_char<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_char(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_str<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_str(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_string<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_string(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_bytes<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_bytes(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_byte_buf<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_byte_buf(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_option<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_option(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_unit<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_unit(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_unit_struct<V: ::serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_unit_struct(
                self.deserializer,
                name,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_newtype_struct<V: ::serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_newtype_struct(
                self.deserializer,
                name,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_seq<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_seq(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_tuple<V: ::serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let len = match u32::try_from(len) {
            Ok(len) => self::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Err(::serde::de::Error::custom(
                    "Deserializer::deserialize_tuple len exceeds u32",
                ));
            }
        };

        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_tuple(
                self.deserializer,
                len,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_tuple_struct<V: ::serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let len = match u32::try_from(len) {
            Ok(len) => self::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Err(::serde::de::Error::custom(
                    "Deserializer::deserialize_tuple_struct len exceeds u32",
                ));
            }
        };

        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_tuple_struct(
                self.deserializer,
                name,
                len,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_map<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_map(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_struct<V: ::serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let fields = fields.iter().copied().map(String::from).collect::<Vec<_>>();

        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_struct(
                self.deserializer,
                name,
                &fields,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_enum<V: ::serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let variants = variants
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<_>>();

        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_enum(
                self.deserializer,
                name,
                &variants,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_identifier<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_identifier(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn deserialize_ignored_any<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = self::exports::serde::serde::serde_deserialize::OwnVisitor::new(visitor);

            self::serde::serde::serde_deserializer::Deserializer::deserialize_ignored_any(
                self.deserializer,
                self::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: visitor.into_handle(),
                },
            )
        })
    }

    fn is_human_readable(&self) -> bool {
        self::serde::serde::serde_deserializer::Deserializer::is_human_readable(&self.deserializer)
    }
}

pub struct Visitor {
    visitor: RefCell<Option<Box<dyn ErasedVisitor>>>,
    expecting: String,
    _scope: ScopedBorrowMut<()>,
}

struct DeserializerableSeqAccess {
    seq_access: Option<self::serde::serde::serde_deserializer::SeqAccess>,
}

impl<'de> ::serde::de::SeqAccess<'de> for DeserializerableSeqAccess {
    type Error = DeError;

    fn next_element_seed<T: ::serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        let Some(seq_access) = self.seq_access.take() else {
            return Err(::serde::de::Error::custom("bug: SeqAccess::next_element_seed after free"));
        };

        let (seq_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                let seed =
                    self::exports::serde::serde::serde_deserialize::OwnDeserializeSeed::new(seed);
                self::serde::serde::serde_deserializer::SeqAccess::next_element_seed(
                    seq_access,
                    self::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: seed.into_handle(),
                    },
                )
            })
        };
        self.seq_access = Some(seq_access);

        unwrap_deserializer_optional_result(result, "SeqAccess::<T>::Value")
    }

    fn size_hint(&self) -> Option<usize> {
        self.seq_access
            .as_ref()
            .and_then(self::serde::serde::serde_deserializer::SeqAccess::size_hint)
            .map(|size_hint| size_hint.val as usize)
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

impl<'de> ::serde::de::MapAccess<'de> for DeserializerableMapAccess {
    type Error = DeError;

    fn next_key_seed<K: ::serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(::serde::de::Error::custom("bug: MapAccess::next_key_seed after free"));
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

        Err(::serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn next_value_seed<V: ::serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(::serde::de::Error::custom("bug: MapAccess::next_key_seed after free"));
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

        Err(::serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn next_entry_seed<
        K: ::serde::de::DeserializeSeed<'de>,
        V: ::serde::de::DeserializeSeed<'de>,
    >(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(::serde::de::Error::custom("bug: MapAccess::next_entry_seed after free"));
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

        Err(::serde::de::Error::custom(
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

impl<'de> ::serde::de::EnumAccess<'de> for DeserializerableEnumAccess {
    type Error = DeError;
    type Variant = DeserializerableVariantAccess;

    fn variant_seed<V: ::serde::de::DeserializeSeed<'de>>(
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

        Err(::serde::de::Error::custom(
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

impl<'de> ::serde::de::VariantAccess<'de> for DeserializerableVariantAccess {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.variant_access.unit_variant()
    }

    fn newtype_variant_seed<T: ::serde::de::DeserializeSeed<'de>>(
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

        Err(::serde::de::Error::custom(
            "bug: type mismatch across the wit boundary",
        ))
    }

    fn tuple_variant<V: ::serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new_old(visitor, |visitor| {
            self.variant_access.tuple_variant(len, visitor)
        })
    }

    fn struct_variant<V: ::serde::de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new_old(visitor, |visitor| {
            self.variant_access.struct_variant(fields, visitor)
        })
    }
}

pub struct DeValue {
    value: RefCell<Option<Any>>,
}

impl DeValue {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: RefCell::new(Some(unsafe { Any::new(value) })),
        }
    }

    fn take<T>(self) -> Option<T> {
        // Safety: TODO
        unsafe { self.value.into_inner().unwrap().take() }
    }
}

impl self::exports::serde::serde::serde_deserialize::DeValue for DeValue {
    fn id(
        this: self::exports::serde::serde::serde_deserialize::OwnDeValue,
    ) -> self::exports::serde::serde::serde_deserialize::OwnDeValue {
        this
    }
}

struct DeError {
    error: self::serde::serde::serde_deserializer::DeError,
}

impl DeError {
    fn into_handle(self) -> self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle {
        self::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle {
            owned_handle: self.error.into_handle(),
        }
    }
}

fn translate_serde_de_unexpected(
    unexp: ::serde::de::Unexpected,
) -> self::serde::serde::serde_deserializer::Unexpected {
    match unexp {
        ::serde::de::Unexpected::Bool(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Bool(v)
        }
        ::serde::de::Unexpected::Unsigned(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Unsigned(v)
        }
        ::serde::de::Unexpected::Signed(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Signed(v)
        }
        ::serde::de::Unexpected::Float(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Float(v)
        }
        ::serde::de::Unexpected::Char(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Char(v)
        }
        ::serde::de::Unexpected::Str(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Str(String::from(v))
        }
        ::serde::de::Unexpected::Bytes(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Bytes(Vec::from(v))
        }
        ::serde::de::Unexpected::Unit => self::serde::serde::serde_deserializer::Unexpected::Unit,
        ::serde::de::Unexpected::Option => {
            self::serde::serde::serde_deserializer::Unexpected::Option
        }
        ::serde::de::Unexpected::NewtypeStruct => {
            self::serde::serde::serde_deserializer::Unexpected::NewtypeStruct
        }
        ::serde::de::Unexpected::Seq => self::serde::serde::serde_deserializer::Unexpected::Seq,
        ::serde::de::Unexpected::Map => self::serde::serde::serde_deserializer::Unexpected::Map,
        ::serde::de::Unexpected::Enum => self::serde::serde::serde_deserializer::Unexpected::Enum,
        ::serde::de::Unexpected::UnitVariant => {
            self::serde::serde::serde_deserializer::Unexpected::UnitVariant
        }
        ::serde::de::Unexpected::NewtypeVariant => {
            self::serde::serde::serde_deserializer::Unexpected::NewtypeVariant
        }
        ::serde::de::Unexpected::TupleVariant => {
            self::serde::serde::serde_deserializer::Unexpected::TupleVariant
        }
        ::serde::de::Unexpected::StructVariant => {
            self::serde::serde::serde_deserializer::Unexpected::StructVariant
        }
        ::serde::de::Unexpected::Other(v) => {
            self::serde::serde::serde_deserializer::Unexpected::Other(String::from(v))
        }
    }
}

impl ::serde::de::Error for DeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            error: self::serde::serde::serde_deserializer::DeError::custom(&format!("{msg}")),
        }
    }

    fn invalid_type(unexp: ::serde::de::Unexpected, exp: &dyn ::serde::de::Expected) -> Self {
        Self {
            error: self::serde::serde::serde_deserializer::DeError::invalid_type(
                &translate_serde_de_unexpected(unexp),
                &format!("{exp}"),
            ),
        }
    }

    fn invalid_value(unexp: ::serde::de::Unexpected, exp: &dyn ::serde::de::Expected) -> Self {
        Self {
            error: self::serde::serde::serde_deserializer::DeError::invalid_value(
                &translate_serde_de_unexpected(unexp),
                &format!("{exp}"),
            ),
        }
    }

    fn invalid_length(len: usize, exp: &dyn ::serde::de::Expected) -> Self {
        let len = match u32::try_from(len) {
            Ok(len) => self::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Self::custom("Deserializer::Error::invalid_length len exceeds u32");
            }
        };

        Self {
            error: self::serde::serde::serde_deserializer::DeError::invalid_length(
                len,
                &format!("{exp}"),
            ),
        }
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        let expected = expected
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<_>>();
        Self {
            error: self::serde::serde::serde_deserializer::DeError::unknown_variant(
                variant, &expected,
            ),
        }
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        let expected = expected
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<_>>();
        Self {
            error: self::serde::serde::serde_deserializer::DeError::unknown_field(field, &expected),
        }
    }

    fn missing_field(field: &'static str) -> Self {
        Self {
            error: self::serde::serde::serde_deserializer::DeError::missing_field(field),
        }
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self {
            error: self::serde::serde::serde_deserializer::DeError::duplicate_field(field),
        }
    }
}

impl ::serde::de::StdError for DeError {}

impl fmt::Debug for DeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self::serde::serde::serde_deserializer::DeError::debug(
            &self.error,
        ))
    }
}

impl fmt::Display for DeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self::serde::serde::serde_deserializer::DeError::display(
            &self.error,
        ))
    }
}
