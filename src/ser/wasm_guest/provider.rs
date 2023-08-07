use core::cell::RefCell;

use alloc::{boxed::Box, format, string::String, vec::Vec};

use ::serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use scoped_reference::{ScopedBorrowMut, ScopedReference};

wit_bindgen::generate!({ world: "serde-serializer-provider", exports: {
    "serde:serde/serde-serializer/serializer": GuestsideSerializerProvider,
    "serde:serde/serde-serializer/ser-ok": SerOk,
    "serde:serde/serde-serializer/ser-error": SerError,
    "serde:serde/serde-serializer/serialize-seq": GuestsideSerializeSeqProvider,
    "serde:serde/serde-serializer/serialize-tuple": GuestsideSerializeTupleProvider,
    "serde:serde/serde-serializer/serialize-tuple-struct": GuestsideSerializeTupleStructProvider,
    "serde:serde/serde-serializer/serialize-tuple-variant": GuestsideSerializeTupleVariantProvider,
    "serde:serde/serde-serializer/serialize-map": GuestsideSerializeMapProvider,
    "serde:serde/serde-serializer/serialize-struct": GuestsideSerializeStructProvider,
    "serde:serde/serde-serializer/serialize-struct-variant": GuestsideSerializeStructVariantProvider,
}, std_feature });

use crate::any::Any;
use crate::intern::intern_string;

pub struct GuestsideSerializerProvider {
    serializer: RefCell<Option<Box<dyn ErasedSerializer>>>,
    is_human_readable: bool,
    scope: RefCell<Option<ScopedBorrowMut<()>>>,
}

trait WrapSerResult {
    type Ok;

    fn wrap(self) -> Result<Self::Ok, self::exports::serde::serde::serde_serializer::OwnSerError>;
}

impl WrapSerResult for Result<SerOk, SerError> {
    type Ok = self::exports::serde::serde::serde_serializer::OwnSerOk;

    fn wrap(
        self,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        match self {
            Ok(ok) => Ok(self::exports::serde::serde::serde_serializer::OwnSerOk::new(ok)),
            Err(error) => {
                Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(error))
            }
        }
    }
}

impl WrapSerResult for Result<(), SerError> {
    type Ok = ();

    fn wrap(self) -> Result<(), self::exports::serde::serde::serde_serializer::OwnSerError> {
        self.map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)
    }
}

impl self::exports::serde::serde::serde_serializer::Serializer for GuestsideSerializerProvider {
    fn serialize_bool(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: bool,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_bool")?
            .erased_serialize_bool(v)
            .wrap()
    }

    fn serialize_i8(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: i8,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_i8")?
            .erased_serialize_i8(v)
            .wrap()
    }

    fn serialize_i16(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: i16,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_i16")?
            .erased_serialize_i16(v)
            .wrap()
    }

    fn serialize_i32(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: i32,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_i32")?
            .erased_serialize_i32(v)
            .wrap()
    }

    fn serialize_i64(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: i64,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_i64")?
            .erased_serialize_i64(v)
            .wrap()
    }

    fn serialize_i128(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: self::serde::serde::serde_types::S128,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        this.try_extract_serializer("serialize_i128")?
            .erased_serialize_i128(i128::from_le_bytes(bytes))
            .wrap()
    }

    fn serialize_u8(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: u8,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_u8")?
            .erased_serialize_u8(v)
            .wrap()
    }

    fn serialize_u16(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: u16,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_u16")?
            .erased_serialize_u16(v)
            .wrap()
    }

    fn serialize_u32(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: u32,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_u32")?
            .erased_serialize_u32(v)
            .wrap()
    }

    fn serialize_u64(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: u64,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_u64")?
            .erased_serialize_u64(v)
            .wrap()
    }

    fn serialize_u128(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: self::serde::serde::serde_types::U128,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        this.try_extract_serializer("serialize_u128")?
            .erased_serialize_u128(u128::from_le_bytes(bytes))
            .wrap()
    }

    fn serialize_f32(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: f32,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_f32")?
            .erased_serialize_f32(v)
            .wrap()
    }

    fn serialize_f64(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: f64,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_f64")?
            .erased_serialize_f64(v)
            .wrap()
    }

    fn serialize_char(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: char,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_char")?
            .erased_serialize_char(v)
            .wrap()
    }

    fn serialize_str(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: String,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_str")?
            .erased_serialize_str(&v)
            .wrap()
    }

    fn serialize_bytes(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        v: Vec<u8>,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_bytes")?
            .erased_serialize_bytes(&v)
            .wrap()
    }

    fn serialize_none(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_none")?
            .erased_serialize_none()
            .wrap()
    }

    fn serialize_some(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(
                value.borrowed_handle,
                false,
            )
        };
        this.try_extract_serializer("serialize_some")?
            .erased_serialize_some(&SerializableSerialize::new(&value))
            .wrap()
    }

    fn serialize_unit(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_unit")?
            .erased_serialize_unit()
            .wrap()
    }

    fn serialize_unit_struct(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_unit_struct")?
            .erased_serialize_unit_struct(intern_string(name))
            .wrap()
    }

    fn serialize_unit_variant(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        variant_index: u32,
        variant: String,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_unit_variant")?
            .erased_serialize_unit_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
            )
            .wrap()
    }

    fn serialize_newtype_struct(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(
                value.borrowed_handle,
                false,
            )
        };
        this.try_extract_serializer("serialize_newtype_struct")?
            .erased_serialize_newtype_struct(
                intern_string(name),
                &SerializableSerialize::new(&value),
            )
            .wrap()
    }

    fn serialize_newtype_variant(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        variant_index: u32,
        variant: String,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(
                value.borrowed_handle,
                false,
            )
        };
        this.try_extract_serializer("serialize_newtype_variant")?
            .erased_serialize_newtype_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
                &SerializableSerialize::new(&value),
            )
            .wrap()
    }

    fn serialize_seq(
        this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        len: Option<self::serde::serde::serde_types::Usize>,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_seq = this
            .try_extract_serializer("serialize_seq")?
            .erased_serialize_seq(len.map(|len| len.val as usize))
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeSeq::new(
                GuestsideSerializeSeqProvider {
                    serialize_seq: RefCell::new(Some(serialize_seq)),
                    _scope: this.try_extract_scope("serialize_seq")?,
                },
            ),
        )
    }

    fn is_human_readable(this: &Self) -> bool {
        this.is_human_readable
    }
}

impl GuestsideSerializerProvider {
    #[must_use]
    pub fn with_new<'a, S: ::serde::Serializer + 'a, F: FnOnce(Self) -> Q, Q>(
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
                is_human_readable: serializer.erased_is_human_readable(),
                serializer: RefCell::new(Some(serializer)),
                scope: RefCell::new(Some(scope.borrow_mut())),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    fn try_extract_serializer(
        &self,
        method: &'static str,
    ) -> Result<Box<dyn ErasedSerializer>, self::exports::serde::serde::serde_serializer::OwnSerError>
    {
        let Ok(mut serializer) = self.serializer.try_borrow_mut() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(format!("bug: could not mutably borrow the owned Serializer in Serializer::{method}"))
            }));
        };
        let Some(serializer) = serializer.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(format!("bug: Serializer::{method} after free"))
            }));
        };
        Ok(serializer)
    }

    fn try_extract_scope(
        &self,
        method: &'static str,
    ) -> Result<ScopedBorrowMut<()>, self::exports::serde::serde::serde_serializer::OwnSerError>
    {
        let Ok(mut scope) = self.scope.try_borrow_mut() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(format!("bug: could not mutably borrow the owned Serializer's scope in Serializer::{method}"))
            }));
        };
        let Some(scope) = scope.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(format!("bug: Serializer::{method} after free"))
            }));
        };
        Ok(scope)
    }

    /*fn serialize_tuple(
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
    }*/
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
    fn erased_serialize_i128(self: Box<Self>, v: i128) -> Result<SerOk, SerError>;
    fn erased_serialize_u128(self: Box<Self>, v: u128) -> Result<SerOk, SerError>;
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
    // fn erased_serialize_tuple<'a>(
    //     self: Box<Self>,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeTuple + 'a>, SerError>
    // where
    //     Self: 'a;
    // fn erased_serialize_tuple_struct<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeTupleStruct + 'a>, SerError>
    // where
    //     Self: 'a;
    // fn erased_serialize_tuple_variant<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     variant_index: u32,
    //     variant: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeTupleVariant + 'a>, SerError>
    // where
    //     Self: 'a;
    // fn erased_serialize_map<'a>(
    //     self: Box<Self>,
    //     len: Option<usize>,
    // ) -> Result<Box<dyn ErasedSerializeMap + 'a>, SerError>
    // where
    //     Self: 'a;
    // fn erased_serialize_struct<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeStruct + 'a>, SerError>
    // where
    //     Self: 'a;
    // fn erased_serialize_struct_variant<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     variant_index: u32,
    //     variant: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeStructVariant + 'a>, SerError>
    // where
    //     Self: 'a;
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

impl<T: ::serde::Serializer> ErasedSerializer for T {
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

    fn erased_serialize_i128(self: Box<Self>, v: i128) -> Result<SerOk, SerError> {
        self.serialize_i128(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
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

    fn erased_serialize_u128(self: Box<Self>, v: u128) -> Result<SerOk, SerError> {
        self.serialize_u128(v)
            .map(SerOk::wrap)
            .map_err(SerError::wrap)
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

    // fn erased_serialize_tuple<'a>(
    //     self: Box<Self>,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeTuple + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_tuple(len)
    //         .map(|serialize_tuple| {
    //             let serialize_tuple: Box<dyn ErasedSerializeTuple + 'a> = Box::new(serialize_tuple);
    //             serialize_tuple
    //         })
    //         .map_err(SerError::wrap)
    // }

    // fn erased_serialize_tuple_struct<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeTupleStruct + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_tuple_struct(name, len)
    //         .map(|serialize_tuple_struct| {
    //             let serialize_tuple_struct: Box<dyn ErasedSerializeTupleStruct + 'a> =
    //                 Box::new(serialize_tuple_struct);
    //             serialize_tuple_struct
    //         })
    //         .map_err(SerError::wrap)
    // }

    // fn erased_serialize_tuple_variant<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     variant_index: u32,
    //     variant: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeTupleVariant + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_tuple_variant(name, variant_index, variant, len)
    //         .map(|serialize_tuple_variant| {
    //             let serialize_tuple_variant: Box<dyn ErasedSerializeTupleVariant + 'a> =
    //                 Box::new(serialize_tuple_variant);
    //             serialize_tuple_variant
    //         })
    //         .map_err(SerError::wrap)
    // }

    // fn erased_serialize_map<'a>(
    //     self: Box<Self>,
    //     len: Option<usize>,
    // ) -> Result<Box<dyn ErasedSerializeMap + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_map(len)
    //         .map(|serialize_map| {
    //             let serialize_map: Box<dyn ErasedSerializeMap + 'a> = Box::new(serialize_map);
    //             serialize_map
    //         })
    //         .map_err(SerError::wrap)
    // }

    // fn erased_serialize_struct<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeStruct + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_struct(name, len)
    //         .map(|serialize_struct| {
    //             let serialize_struct: Box<dyn ErasedSerializeStruct + 'a> =
    //                 Box::new(serialize_struct);
    //             serialize_struct
    //         })
    //         .map_err(SerError::wrap)
    // }

    // fn erased_serialize_struct_variant<'a>(
    //     self: Box<Self>,
    //     name: &'static str,
    //     variant_index: u32,
    //     variant: &'static str,
    //     len: usize,
    // ) -> Result<Box<dyn ErasedSerializeStructVariant + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_struct_variant(name, variant_index, variant, len)
    //         .map(|serialize_struct_variant| {
    //             let serialize_struct_variant: Box<dyn ErasedSerializeStructVariant + 'a> =
    //                 Box::new(serialize_struct_variant);
    //             serialize_struct_variant
    //         })
    //         .map_err(SerError::wrap)
    // }

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

pub struct SerOk {
    value: RefCell<Option<Any>>,
}

impl SerOk {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: unsafe { RefCell::new(Some(Any::new(value))) },
        }
    }
}

pub struct SerError {
    inner: SerErrorOrCustom,
}

enum SerErrorOrCustom {
    Error {
        err: RefCell<Option<Any>>,
        display: String,
        debug: String,
    },
    Custom(String),
}

impl self::exports::serde::serde::serde_serializer::SerError for SerError {
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

    fn custom(msg: String) -> self::exports::serde::serde::serde_serializer::OwnSerError {
        let error = Self {
            inner: SerErrorOrCustom::Custom(msg),
        };

        self::exports::serde::serde::serde_serializer::OwnSerError::new(error)
    }
}

impl SerError {
    fn wrap<T: ::serde::ser::Error>(err: T) -> Self {
        let display = format!("{err}");
        let debug = format!("{err:?}");

        // Safety: TODO
        Self {
            inner: SerErrorOrCustom::Error {
                err: RefCell::new(Some(unsafe { Any::new(err) })),
                display,
                debug,
            },
        }
    }
}

pub struct GuestsideSerializeSeqProvider {
    serialize_seq: RefCell<Option<Box<dyn ErasedSerializeSeq>>>,
    _scope: ScopedBorrowMut<()>,
}

/*pub struct GuestsideSerializeTupleProvider {
    serialize_tuple: Box<dyn ErasedSerializeTuple>,
    scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleStructProvider {
    serialize_tuple_struct: Box<dyn ErasedSerializeTupleStruct>,
    scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleVariantProvider {
    serialize_tuple_variant: Box<dyn ErasedSerializeTupleVariant>,
    scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeMapProvider {
    serialize_map: Box<dyn ErasedSerializeMap>,
    scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeStructProvider {
    serialize_struct: Box<dyn ErasedSerializeStruct>,
    scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeStructVariantProvider {
    serialize_struct_variant: Box<dyn ErasedSerializeStructVariant>,
    scope: ScopedBorrowMut<()>,
}*/

impl self::exports::serde::serde::serde_serializer::SerializeSeq for GuestsideSerializeSeqProvider {
    fn serialize_element(
        this: self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Ok(mut serialize_seq_mut) = this.serialize_seq.try_borrow_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: could not mutably borrow the mutably borrowed SerializeSeq in SerializeSeq::serialize_element"))
            })));
        };
        let Some(serialize_seq) = serialize_seq_mut.as_mut() else {
            core::mem::drop(serialize_seq_mut);

            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeSeq::serialize_element after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(
                value.borrowed_handle,
                false,
            )
        };

        let result = serialize_seq
            .erased_serialize_element(&SerializableSerialize::new(&value))
            .wrap();

        core::mem::drop(serialize_seq_mut);

        (this, result)
    }

    fn end(
        this: self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Ok(mut serialize_seq) = this.serialize_seq.try_borrow_mut() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: could not mutably borrow the owned SerializeSeq in SerializeSeq::end"))
            }));
        };
        let Some(serialize_seq) = serialize_seq.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeSeq::end after free"))
            }));
        };
        serialize_seq.erased_end().wrap()
    }
}

/*impl GuestsideSerializeTupleProvider {
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
}*/

struct SerializableSerialize<'a> {
    serialize: &'a self::serde::serde::serde_serialize::Serialize,
}

impl<'a> SerializableSerialize<'a> {
    fn new(serialize: &'a self::serde::serde::serde_serialize::Serialize) -> Self {
        Self { serialize }
    }
}

impl<'a> ::serde::Serialize for SerializableSerialize<'a> {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let result = GuestsideSerializerProvider::with_new(serializer, |serializer| {
            let owned_handle =
                self::exports::serde::serde::serde_serializer::OwnSerializer::new(serializer)
                    .into_handle();
            let serializer =
                self::serde::serde::serde_serialize::OwnedSerializerHandle { owned_handle };
            self.serialize.serialize(serializer)
        });

        match result {
            Ok(value) => {
                // TODO: Safety
                let value = unsafe {
                    self::exports::serde::serde::serde_serializer::OwnSerOk::from_handle(
                        value.owned_handle,
                    )
                };
                let Ok(mut value) = value.value.try_borrow_mut() else {
                    return Err(::serde::ser::Error::custom(
                        "bug: could not mutably borrow the owned Serializer::Ok result",
                    ));
                };
                let Some(value) = value.take() else {
                    return Err(::serde::ser::Error::custom(
                        "bug: use of Serializer::Ok after free",
                    ));
                };
                // TODO: Safety
                let Some(value): Option<S::Ok> = (unsafe { value.take() }) else {
                    return Err(::serde::ser::Error::custom(
                        "bug: Serializer::Ok type mismatch across the wit boundary",
                    ))
                };
                Ok(value)
            }
            Err(err) => {
                // TODO: Safety
                let err = unsafe {
                    self::exports::serde::serde::serde_serializer::OwnSerError::from_handle(
                        err.owned_handle,
                    )
                };
                let err = match &err.inner {
                    SerErrorOrCustom::Error { err, .. } => err,
                    SerErrorOrCustom::Custom(msg) => return Err(::serde::ser::Error::custom(msg)),
                };
                let Ok(mut err) = err.try_borrow_mut() else {
                    return Err(::serde::ser::Error::custom(
                        "bug: could not mutably borrow the owned Serializer::Error result",
                    ));
                };
                let Some(err) = err.take() else {
                    return Err(::serde::ser::Error::custom(
                        "bug: use of Serializer::Error after free",
                    ));
                };
                // TODO: Safety
                let Some(err): Option<S::Error> = (unsafe { err.take() }) else {
                    return Err(::serde::ser::Error::custom(
                        "bug: Serializer::Error type mismatch across the wit boundary",
                    ))
                };
                Err(err)
            }
        }
    }
}
