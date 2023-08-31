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
use crate::wit_to_usize;

pub struct GuestsideSerializerProvider {
    serializer: Option<Box<dyn ErasedSerializer>>,
    is_human_readable: bool,
    scope: Option<ScopedBorrowMut<()>>,
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

impl self::exports::serde::serde::serde_serializer::GuestSerializer
    for GuestsideSerializerProvider
{
    fn serialize_bool(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_none")?
            .erased_serialize_none()
            .wrap()
    }

    fn serialize_some(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };
        this.try_extract_serializer("serialize_some")?
            .erased_serialize_some(&SerializableSerialize::new(&value))
            .wrap()
    }

    fn serialize_unit(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        this.try_extract_serializer("serialize_unit")?
            .erased_serialize_unit()
            .wrap()
    }

    fn serialize_unit_struct(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };
        this.try_extract_serializer("serialize_newtype_struct")?
            .erased_serialize_newtype_struct(
                intern_string(name),
                &SerializableSerialize::new(&value),
            )
            .wrap()
    }

    fn serialize_newtype_variant(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
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
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
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
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        len: Option<self::serde::serde::serde_types::Usize>,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_seq = this
            .try_extract_serializer("serialize_seq")?
            .erased_serialize_seq(len.map(|len| wit_to_usize(len.val)))
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeSeq::new(
                GuestsideSerializeSeqProvider {
                    serialize_seq: Some(serialize_seq),
                    _scope: this.try_extract_scope("serialize_seq")?,
                },
            ),
        )
    }

    fn serialize_tuple(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        len: self::serde::serde::serde_types::Usize,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeTuple,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_tuple = this
            .try_extract_serializer("serialize_tuple")?
            .erased_serialize_tuple(wit_to_usize(len.val))
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeTuple::new(
                GuestsideSerializeTupleProvider {
                    serialize_tuple: Some(serialize_tuple),
                    _scope: this.try_extract_scope("serialize_tuple")?,
                },
            ),
        )
    }

    fn serialize_tuple_struct(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        len: self::serde::serde::serde_types::Usize,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeTupleStruct,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_tuple_struct = this
            .try_extract_serializer("serialize_tuple_struct")?
            .erased_serialize_tuple_struct(intern_string(name), wit_to_usize(len.val))
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeTupleStruct::new(
                GuestsideSerializeTupleStructProvider {
                    serialize_tuple_struct: Some(serialize_tuple_struct),
                    _scope: this.try_extract_scope("serialize_tuple_struct")?,
                },
            ),
        )
    }

    fn serialize_tuple_variant(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        variant_index: u32,
        variant: String,
        len: self::serde::serde::serde_types::Usize,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeTupleVariant,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_tuple_variant = this
            .try_extract_serializer("serialize_tuple_variant")?
            .erased_serialize_tuple_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
                wit_to_usize(len.val),
            )
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeTupleVariant::new(
                GuestsideSerializeTupleVariantProvider {
                    serialize_tuple_variant: Some(serialize_tuple_variant),
                    _scope: this.try_extract_scope("serialize_tuple_variant")?,
                },
            ),
        )
    }

    fn serialize_map(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        len: Option<self::serde::serde::serde_types::Usize>,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeMap,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_map = this
            .try_extract_serializer("serialize_map")?
            .erased_serialize_map(len.map(|len| wit_to_usize(len.val)))
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeMap::new(
                GuestsideSerializeMapProvider {
                    serialize_map: Some(serialize_map),
                    _scope: this.try_extract_scope("serialize_map")?,
                },
            ),
        )
    }

    fn serialize_struct(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        len: self::serde::serde::serde_types::Usize,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeStruct,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_struct = this
            .try_extract_serializer("serialize_struct")?
            .erased_serialize_struct(intern_string(name), wit_to_usize(len.val))
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeStruct::new(
                GuestsideSerializeStructProvider {
                    serialize_struct: Some(serialize_struct),
                    _scope: this.try_extract_scope("serialize_struct")?,
                },
            ),
        )
    }

    fn serialize_struct_variant(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializer,
        name: String,
        variant_index: u32,
        variant: String,
        len: self::serde::serde::serde_types::Usize,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let serialize_struct_variant = this
            .try_extract_serializer("serialize_struct_variant")?
            .erased_serialize_struct_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
                wit_to_usize(len.val),
            )
            .map_err(self::exports::serde::serde::serde_serializer::OwnSerError::new)?;

        Ok(
            self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant::new(
                GuestsideSerializeStructVariantProvider {
                    serialize_struct_variant: Some(serialize_struct_variant),
                    _scope: this.try_extract_scope("serialize_struct_variant")?,
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
                serializer: Some(serializer),
                scope: Some(scope.borrow_mut()),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }

    fn try_extract_serializer(
        &mut self,
        method: &'static str,
    ) -> Result<Box<dyn ErasedSerializer>, self::exports::serde::serde::serde_serializer::OwnSerError>
    {
        let Some(serializer) = self.serializer.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(format!("bug: Serializer::{method} after free"))
            }));
        };
        Ok(serializer)
    }

    fn try_extract_scope(
        &mut self,
        method: &'static str,
    ) -> Result<ScopedBorrowMut<()>, self::exports::serde::serde::serde_serializer::OwnSerError>
    {
        let Some(scope) = self.scope.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(format!("bug: Serializer::{method} after free"))
            }));
        };
        Ok(scope)
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

pub struct SerOk {
    value: Option<Any>,
}

impl SerOk {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: unsafe { Some(Any::new(value)) },
        }
    }
}

pub struct SerError {
    inner: SerErrorOrCustom,
}

enum SerErrorOrCustom {
    Error {
        err: Option<Any>,
        display: String,
        debug: String,
    },
    Custom(String),
}

impl self::exports::serde::serde::serde_serializer::GuestSerError for SerError {
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
                err: Some(unsafe { Any::new(err) }),
                display,
                debug,
            },
        }
    }
}

pub struct GuestsideSerializeSeqProvider {
    serialize_seq: Option<Box<dyn ErasedSerializeSeq>>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleProvider {
    serialize_tuple: Option<Box<dyn ErasedSerializeTuple>>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleStructProvider {
    serialize_tuple_struct: Option<Box<dyn ErasedSerializeTupleStruct>>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleVariantProvider {
    serialize_tuple_variant: Option<Box<dyn ErasedSerializeTupleVariant>>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeMapProvider {
    serialize_map: Option<Box<dyn ErasedSerializeMap>>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeStructProvider {
    serialize_struct: Option<Box<dyn ErasedSerializeStruct>>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeStructVariantProvider {
    serialize_struct_variant: Option<Box<dyn ErasedSerializeStructVariant>>,
    _scope: ScopedBorrowMut<()>,
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeSeq
    for GuestsideSerializeSeqProvider
{
    fn serialize_element(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_seq) = this.serialize_seq.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeSeq::serialize_element after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_seq
            .erased_serialize_element(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeSeq,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_seq) = this.serialize_seq.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeSeq::end after free"))
            }));
        };
        serialize_seq.erased_end().wrap()
    }
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeTuple
    for GuestsideSerializeTupleProvider
{
    fn serialize_element(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeTuple,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeTuple,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_tuple) = this.serialize_tuple.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeTuple::serialize_element after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_tuple
            .erased_serialize_element(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeTuple,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_tuple) = this.serialize_tuple.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeTuple::end after free"))
            }));
        };
        serialize_tuple.erased_end().wrap()
    }
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeTupleStruct
    for GuestsideSerializeTupleStructProvider
{
    fn serialize_field(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeTupleStruct,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeTupleStruct,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_tuple_struct) = this.serialize_tuple_struct.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeTupleStruct::serialize_field after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_tuple_struct
            .erased_serialize_field(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeTupleStruct,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_tuple_struct) = this.serialize_tuple_struct.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeTupleStruct::end after free"))
            }));
        };
        serialize_tuple_struct.erased_end().wrap()
    }
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeTupleVariant
    for GuestsideSerializeTupleVariantProvider
{
    fn serialize_field(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeTupleVariant,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeTupleVariant,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_tuple_variant) = this.serialize_tuple_variant.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeTupleVariant::serialize_field after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_tuple_variant
            .erased_serialize_field(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeTupleVariant,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_tuple_variant) = this.serialize_tuple_variant.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeTupleVariant::end after free"))
            }));
        };
        serialize_tuple_variant.erased_end().wrap()
    }
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeMap
    for GuestsideSerializeMapProvider
{
    fn serialize_key(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeMap,
        key: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeMap,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_map) = this.serialize_map.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeMap::serialize_key after free"))
            })));
        };

        // TODO: Safety
        let key = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(key.borrowed_handle)
        };

        let result = serialize_map
            .erased_serialize_key(&SerializableSerialize::new(&key))
            .wrap();

        (this, result)
    }

    fn serialize_value(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeMap,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeMap,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_map) = this.serialize_map.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeMap::serialize_value after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_map
            .erased_serialize_key(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeMap,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_map) = this.serialize_map.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeMap::end after free"))
            }));
        };
        serialize_map.erased_end().wrap()
    }
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeStruct
    for GuestsideSerializeStructProvider
{
    fn serialize_field(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeStruct,
        key: String,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeStruct,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_struct) = this.serialize_struct.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeStruct::serialize_field after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_struct
            .erased_serialize_field(intern_string(key), &SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeStruct,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_struct) = this.serialize_struct.take() else {
        return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
            inner: SerErrorOrCustom::Custom(String::from("bug: SerializeStruct::end after free"))
        }));
    };
        serialize_struct.erased_end().wrap()
    }

    fn skip_field(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeStruct,
        key: String,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeStruct,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_struct) = this.serialize_struct.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeStruct::skip_field after free"))
            })));
        };

        let result = serialize_struct
            .erased_skip_field(intern_string(key))
            .wrap();

        (this, result)
    }
}

impl self::exports::serde::serde::serde_serializer::GuestSerializeStructVariant
    for GuestsideSerializeStructVariantProvider
{
    fn serialize_field(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant,
        key: String,
        value: self::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_struct_variant) = this.serialize_struct_variant.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeStructVariant::serialize_field after free"))
            })));
        };

        // TODO: Safety
        let value = unsafe {
            self::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = serialize_struct_variant
            .erased_serialize_field(intern_string(key), &SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant,
    ) -> Result<
        self::exports::serde::serde::serde_serializer::OwnSerOk,
        self::exports::serde::serde::serde_serializer::OwnSerError,
    > {
        let Some(serialize_struct_variant) = this.serialize_struct_variant.take() else {
            return Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeStructVariant::end after free"))
            }));
        };
        serialize_struct_variant.erased_end().wrap()
    }

    fn skip_field(
        mut this: self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant,
        key: String,
    ) -> (
        self::exports::serde::serde::serde_serializer::OwnSerializeStructVariant,
        Result<(), self::exports::serde::serde::serde_serializer::OwnSerError>,
    ) {
        let Some(serialize_struct_variant) = this.serialize_struct_variant.as_mut() else {
            return (this, Err(self::exports::serde::serde::serde_serializer::OwnSerError::new(SerError {
                inner: SerErrorOrCustom::Custom(String::from("bug: SerializeStructVariant::skip_field after free"))
            })));
        };

        let result = serialize_struct_variant
            .erased_skip_field(intern_string(key))
            .wrap();

        (this, result)
    }
}

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
            let owned_handle = wit_bindgen::Resource::into_handle(
                self::exports::serde::serde::serde_serializer::OwnSerializer::new(serializer),
            );
            let serializer =
                self::serde::serde::serde_serialize::OwnedSerializerHandle { owned_handle };
            self.serialize.serialize(serializer)
        });

        match result {
            Ok(value) => {
                // TODO: Safety
                let mut value = unsafe {
                    self::exports::serde::serde::serde_serializer::OwnSerOk::from_handle(
                        value.owned_handle,
                    )
                };
                let Some(value) = value.value.take() else {
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
                let mut err = unsafe {
                    self::exports::serde::serde::serde_serializer::OwnSerError::from_handle(
                        err.owned_handle,
                    )
                };
                let err = match &mut err.inner {
                    SerErrorOrCustom::Error { err, .. } => err,
                    SerErrorOrCustom::Custom(msg) => return Err(::serde::ser::Error::custom(msg)),
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
