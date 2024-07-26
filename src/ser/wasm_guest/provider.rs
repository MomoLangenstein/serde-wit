use alloc::{boxed::Box, format, string::String, vec::Vec};

use ::serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use scoped_reference::{ScopedBorrowMut, ScopedReference};

mod bindings {
    wit_bindgen::generate!({ world: "serde-serializer-provider", std_feature });
}

mod export {
    enum GuestsideSerializerProvider {}

    impl super::bindings::exports::serde::serde::serde_serializer::Guest
        for GuestsideSerializerProvider
    {
        type Serializer = super::GuestsideSerializerProvider;
        type SerOk = super::SerOk;
        type SerError = super::SerError;
        type SerializeSeq = super::GuestsideSerializeSeqProvider;
        type SerializeTuple = super::GuestsideSerializeTupleProvider;
        type SerializeTupleStruct = super::GuestsideSerializeTupleStructProvider;
        type SerializeTupleVariant = super::GuestsideSerializeTupleVariantProvider;
        type SerializeMap = super::GuestsideSerializeMapProvider;
        type SerializeStruct = super::GuestsideSerializeStructProvider;
        type SerializeStructVariant = super::GuestsideSerializeStructVariantProvider;
    }

    super::bindings::export!(GuestsideSerializerProvider with_types_in super::bindings);
}

use crate::any::Any;
use crate::intern::intern_string;
use crate::wit_to_usize;

pub struct GuestsideSerializerProvider {
    serializer: Box<dyn ErasedSerializer>,
    is_human_readable: bool,
    scope: ScopedBorrowMut<()>,
}

trait WrapSerResult {
    type Ok;

    fn wrap(self) -> Result<Self::Ok, bindings::exports::serde::serde::serde_serializer::SerError>;
}

impl WrapSerResult for Result<SerOk, SerError> {
    type Ok = bindings::exports::serde::serde::serde_serializer::SerOk;

    fn wrap(
        self,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        match self {
            Ok(ok) => Ok(bindings::exports::serde::serde::serde_serializer::SerOk::new(ok)),
            Err(error) => {
                Err(bindings::exports::serde::serde::serde_serializer::SerError::new(error))
            }
        }
    }
}

impl WrapSerResult for Result<(), SerError> {
    type Ok = ();

    fn wrap(self) -> Result<(), bindings::exports::serde::serde::serde_serializer::SerError> {
        self.map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializer
    for GuestsideSerializerProvider
{
    fn serialize_bool(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: bool,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_bool(v).wrap()
    }

    fn serialize_i8(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: i8,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_i8(v).wrap()
    }

    fn serialize_i16(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: i16,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_i16(v).wrap()
    }

    fn serialize_i32(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: i32,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_i32(v).wrap()
    }

    fn serialize_i64(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: i64,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_i64(v).wrap()
    }

    fn serialize_i128(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: bindings::serde::serde::serde_types::S128,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_i128(i128::from_le_bytes(bytes))
            .wrap()
    }

    fn serialize_u8(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: u8,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_u8(v).wrap()
    }

    fn serialize_u16(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: u16,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_u16(v).wrap()
    }

    fn serialize_u32(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: u32,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_u32(v).wrap()
    }

    fn serialize_u64(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: u64,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_u64(v).wrap()
    }

    fn serialize_u128(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: bindings::serde::serde::serde_types::U128,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_u128(u128::from_le_bytes(bytes))
            .wrap()
    }

    fn serialize_f32(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: f32,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_f32(v).wrap()
    }

    fn serialize_f64(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: f64,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_f64(v).wrap()
    }

    fn serialize_char(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: char,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_char(v).wrap()
    }

    fn serialize_str(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: String,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_str(&v).wrap()
    }

    fn serialize_bytes(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        v: Vec<u8>,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_bytes(&v).wrap()
    }

    fn serialize_none(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_none().wrap()
    }

    fn serialize_some(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };
        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_some(&SerializableSerialize::new(&value))
            .wrap()
    }

    fn serialize_unit(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer.erased_serialize_unit().wrap()
    }

    fn serialize_unit_struct(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_unit_struct(intern_string(name))
            .wrap()
    }

    fn serialize_unit_variant(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        variant_index: u32,
        variant: String,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_unit_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
            )
            .wrap()
    }

    fn serialize_newtype_struct(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };
        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_newtype_struct(
                intern_string(name),
                &SerializableSerialize::new(&value),
            )
            .wrap()
    }

    fn serialize_newtype_variant(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        variant_index: u32,
        variant: String,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };
        let Self { serializer, .. } = this.into_inner();
        serializer
            .erased_serialize_newtype_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
                &SerializableSerialize::new(&value),
            )
            .wrap()
    }

    fn serialize_seq(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        len: Option<bindings::serde::serde::serde_types::Usize>,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeSeq,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_seq = serializer
            .erased_serialize_seq(len.map(|len| wit_to_usize(len.val)))
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeSeq::new(
                GuestsideSerializeSeqProvider {
                    serialize_seq,
                    _scope: scope,
                },
            ),
        )
    }

    fn serialize_tuple(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        len: bindings::serde::serde::serde_types::Usize,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeTuple,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_tuple = serializer
            .erased_serialize_tuple(wit_to_usize(len.val))
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeTuple::new(
                GuestsideSerializeTupleProvider {
                    serialize_tuple,
                    _scope: scope,
                },
            ),
        )
    }

    fn serialize_tuple_struct(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        len: bindings::serde::serde::serde_types::Usize,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_tuple_struct = serializer
            .erased_serialize_tuple_struct(intern_string(name), wit_to_usize(len.val))
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct::new(
                GuestsideSerializeTupleStructProvider {
                    serialize_tuple_struct,
                    _scope: scope,
                },
            ),
        )
    }

    fn serialize_tuple_variant(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        variant_index: u32,
        variant: String,
        len: bindings::serde::serde::serde_types::Usize,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_tuple_variant = serializer
            .erased_serialize_tuple_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
                wit_to_usize(len.val),
            )
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant::new(
                GuestsideSerializeTupleVariantProvider {
                    serialize_tuple_variant,
                    _scope: scope,
                },
            ),
        )
    }

    fn serialize_map(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        len: Option<bindings::serde::serde::serde_types::Usize>,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeMap,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_map = serializer
            .erased_serialize_map(len.map(|len| wit_to_usize(len.val)))
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeMap::new(
                GuestsideSerializeMapProvider {
                    serialize_map,
                    _scope: scope,
                },
            ),
        )
    }

    fn serialize_struct(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        len: bindings::serde::serde::serde_types::Usize,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeStruct,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_struct = serializer
            .erased_serialize_struct(intern_string(name), wit_to_usize(len.val))
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeStruct::new(
                GuestsideSerializeStructProvider {
                    serialize_struct,
                    _scope: scope,
                },
            ),
        )
    }

    fn serialize_struct_variant(
        this: bindings::exports::serde::serde::serde_serializer::Serializer,
        name: String,
        variant_index: u32,
        variant: String,
        len: bindings::serde::serde::serde_types::Usize,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        let Self {
            serializer, scope, ..
        } = this.into_inner();

        let serialize_struct_variant = serializer
            .erased_serialize_struct_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
                wit_to_usize(len.val),
            )
            .map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)?;

        Ok(
            bindings::exports::serde::serde::serde_serializer::SerializeStructVariant::new(
                GuestsideSerializeStructVariantProvider {
                    serialize_struct_variant,
                    _scope: scope,
                },
            ),
        )
    }

    fn is_human_readable(
        this: bindings::exports::serde::serde::serde_serializer::SerializerBorrow,
    ) -> bool {
        this.get::<Self>().is_human_readable
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
                serializer,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
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
    fn erased_serialize_entry(
        &mut self,
        key: &SerializableSerialize,
        value: &SerializableSerialize,
    ) -> Result<(), SerError>;
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

    fn erased_serialize_entry(
        &mut self,
        key: &SerializableSerialize,
        value: &SerializableSerialize,
    ) -> Result<(), SerError> {
        self.serialize_entry(key, value).map_err(SerError::wrap)
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
    value: Any,
}

impl SerOk {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: unsafe { Any::new(value) },
        }
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerOk for SerOk {}

pub struct SerError {
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

impl bindings::exports::serde::serde::serde_serializer::GuestSerError for SerError {
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

    fn custom(msg: String) -> bindings::exports::serde::serde::serde_serializer::SerError {
        let error = Self {
            inner: SerErrorOrCustom::Custom(msg),
        };

        bindings::exports::serde::serde::serde_serializer::SerError::new(error)
    }
}

impl SerError {
    fn wrap<T: ::serde::ser::Error>(err: T) -> Self {
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
}

pub struct GuestsideSerializeSeqProvider {
    serialize_seq: Box<dyn ErasedSerializeSeq>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleProvider {
    serialize_tuple: Box<dyn ErasedSerializeTuple>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleStructProvider {
    serialize_tuple_struct: Box<dyn ErasedSerializeTupleStruct>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeTupleVariantProvider {
    serialize_tuple_variant: Box<dyn ErasedSerializeTupleVariant>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeMapProvider {
    serialize_map: Box<dyn ErasedSerializeMap>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeStructProvider {
    serialize_struct: Box<dyn ErasedSerializeStruct>,
    _scope: ScopedBorrowMut<()>,
}

pub struct GuestsideSerializeStructVariantProvider {
    serialize_struct_variant: Box<dyn ErasedSerializeStructVariant>,
    _scope: ScopedBorrowMut<()>,
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeSeq
    for GuestsideSerializeSeqProvider
{
    fn serialize_element(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeSeq,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeSeq,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_seq
            .erased_serialize_element(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeSeq,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>().serialize_seq.erased_end().wrap()
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeTuple
    for GuestsideSerializeTupleProvider
{
    fn serialize_element(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeTuple,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeTuple,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_tuple
            .erased_serialize_element(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeTuple,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>()
            .serialize_tuple
            .erased_end()
            .wrap()
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeTupleStruct
    for GuestsideSerializeTupleStructProvider
{
    fn serialize_field(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_tuple_struct
            .erased_serialize_field(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>()
            .serialize_tuple_struct
            .erased_end()
            .wrap()
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeTupleVariant
    for GuestsideSerializeTupleVariantProvider
{
    fn serialize_field(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_tuple_variant
            .erased_serialize_field(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>()
            .serialize_tuple_variant
            .erased_end()
            .wrap()
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeMap
    for GuestsideSerializeMapProvider
{
    fn serialize_key(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
        key: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeMap,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let key = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(key.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_map
            .erased_serialize_key(&SerializableSerialize::new(&key))
            .wrap();

        (this, result)
    }

    fn serialize_value(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeMap,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_map
            .erased_serialize_value(&SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn serialize_entry(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
        key: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeMap,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let key = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(key.borrowed_handle)
        };

        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_map
            .erased_serialize_entry(
                &SerializableSerialize::new(&key),
                &SerializableSerialize::new(&value),
            )
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>().serialize_map.erased_end().wrap()
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeStruct
    for GuestsideSerializeStructProvider
{
    fn serialize_field(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeStruct,
        key: String,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeStruct,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_struct
            .erased_serialize_field(intern_string(key), &SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeStruct,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>()
            .serialize_struct
            .erased_end()
            .wrap()
    }

    fn skip_field(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeStruct,
        key: String,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeStruct,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        let result = this
            .get_mut::<Self>()
            .serialize_struct
            .erased_skip_field(intern_string(key))
            .wrap();

        (this, result)
    }
}

impl bindings::exports::serde::serde::serde_serializer::GuestSerializeStructVariant
    for GuestsideSerializeStructVariantProvider
{
    fn serialize_field(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
        key: String,
        value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        // TODO: Safety
        let value = unsafe {
            bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
        };

        let result = this
            .get_mut::<Self>()
            .serialize_struct_variant
            .erased_serialize_field(intern_string(key), &SerializableSerialize::new(&value))
            .wrap();

        (this, result)
    }

    fn end(
        this: bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
    ) -> Result<
        bindings::exports::serde::serde::serde_serializer::SerOk,
        bindings::exports::serde::serde::serde_serializer::SerError,
    > {
        this.into_inner::<Self>()
            .serialize_struct_variant
            .erased_end()
            .wrap()
    }

    fn skip_field(
        mut this: bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
        key: String,
    ) -> (
        bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
        Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
    ) {
        let result = this
            .get_mut::<Self>()
            .serialize_struct_variant
            .erased_skip_field(intern_string(key))
            .wrap();

        (this, result)
    }
}

struct SerializableSerialize<'a> {
    serialize: &'a bindings::serde::serde::serde_serialize::Serialize,
}

impl<'a> SerializableSerialize<'a> {
    fn new(serialize: &'a bindings::serde::serde::serde_serialize::Serialize) -> Self {
        Self { serialize }
    }
}

impl<'a> ::serde::Serialize for SerializableSerialize<'a> {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let result = GuestsideSerializerProvider::with_new(serializer, |serializer| {
            let serializer =
                bindings::exports::serde::serde::serde_serializer::Serializer::new(serializer);
            let owned_handle = core::mem::ManuallyDrop::new(serializer).handle();
            let serializer =
                bindings::serde::serde::serde_serialize::OwnedSerializerHandle { owned_handle };
            self.serialize.serialize(serializer)
        });

        match result {
            Ok(value) => {
                // TODO: Safety
                let value = unsafe {
                    bindings::exports::serde::serde::serde_serializer::SerOk::from_handle(
                        value.owned_handle,
                    )
                };
                let SerOk { value } = value.into_inner();
                // TODO: Safety
                let Some(value): Option<S::Ok> = (unsafe { value.take() }) else {
                    return Err(::serde::ser::Error::custom(
                        "bug: Serializer::Ok type mismatch across the wit boundary",
                    ));
                };
                Ok(value)
            }
            Err(err) => {
                // TODO: Safety
                let err = unsafe {
                    bindings::exports::serde::serde::serde_serializer::SerError::from_handle(
                        err.owned_handle,
                    )
                };
                let SerError { inner: err } = err.into_inner();
                let err = match err {
                    SerErrorOrCustom::Error { err, .. } => err,
                    SerErrorOrCustom::Custom(msg) => return Err(::serde::ser::Error::custom(msg)),
                };
                // TODO: Safety
                let Some(err): Option<S::Error> = (unsafe { err.take() }) else {
                    return Err(::serde::ser::Error::custom(
                        "bug: Serializer::Error type mismatch across the wit boundary",
                    ));
                };
                Err(err)
            }
        }
    }
}
