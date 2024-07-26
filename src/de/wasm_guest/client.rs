use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::fmt;

use ::serde::serde_if_integer128;
use scoped_reference::{ScopedBorrowMut, ScopedReference};

mod bindings {
    wit_bindgen::generate!({ world: "serde-deserializer-client", std_feature });
}

mod export {
    enum GuestsideDeserializerClient {}

    impl super::bindings::exports::serde::serde::serde_deserialize::Guest
        for GuestsideDeserializerClient
    {
        type DeserializeSeed = super::GuestsideDeserializerClient;
        type DeValue = super::DeValue;
        type Visitor = super::Visitor;
    }

    super::bindings::export!(GuestsideDeserializerClient with_types_in super::bindings);
}

use crate::{any::Any, wit_to_usize};

pub struct GuestsideDeserializerClient {
    deserialize_seed: Box<dyn ErasedDeserializeSeed>,
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
                deserialize_seed,
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
                deserialize_seed,
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }
}

impl bindings::exports::serde::serde::serde_deserialize::GuestDeserializeSeed
    for GuestsideDeserializerClient
{
    fn deserialize(
        this: bindings::exports::serde::serde::serde_deserialize::DeserializeSeed,
        deserializer: bindings::exports::serde::serde::serde_deserialize::OwnedDeserializerHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self {
            deserialize_seed, ..
        } = this.into_inner();

        // TODO: Safety
        let deserializer = unsafe {
            bindings::serde::serde::serde_deserializer::Deserializer::from_handle(
                deserializer.owned_handle,
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
    ) -> Result<Self::Ok, bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle>;
}

impl WrapDeResult for Result<DeValue, DeError> {
    type Ok = bindings::exports::serde::serde::serde_deserialize::DeValue;

    fn wrap(
        self,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        match self {
            Ok(ok) => Ok(bindings::exports::serde::serde::serde_deserialize::DeValue::new(ok)),
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
    deserializer: bindings::serde::serde::serde_deserializer::Deserializer,
}

fn unwrap_deserializer_result<V>(
    result: Result<
        bindings::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::serde::serde::serde_deserializer::DeError,
    >,
    value_ty: &'static str,
) -> Result<V, DeError> {
    match result {
        Ok(value) => {
            // TODO: Safety
            let value = unsafe {
                bindings::exports::serde::serde::serde_deserialize::DeValue::from_handle(
                    value.owned_handle,
                )
            };
            let DeValue { value } = value.into_inner();
            // TODO: Safety
            let Some(value): Option<V> = (unsafe { value.take() }) else {
                return Err(::serde::de::Error::custom(format!(
                    "bug: {value_ty} type mismatch across the wit boundary"
                )));
            };
            Ok(value)
        }
        Err(error) => Err(DeError { error }),
    }
}

fn unwrap_deserializer_optional_result<V>(
    result: Result<
        Option<bindings::serde::serde::serde_deserializer::OwnedDeValueHandle>,
        bindings::serde::serde::serde_deserializer::DeError,
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
    fn with_new<
        'de,
        V: ::serde::de::Visitor<'de>,
        F: FnOnce(
            Self,
        ) -> Result<
            bindings::serde::serde::serde_deserializer::OwnedDeValueHandle,
            bindings::serde::serde::serde_deserializer::DeError,
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

            #[allow(clippy::items_after_statements)]
            struct Expecting<'a> {
                visitor: &'a dyn ErasedVisitor,
            }

            #[allow(clippy::items_after_statements)]
            impl<'a> fmt::Display for Expecting<'a> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.visitor.erased_expecting(f)
                }
            }

            let expecting = format!("{}", Expecting { visitor: &*visitor });

            inner(Self {
                visitor,
                expecting,
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        unwrap_deserializer_result(result, "Visitor::Value")
    }
}

impl bindings::exports::serde::serde::serde_deserialize::GuestVisitor for Visitor {
    fn expecting(
        this: bindings::exports::serde::serde::serde_deserialize::VisitorBorrow,
    ) -> String {
        this.get::<Self>().expecting.clone()
    }

    fn visit_bool(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: bool,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_bool(v).wrap()
    }

    fn visit_i8(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: i8,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_i8(v).wrap()
    }

    fn visit_i16(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: i16,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_i16(v).wrap()
    }

    fn visit_i32(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: i32,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_i32(v).wrap()
    }

    fn visit_i64(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: i64,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_i64(v).wrap()
    }

    fn visit_i128(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: bindings::serde::serde::serde_types::S128,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_i128(i128::from_le_bytes(bytes)).wrap()
    }

    fn visit_u8(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: u8,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_u8(v).wrap()
    }

    fn visit_u16(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: u16,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_u16(v).wrap()
    }

    fn visit_u32(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: u32,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_u32(v).wrap()
    }

    fn visit_u64(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: u64,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_u64(v).wrap()
    }

    fn visit_u128(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: bindings::serde::serde::serde_types::U128,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_u128(u128::from_le_bytes(bytes)).wrap()
    }

    fn visit_f32(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: f32,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_f32(v).wrap()
    }

    fn visit_f64(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: f64,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_f64(v).wrap()
    }

    fn visit_char(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: char,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_char(v).wrap()
    }

    fn visit_string(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: String,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_string(v).wrap()
    }

    fn visit_byte_buf(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        v: Vec<u8>,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_byte_buf(v).wrap()
    }

    fn visit_none(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_none().wrap()
    }

    fn visit_some(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        deserializer: bindings::exports::serde::serde::serde_deserialize::OwnedDeserializerHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let deserializer = unsafe {
            bindings::serde::serde::serde_deserializer::Deserializer::from_handle(
                deserializer.owned_handle,
            )
        };

        let Self { visitor, .. } = this.into_inner();
        visitor
            .erased_visit_some(DeserializerableDeserializer { deserializer })
            .wrap()
    }

    fn visit_unit(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        let Self { visitor, .. } = this.into_inner();
        visitor.erased_visit_unit().wrap()
    }

    fn visit_newtype_struct(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        deserializer: bindings::exports::serde::serde::serde_deserialize::OwnedDeserializerHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let deserializer = unsafe {
            bindings::serde::serde::serde_deserializer::Deserializer::from_handle(
                deserializer.owned_handle,
            )
        };

        let Self { visitor, .. } = this.into_inner();
        visitor
            .erased_visit_newtype_struct(DeserializerableDeserializer { deserializer })
            .wrap()
    }

    fn visit_seq(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        seq: bindings::exports::serde::serde::serde_deserialize::OwnedSeqAccessHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let seq = unsafe {
            bindings::serde::serde::serde_deserializer::SeqAccess::from_handle(seq.owned_handle)
        };

        let Self { visitor, .. } = this.into_inner();
        visitor
            .erased_visit_seq(DeserializerableSeqAccess {
                seq_access: Some(seq),
            })
            .wrap()
    }

    fn visit_map(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        map: bindings::exports::serde::serde::serde_deserialize::OwnedMapAccessHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let map = unsafe {
            bindings::serde::serde::serde_deserializer::MapAccess::from_handle(map.owned_handle)
        };

        let Self { visitor, .. } = this.into_inner();
        visitor
            .erased_visit_map(DeserializerableMapAccess {
                map_access: Some(map),
            })
            .wrap()
    }

    fn visit_enum(
        this: bindings::exports::serde::serde::serde_deserialize::Visitor,
        data: bindings::exports::serde::serde::serde_deserialize::OwnedEnumAccessHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserialize::DeValue,
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    > {
        // TODO: Safety
        let data = unsafe {
            bindings::serde::serde::serde_deserializer::EnumAccess::from_handle(data.owned_handle)
        };

        let Self { visitor, .. } = this.into_inner();
        visitor
            .erased_visit_enum(DeserializerableEnumAccess { enum_access: data })
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_any(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_bool<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_bool(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_i8<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_i8(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_i16<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_i16(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_i32<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_i32(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_i64<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_i64(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
                let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

                bindings::serde::serde::serde_deserializer::Deserializer::deserialize_i128(
                    self.deserializer,
                    bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                        owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_u8(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_u16<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_u16(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_u32<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_u32(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_u64<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_u64(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
                let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

                bindings::serde::serde::serde_deserializer::Deserializer::deserialize_u128(
                    self.deserializer,
                    bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                        owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_f32(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_f64<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_f64(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_char<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_char(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_str<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_str(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_string<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_string(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_bytes<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_bytes(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_byte_buf<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_byte_buf(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_option<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_option(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_unit<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_unit(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_unit_struct(
                self.deserializer,
                name,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_newtype_struct(
                self.deserializer,
                name,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_seq<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_seq(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            Ok(len) => bindings::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Err(::serde::de::Error::custom(
                    "Deserializer::deserialize_tuple len exceeds u32",
                ));
            }
        };

        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_tuple(
                self.deserializer,
                len,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            Ok(len) => bindings::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Err(::serde::de::Error::custom(
                    "Deserializer::deserialize_tuple_struct len exceeds u32",
                ));
            }
        };

        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_tuple_struct(
                self.deserializer,
                name,
                len,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_map<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_map(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_struct(
                self.deserializer,
                name,
                &fields,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
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
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_enum(
                self.deserializer,
                name,
                &variants,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_identifier<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_identifier(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn deserialize_ignored_any<V: ::serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::Deserializer::deserialize_ignored_any(
                self.deserializer,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn is_human_readable(&self) -> bool {
        bindings::serde::serde::serde_deserializer::Deserializer::is_human_readable(
            &self.deserializer,
        )
    }
}

struct Visitor {
    #[allow(clippy::struct_field_names)]
    visitor: Box<dyn ErasedVisitor>,
    expecting: String,
    _scope: ScopedBorrowMut<()>,
}

struct DeserializerableSeqAccess {
    seq_access: Option<bindings::serde::serde::serde_deserializer::SeqAccess>,
}

impl<'de> ::serde::de::SeqAccess<'de> for DeserializerableSeqAccess {
    type Error = DeError;

    fn next_element_seed<T: ::serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        let Some(seq_access) = self.seq_access.take() else {
            return Err(::serde::de::Error::custom(
                "bug: SeqAccess::next_element_seed after free",
            ));
        };

        let (seq_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                let seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(seed);
                bindings::serde::serde::serde_deserializer::SeqAccess::next_element_seed(
                    seq_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(seed).handle(),
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
            .and_then(bindings::serde::serde::serde_deserializer::SeqAccess::size_hint)
            .map(|size_hint| wit_to_usize(size_hint.val))
    }
}

struct DeserializerableMapAccess {
    map_access: Option<bindings::serde::serde::serde_deserializer::MapAccess>,
}

impl<'de> ::serde::de::MapAccess<'de> for DeserializerableMapAccess {
    type Error = DeError;

    fn next_key_seed<K: ::serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(::serde::de::Error::custom(
                "bug: MapAccess::next_key_seed after free",
            ));
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                let seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(seed);
                bindings::serde::serde::serde_deserializer::MapAccess::next_key_seed(
                    map_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(seed).handle(),
                    },
                )
            })
        };
        self.map_access = Some(map_access);

        unwrap_deserializer_optional_result(result, "MapAccess::<T>::Value")
    }

    fn next_value_seed<V: ::serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(::serde::de::Error::custom(
                "bug: MapAccess::next_key_seed after free",
            ));
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                let seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(seed);
                bindings::serde::serde::serde_deserializer::MapAccess::next_value_seed(
                    map_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(seed).handle(),
                    },
                )
            })
        };
        self.map_access = Some(map_access);

        unwrap_deserializer_result(result, "MapAccess::<T>::Value")
    }

    fn next_entry_seed<
        K: ::serde::de::DeserializeSeed<'de>,
        V: ::serde::de::DeserializeSeed<'de>,
    >(
        &mut self,
        key_seed: K,
        value_seed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error> {
        let Some(map_access) = self.map_access.take() else {
            return Err(::serde::de::Error::custom(
                "bug: MapAccess::next_entry_seed after free",
            ));
        };

        let (map_access, result) = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(key_seed, |key_seed| {
                let key_seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(
                        key_seed,
                    );
                bindings::serde::serde::serde_deserializer::MapAccess::next_key_seed(
                    map_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(key_seed).handle(),
                    },
                )
            })
        };

        let key = match unwrap_deserializer_optional_result(result, "MapAccess::<T>::Value") {
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
            GuestsideDeserializerClient::with_new_seed_unchecked(value_seed, |value_seed| {
                let value_seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(
                        value_seed,
                    );
                bindings::serde::serde::serde_deserializer::MapAccess::next_value_seed(
                    map_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(value_seed).handle(),
                    },
                )
            })
        };
        self.map_access = Some(map_access);

        let value = unwrap_deserializer_result(result, "MapAccess::<T>::Value")?;

        Ok(Some((key, value)))
    }

    fn size_hint(&self) -> Option<usize> {
        self.map_access
            .as_ref()
            .and_then(bindings::serde::serde::serde_deserializer::MapAccess::size_hint)
            .map(|size_hint| wit_to_usize(size_hint.val))
    }
}

struct DeserializerableEnumAccess {
    enum_access: bindings::serde::serde::serde_deserializer::EnumAccess,
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
                let seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(seed);
                bindings::serde::serde::serde_deserializer::EnumAccess::variant_seed(
                    self.enum_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(seed).handle(),
                    },
                )
            })
        };

        match result {
            Ok((value, variant)) => {
                // TODO: Safety
                let value = unsafe {
                    bindings::exports::serde::serde::serde_deserialize::DeValue::from_handle(
                        value.owned_handle,
                    )
                };
                let DeValue { value } = value.into_inner();
                // TODO: Safety
                let Some(value): Option<V::Value> = (unsafe { value.take() }) else {
                    return Err(::serde::de::Error::custom(
                        "bug: EnumAccess::variant_seed::<V>::Value type mismatch across the wit boundary",
                    ));
                };
                Ok((
                    value,
                    DeserializerableVariantAccess {
                        variant_access: variant,
                    },
                ))
            }
            Err(error) => Err(DeError { error }),
        }
    }
}

struct DeserializerableVariantAccess {
    variant_access: bindings::serde::serde::serde_deserializer::VariantAccess,
}

impl<'de> ::serde::de::VariantAccess<'de> for DeserializerableVariantAccess {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        bindings::serde::serde::serde_deserializer::VariantAccess::unit_variant(self.variant_access)
            .map_err(|error| DeError { error })
    }

    fn newtype_variant_seed<T: ::serde::de::DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        let result = unsafe {
            GuestsideDeserializerClient::with_new_seed_unchecked(seed, |seed| {
                let seed =
                    bindings::exports::serde::serde::serde_deserialize::DeserializeSeed::new(seed);
                bindings::serde::serde::serde_deserializer::VariantAccess::newtype_variant_seed(
                    self.variant_access,
                    bindings::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle {
                        owned_handle: core::mem::ManuallyDrop::new(seed).handle(),
                    },
                )
            })
        };

        unwrap_deserializer_result(result, "VariantAccess::newtype_variant_seed::<T>::Value")
    }

    fn tuple_variant<V: ::serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let len = match u32::try_from(len) {
            Ok(len) => bindings::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Err(::serde::de::Error::custom(
                    "VariantAccess::tuple_variant len exceeds u32",
                ));
            }
        };

        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::VariantAccess::tuple_variant(
                self.variant_access,
                len,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
        })
    }

    fn struct_variant<V: ::serde::de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let fields = fields.iter().copied().map(String::from).collect::<Vec<_>>();

        Visitor::with_new(visitor, |visitor| {
            let visitor = bindings::exports::serde::serde::serde_deserialize::Visitor::new(visitor);

            bindings::serde::serde::serde_deserializer::VariantAccess::struct_variant(
                self.variant_access,
                &fields,
                bindings::serde::serde::serde_deserializer::OwnedVisitorHandle {
                    owned_handle: core::mem::ManuallyDrop::new(visitor).handle(),
                },
            )
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
}

impl bindings::exports::serde::serde::serde_deserialize::GuestDeValue for DeValue {
    fn id(
        this: bindings::exports::serde::serde::serde_deserialize::DeValue,
    ) -> bindings::exports::serde::serde::serde_deserialize::DeValue {
        this
    }
}

struct DeError {
    error: bindings::serde::serde::serde_deserializer::DeError,
}

impl DeError {
    fn into_handle(self) -> bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle {
        bindings::exports::serde::serde::serde_deserialize::OwnedDeErrorHandle {
            owned_handle: core::mem::ManuallyDrop::new(self.error).handle(),
        }
    }
}

fn translate_serde_de_unexpected(
    unexp: ::serde::de::Unexpected,
) -> bindings::serde::serde::serde_deserializer::Unexpected {
    match unexp {
        ::serde::de::Unexpected::Bool(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Bool(v)
        }
        ::serde::de::Unexpected::Unsigned(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Unsigned(v)
        }
        ::serde::de::Unexpected::Signed(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Signed(v)
        }
        ::serde::de::Unexpected::Float(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Float(v)
        }
        ::serde::de::Unexpected::Char(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Char(v)
        }
        ::serde::de::Unexpected::Str(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Str(String::from(v))
        }
        ::serde::de::Unexpected::Bytes(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Bytes(Vec::from(v))
        }
        ::serde::de::Unexpected::Unit => {
            bindings::serde::serde::serde_deserializer::Unexpected::Unit
        }
        ::serde::de::Unexpected::Option => {
            bindings::serde::serde::serde_deserializer::Unexpected::Option
        }
        ::serde::de::Unexpected::NewtypeStruct => {
            bindings::serde::serde::serde_deserializer::Unexpected::NewtypeStruct
        }
        ::serde::de::Unexpected::Seq => bindings::serde::serde::serde_deserializer::Unexpected::Seq,
        ::serde::de::Unexpected::Map => bindings::serde::serde::serde_deserializer::Unexpected::Map,
        ::serde::de::Unexpected::Enum => {
            bindings::serde::serde::serde_deserializer::Unexpected::Enum
        }
        ::serde::de::Unexpected::UnitVariant => {
            bindings::serde::serde::serde_deserializer::Unexpected::UnitVariant
        }
        ::serde::de::Unexpected::NewtypeVariant => {
            bindings::serde::serde::serde_deserializer::Unexpected::NewtypeVariant
        }
        ::serde::de::Unexpected::TupleVariant => {
            bindings::serde::serde::serde_deserializer::Unexpected::TupleVariant
        }
        ::serde::de::Unexpected::StructVariant => {
            bindings::serde::serde::serde_deserializer::Unexpected::StructVariant
        }
        ::serde::de::Unexpected::Other(v) => {
            bindings::serde::serde::serde_deserializer::Unexpected::Other(String::from(v))
        }
    }
}

impl ::serde::de::Error for DeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            error: bindings::serde::serde::serde_deserializer::DeError::custom(&format!("{msg}")),
        }
    }

    fn invalid_type(unexp: ::serde::de::Unexpected, exp: &dyn ::serde::de::Expected) -> Self {
        Self {
            error: bindings::serde::serde::serde_deserializer::DeError::invalid_type(
                &translate_serde_de_unexpected(unexp),
                &format!("{exp}"),
            ),
        }
    }

    fn invalid_value(unexp: ::serde::de::Unexpected, exp: &dyn ::serde::de::Expected) -> Self {
        Self {
            error: bindings::serde::serde::serde_deserializer::DeError::invalid_value(
                &translate_serde_de_unexpected(unexp),
                &format!("{exp}"),
            ),
        }
    }

    fn invalid_length(len: usize, exp: &dyn ::serde::de::Expected) -> Self {
        let len = match u32::try_from(len) {
            Ok(len) => bindings::serde::serde::serde_types::Usize { val: len },
            Err(_) => {
                return Self::custom("Deserializer::Error::invalid_length len exceeds u32");
            }
        };

        Self {
            error: bindings::serde::serde::serde_deserializer::DeError::invalid_length(
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
            error: bindings::serde::serde::serde_deserializer::DeError::unknown_variant(
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
            error: bindings::serde::serde::serde_deserializer::DeError::unknown_field(
                field, &expected,
            ),
        }
    }

    fn missing_field(field: &'static str) -> Self {
        Self {
            error: bindings::serde::serde::serde_deserializer::DeError::missing_field(field),
        }
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self {
            error: bindings::serde::serde::serde_deserializer::DeError::duplicate_field(field),
        }
    }
}

impl ::serde::de::StdError for DeError {}

impl fmt::Debug for DeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&bindings::serde::serde::serde_deserializer::DeError::debug(
            &self.error,
        ))
    }
}

impl fmt::Display for DeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&bindings::serde::serde::serde_deserializer::DeError::display(&self.error))
    }
}
