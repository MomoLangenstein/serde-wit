use std::sync::Mutex;

use ::serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use scoped_reference::{ScopedBorrowMut, ScopedReference};
use send_wrapper::SendWrapper;

mod bindings;
// mod bindings {
//     wasmtime::component::bindgen!({
//         world: "serde-serializer-client",
//         trappable_imports: true,
//         // include_generated_code_from_file: true,
//     });
// }

pub use bindings::serde::serde::serde_serializer::{add_to_linker, add_to_linker_get_host};

use crate::any::Any;

#[derive(Default)]
pub struct HostsideSerializerProviderState {
    table: wasmtime::component::ResourceTable,
}

impl HostsideSerializerProviderState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct HostsideSerializerProvider {
    serializer: SendWrapper<Box<dyn ErasedSerializer>>,
    scope: ScopedBorrowMut<()>,
}

trait WrapSerResult {
    type Ok;

    fn wrap(
        self,
        state: &mut HostsideSerializerProviderState,
    ) -> anyhow::Result<Result<Self::Ok, wasmtime::component::Resource<SerError>>>;
}

impl WrapSerResult for Result<SerOk, SerError> {
    type Ok = wasmtime::component::Resource<SerOk>;

    fn wrap(
        self,
        state: &mut HostsideSerializerProviderState,
    ) -> anyhow::Result<
        Result<wasmtime::component::Resource<SerOk>, wasmtime::component::Resource<SerError>>,
    > {
        match self {
            Ok(ok) => Ok(Ok(state.table.push(ok)?)),
            Err(error) => Ok(Err(state.table.push(error)?)),
        }
    }
}

impl WrapSerResult for Result<(), SerError> {
    type Ok = ();

    fn wrap(
        self,
        state: &mut HostsideSerializerProviderState,
    ) -> anyhow::Result<Result<(), wasmtime::component::Resource<SerError>>> {
        match self {
            Ok(()) => Ok(Ok(())),
            Err(error) => Ok(Err(state.table.push(error)?)),
        }
    }
}

impl HostsideSerializerProvider {
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
                serializer: SendWrapper::new(serializer),
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

struct SerOk {
    value: SendWrapper<Any>,
}

impl SerOk {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: SendWrapper::new(unsafe { Any::new(value) }),
        }
    }
}

struct SerError {
    inner: SerErrorOrCustom,
}

enum SerErrorOrCustom {
    Error {
        err: SendWrapper<Any>,
        display: String,
        debug: String,
    },
    Custom(String),
}

impl SerError {
    fn wrap<T: ::serde::ser::Error>(err: T) -> Self {
        let display = format!("{err}");
        let debug = format!("{err:?}");

        // Safety: TODO
        Self {
            inner: SerErrorOrCustom::Error {
                err: SendWrapper::new(unsafe { Any::new(err) }),
                display,
                debug,
            },
        }
    }
}

struct HostsideSerializeSeqProvider {
    serialize_seq: SendWrapper<Box<dyn ErasedSerializeSeq>>,
    _scope: ScopedBorrowMut<()>,
}

struct HostsideSerializeTupleProvider {
    serialize_tuple: SendWrapper<Box<dyn ErasedSerializeTuple>>,
    _scope: ScopedBorrowMut<()>,
}

struct HostsideSerializeTupleStructProvider {
    serialize_tuple_struct: SendWrapper<Box<dyn ErasedSerializeTupleStruct>>,
    _scope: ScopedBorrowMut<()>,
}

struct HostsideSerializeTupleVariantProvider {
    serialize_tuple_variant: SendWrapper<Box<dyn ErasedSerializeTupleVariant>>,
    _scope: ScopedBorrowMut<()>,
}

struct HostsideSerializeMapProvider {
    serialize_map: SendWrapper<Box<dyn ErasedSerializeMap>>,
    _scope: ScopedBorrowMut<()>,
}

struct HostsideSerializeStructProvider {
    serialize_struct: SendWrapper<Box<dyn ErasedSerializeStruct>>,
    _scope: ScopedBorrowMut<()>,
}

struct HostsideSerializeStructVariantProvider {
    serialize_struct_variant: SendWrapper<Box<dyn ErasedSerializeStructVariant>>,
    _scope: ScopedBorrowMut<()>,
}

trait SerializeWithContext {
    fn serialize(
        &self,
        guest: &bindings::exports::serde::serde::serde_serialize::GuestSerialize,
        serialize: wasmtime::component::Resource<GuestSerialize>,
        serializer: bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
    ) -> anyhow::Result<
        Result<
            bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle,
            bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle,
        >,
    >;

    fn new_serializer_resource(
        &self,
        serializer: HostsideSerializerProvider,
    ) -> anyhow::Result<wasmtime::component::Resource<HostsideSerializerProvider>>;

    fn delete_ser_ok_resource(
        &self,
        ok: wasmtime::component::Resource<SerOk>,
    ) -> anyhow::Result<SerOk>;

    fn delete_ser_error_resource(
        &self,
        error: wasmtime::component::Resource<SerError>,
    ) -> anyhow::Result<SerError>;
}

struct SpecificSerializeWithContext<
    'a,
    'b,
    T,
    H: bindings::serde::serde::serde_serializer::GetHost<T>,
    S: Fn(
        &mut wasmtime::StoreContextMut<T>,
        &bindings::exports::serde::serde::serde_serialize::GuestSerialize,
        wasmtime::component::Resource<GuestSerialize>,
        bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
    ) -> anyhow::Result<
        Result<
            bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle,
            bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle,
        >,
    >,
> {
    ctx: &'b Mutex<wasmtime::StoreContextMut<'a, T>>,
    host_getter: H,
    do_serialize: S,
}

impl<
        'a,
        'b,
        T,
        H: bindings::serde::serde::serde_serializer::GetHost<T>,
        S: Fn(
            &mut wasmtime::StoreContextMut<T>,
            &bindings::exports::serde::serde::serde_serialize::GuestSerialize,
            wasmtime::component::Resource<GuestSerialize>,
            bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
        ) -> anyhow::Result<
            Result<
                bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle,
                bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle,
            >,
        >,
    > SerializeWithContext for SpecificSerializeWithContext<'a, 'b, T, H, S>
{
    fn serialize(
        &self,
        guest: &bindings::exports::serde::serde::serde_serialize::GuestSerialize,
        serialize: wasmtime::component::Resource<GuestSerialize>,
        serializer: bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
    ) -> anyhow::Result<
        Result<
            bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle,
            bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle,
        >,
    > {
        let mut ctx = self
            .ctx
            .lock()
            .map_err(|_| anyhow::anyhow!("SerializableSerialize should not be poisoned"))?;
        (self.do_serialize)(&mut *ctx, guest, serialize, serializer)
    }

    fn new_serializer_resource(
        &self,
        serializer: HostsideSerializerProvider,
    ) -> anyhow::Result<wasmtime::component::Resource<HostsideSerializerProvider>> {
        let mut ctx = self
            .ctx
            .lock()
            .map_err(|_| anyhow::anyhow!("SerializableSerialize should not be poisoned"))?;
        let host = (self.host_getter)(ctx.data_mut());
        host.table
            .push(serializer)
            .map_err(|err| anyhow::anyhow!("bug: failed to create a Serializer resource: {err}"))
    }

    fn delete_ser_ok_resource(
        &self,
        ok: wasmtime::component::Resource<SerOk>,
    ) -> anyhow::Result<SerOk> {
        let mut ctx = self
            .ctx
            .lock()
            .map_err(|_| anyhow::anyhow!("SerializableSerialize should not be poisoned"))?;
        let host = (self.host_getter)(ctx.data_mut());
        host.table
            .delete(ok)
            .map_err(|err| anyhow::anyhow!("bug: invalid Serializer::Ok handle: {err}"))
    }

    fn delete_ser_error_resource(
        &self,
        error: wasmtime::component::Resource<SerError>,
    ) -> anyhow::Result<SerError> {
        let mut ctx = self
            .ctx
            .lock()
            .map_err(|_| anyhow::anyhow!("SerializableSerialize should not be poisoned"))?;
        let host = (self.host_getter)(ctx.data_mut());
        host.table
            .delete(error)
            .map_err(|err| anyhow::anyhow!("bug: invalid Serializer::Error handle: {err}"))
    }
}

struct SerializableSerialize<'a> {
    serialize: Box<dyn SerializeWithContext + 'a>,
    borrowed_serialize_handle: u32,
    _borrow: core::marker::PhantomData<&'a GuestSerialize>,
}

enum GuestSerialize {}

impl<'b> SerializableSerialize<'b> {
    fn new<
        'a: 'b,
        T,
        H: bindings::serde::serde::serde_serializer::GetHost<T>,
        S: 'b
            + Fn(
                &mut wasmtime::StoreContextMut<T>,
                &bindings::exports::serde::serde::serde_serialize::GuestSerialize,
                wasmtime::component::Resource<GuestSerialize>,
                bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle,
            ) -> anyhow::Result<
                Result<
                    bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle,
                    bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle,
                >,
            >,
    >(
        ctx: &'b Mutex<wasmtime::StoreContextMut<'a, T>>,
        host_getter: H,
        do_serialize: S,
        serialize: &'b bindings::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> Self {
        Self {
            serialize: Box::new(SpecificSerializeWithContext {
                ctx,
                host_getter,
                do_serialize,
            }),
            borrowed_serialize_handle: serialize.borrowed_handle,
            _borrow: core::marker::PhantomData::<&'a GuestSerialize>,
        }
    }
}

impl<'a> ::serde::Serialize for SerializableSerialize<'a> {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let result =
            HostsideSerializerProvider::with_new(serializer, |serializer| -> anyhow::Result<_> {
                let serializer = self.serialize.new_serializer_resource(serializer)?;
                let serializer =
                    bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle {
                        owned_handle: serializer.rep(),
                    };
                #[allow(clippy::todo)] // FIXME
                let guest = todo!();
                self.serialize.serialize(
                    &guest,
                    wasmtime::component::Resource::new_borrow(self.borrowed_serialize_handle),
                    serializer,
                )
            })
            .map_err(::serde::ser::Error::custom)?;

        match result {
            Ok(value) => {
                let SerOk { value } = self
                    .serialize
                    .delete_ser_ok_resource(wasmtime::component::Resource::new_own(
                        value.owned_handle,
                    ))
                    .map_err(::serde::ser::Error::custom)?;
                // TODO: Safety
                let Some(value): Option<S::Ok> = (unsafe { value.take().take() }) else {
                    return Err(::serde::ser::Error::custom(
                        "bug: Serializer::Ok type mismatch across the wit boundary",
                    ));
                };
                Ok(value)
            }
            Err(err) => {
                // TODO: Safety
                let SerError { inner: err } = self
                    .serialize
                    .delete_ser_error_resource(wasmtime::component::Resource::new_own(
                        err.owned_handle,
                    ))
                    .map_err(::serde::ser::Error::custom)?;
                let err = match err {
                    SerErrorOrCustom::Error { err, .. } => err,
                    SerErrorOrCustom::Custom(msg) => return Err(::serde::ser::Error::custom(msg)),
                };
                // TODO: Safety
                let Some(err): Option<S::Error> = (unsafe { err.take().take() }) else {
                    return Err(::serde::ser::Error::custom(
                        "bug: Serializer::Error type mismatch across the wit boundary",
                    ));
                };
                Err(err)
            }
        }
    }
}
