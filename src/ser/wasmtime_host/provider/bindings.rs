#[doc(hidden)]
pub use super::HostsideSerializerProvider as __with_name2;
#[doc(hidden)]
pub use super::SerError as __with_name0;
#[doc(hidden)]
pub use super::SerOk as __with_name1;
pub struct SerdeSerializerClient {
    interface0: exports::serde::serde::serde_serialize::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;

    impl SerdeSerializerClient {
        // pub fn add_to_linker<T, U>(
        //     linker: &mut wasmtime::component::Linker<T>,
        //     get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        // ) -> wasmtime::Result<()>
        // where
        //     U: serde::serde::serde_types::Host /*+ serde::serde::serde_serializer::Host*/,
        // {
        //     serde::serde::serde_types::add_to_linker(linker, get)?;
        //     serde::serde::serde_serializer::add_to_linker(linker, get)?;
        //     Ok(())
        // }

        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }

        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();

            let interface0 = exports::serde::serde::serde_serialize::Guest::new(
                &mut __exports
                    .instance("serde:serde/serde-serialize")
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "exported instance `serde:serde/serde-serialize` not present"
                        )
                    })?,
            )?;
            Ok(SerdeSerializerClient { interface0 })
        }

        pub fn serde_serde_serde_serialize(
            &self,
        ) -> &exports::serde::serde::serde_serialize::Guest {
            &self.interface0
        }
    }
};
pub mod serde {
    pub mod serde {

        #[allow(clippy::all)]
        pub mod serde_types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;

            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct S128 {
                #[component(name = "le-hi")]
                pub le_hi: u64,
                #[component(name = "le-lo")]
                pub le_lo: u64,
            }
            impl core::fmt::Debug for S128 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("S128")
                        .field("le-hi", &self.le_hi)
                        .field("le-lo", &self.le_lo)
                        .finish()
                }
            }
            const _: () = {
                assert!(16 == <S128 as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <S128 as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct U128 {
                #[component(name = "le-hi")]
                pub le_hi: u64,
                #[component(name = "le-lo")]
                pub le_lo: u64,
            }
            impl core::fmt::Debug for U128 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("U128")
                        .field("le-hi", &self.le_hi)
                        .field("le-lo", &self.le_lo)
                        .finish()
                }
            }
            const _: () = {
                assert!(16 == <U128 as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <U128 as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct Usize {
                #[component(name = "val")]
                pub val: u32,
            }
            impl core::fmt::Debug for Usize {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Usize").field("val", &self.val).finish()
                }
            }
            const _: () = {
                assert!(4 == <Usize as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <Usize as wasmtime::component::ComponentType>::ALIGN32);
            };
            pub trait Host {}

            pub trait GetHost<T>:
                Fn(T) -> <Self as GetHost<T>>::Host + Send + Sync + Copy + 'static
            {
                type Host: Host;
            }

            impl<F, T, O> GetHost<T> for F
            where
                F: Fn(T) -> O + Send + Sync + Copy + 'static,
                O: Host,
            {
                type Host = O;
            }

            pub fn add_to_linker_get_host<T>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: impl for<'a> GetHost<&'a mut T>,
            ) -> wasmtime::Result<()>
where {
                let mut inst = linker.instance("serde:serde/serde-types")?;
                Ok(())
            }

            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                add_to_linker_get_host(linker, get)
            }

            impl<_T: Host + ?Sized> Host for &mut _T {}
        }

        #[allow(clippy::all)]
        pub mod serde_serializer {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;

            pub type S128 = super::super::super::serde::serde::serde_types::S128;
            const _: () = {
                assert!(16 == <S128 as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <S128 as wasmtime::component::ComponentType>::ALIGN32);
            };
            pub type Usize = super::super::super::serde::serde::serde_types::Usize;
            const _: () = {
                assert!(4 == <Usize as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <Usize as wasmtime::component::ComponentType>::ALIGN32);
            };
            pub type U128 = super::super::super::serde::serde::serde_types::U128;
            const _: () = {
                assert!(16 == <U128 as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <U128 as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct BorrowedSerializeHandle {
                #[component(name = "borrowed-handle")]
                pub borrowed_handle: u32,
            }
            impl core::fmt::Debug for BorrowedSerializeHandle {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("BorrowedSerializeHandle")
                        .field("borrowed-handle", &self.borrowed_handle)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    4 == <BorrowedSerializeHandle as wasmtime::component::ComponentType>::SIZE32
                );
                assert!(
                    4 == <BorrowedSerializeHandle as wasmtime::component::ComponentType>::ALIGN32
                );
            };
            use crate::ser::wasmtime_host::provider::WrapSerResult;

            pub use super::super::super::__with_name2 as Serializer;
            /*pub trait HostSerializer {
                fn serialize_bool(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: bool,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_i8(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i8,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_i16(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i16,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_i32(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i32,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_i64(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i64,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_i128(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: S128,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_u8(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u8,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_u16(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u16,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_u32(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u32,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_u64(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u64,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_u128(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: U128,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_f32(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: f32,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_f64(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: f64,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_char(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: char,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_str(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_bytes(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: wasmtime::component::__internal::Vec<u8>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_none(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_some<T>(
                    &mut self,
                    ctx: *mut wasmtime::StoreContextMut<'_, T>,
                    self_: wasmtime::component::Resource<Serializer>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_unit(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_unit_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_unit_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_newtype_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_newtype_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_seq(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    len: Option<Usize>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeSeq>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_tuple(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeTuple>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_tuple_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeTupleStruct>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_tuple_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeTupleVariant>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_map(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    len: Option<Usize>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeMap>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeStruct>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn serialize_struct_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeStructVariant>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn is_human_readable(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<bool>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializer + ?Sized> HostSerializer for &mut _T {
                fn serialize_bool(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: bool,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_bool(*self, self_, v)
                }
                fn serialize_i8(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i8,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_i8(*self, self_, v)
                }
                fn serialize_i16(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i16,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_i16(*self, self_, v)
                }
                fn serialize_i32(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i32,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_i32(*self, self_, v)
                }
                fn serialize_i64(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: i64,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_i64(*self, self_, v)
                }
                fn serialize_i128(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: S128,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_i128(*self, self_, v)
                }
                fn serialize_u8(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u8,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_u8(*self, self_, v)
                }
                fn serialize_u16(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u16,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_u16(*self, self_, v)
                }
                fn serialize_u32(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u32,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_u32(*self, self_, v)
                }
                fn serialize_u64(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: u64,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_u64(*self, self_, v)
                }
                fn serialize_u128(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: U128,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_u128(*self, self_, v)
                }
                fn serialize_f32(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: f32,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_f32(*self, self_, v)
                }
                fn serialize_f64(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: f64,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_f64(*self, self_, v)
                }
                fn serialize_char(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: char,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_char(*self, self_, v)
                }
                fn serialize_str(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_str(*self, self_, v)
                }
                fn serialize_bytes(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    v: wasmtime::component::__internal::Vec<u8>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_bytes(*self, self_, v)
                }
                fn serialize_none(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_none(*self, self_)
                }
                fn serialize_some<T>(
                    &mut self,
                    ctx: *mut wasmtime::StoreContextMut<'_, T>,
                    self_: wasmtime::component::Resource<Serializer>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    <_T as HostSerializer>::serialize_some(*self, ctx, self_, value)
                }
                fn serialize_unit(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_unit(*self, self_)
                }
                fn serialize_unit_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_unit_struct(*self, self_, name)
                }
                fn serialize_unit_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_unit_variant(
                        *self,
                        self_,
                        name,
                        variant_index,
                        variant,
                    )
                }
                fn serialize_newtype_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_newtype_struct(*self, self_, name, value)
                }
                fn serialize_newtype_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_newtype_variant(
                        *self,
                        self_,
                        name,
                        variant_index,
                        variant,
                        value,
                    )
                }
                fn serialize_seq(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    len: Option<Usize>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeSeq>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_seq(*self, self_, len)
                }
                fn serialize_tuple(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeTuple>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_tuple(*self, self_, len)
                }
                fn serialize_tuple_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeTupleStruct>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_tuple_struct(*self, self_, name, len)
                }
                fn serialize_tuple_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeTupleVariant>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_tuple_variant(
                        *self,
                        self_,
                        name,
                        variant_index,
                        variant,
                        len,
                    )
                }
                fn serialize_map(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    len: Option<Usize>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeMap>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_map(*self, self_, len)
                }
                fn serialize_struct(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeStruct>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_struct(*self, self_, name, len)
                }
                fn serialize_struct_variant(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                    name: wasmtime::component::__internal::String,
                    variant_index: u32,
                    variant: wasmtime::component::__internal::String,
                    len: Usize,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerializeStructVariant>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializer::serialize_struct_variant(
                        *self,
                        self_,
                        name,
                        variant_index,
                        variant,
                        len,
                    )
                }
                fn is_human_readable(
                    &mut self,
                    self_: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<bool> {
                    HostSerializer::is_human_readable(*self, self_)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Serializer>,
                ) -> wasmtime::Result<()> {
                    HostSerializer::drop(*self, rep)
                }
            }*/
            pub use super::super::super::__with_name1 as SerOk;
            pub trait HostSerOk {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerOk>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerOk + ?Sized> HostSerOk for &mut _T {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerOk>,
                ) -> wasmtime::Result<()> {
                    HostSerOk::drop(*self, rep)
                }
            }
            pub use super::super::super::__with_name0 as SerError;
            pub trait HostSerError {
                fn display(
                    &mut self,
                    self_: wasmtime::component::Resource<SerError>,
                ) -> wasmtime::Result<wasmtime::component::__internal::String>;
                fn debug(
                    &mut self,
                    self_: wasmtime::component::Resource<SerError>,
                ) -> wasmtime::Result<wasmtime::component::__internal::String>;
                fn custom(
                    &mut self,
                    msg: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<wasmtime::component::Resource<SerError>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerError>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerError + ?Sized> HostSerError for &mut _T {
                fn display(
                    &mut self,
                    self_: wasmtime::component::Resource<SerError>,
                ) -> wasmtime::Result<wasmtime::component::__internal::String> {
                    HostSerError::display(*self, self_)
                }
                fn debug(
                    &mut self,
                    self_: wasmtime::component::Resource<SerError>,
                ) -> wasmtime::Result<wasmtime::component::__internal::String> {
                    HostSerError::debug(*self, self_)
                }
                fn custom(
                    &mut self,
                    msg: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<wasmtime::component::Resource<SerError>> {
                    HostSerError::custom(*self, msg)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerError>,
                ) -> wasmtime::Result<()> {
                    HostSerError::drop(*self, rep)
                }
            }
            pub enum SerializeSeq {}
            pub trait HostSerializeSeq {
                fn serialize_element(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeSeq>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeSeq>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeSeq>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeSeq>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeSeq + ?Sized> HostSerializeSeq for &mut _T {
                fn serialize_element(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeSeq>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeSeq>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeSeq::serialize_element(*self, self_, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeSeq>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeSeq::end(*self, self_)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeSeq>,
                ) -> wasmtime::Result<()> {
                    HostSerializeSeq::drop(*self, rep)
                }
            }
            pub enum SerializeTuple {}
            pub trait HostSerializeTuple {
                fn serialize_element(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTuple>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeTuple>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTuple>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeTuple>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeTuple + ?Sized> HostSerializeTuple for &mut _T {
                fn serialize_element(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTuple>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeTuple>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeTuple::serialize_element(*self, self_, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTuple>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeTuple::end(*self, self_)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeTuple>,
                ) -> wasmtime::Result<()> {
                    HostSerializeTuple::drop(*self, rep)
                }
            }
            pub enum SerializeTupleStruct {}
            pub trait HostSerializeTupleStruct {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleStruct>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeTupleStruct>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleStruct>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeTupleStruct>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeTupleStruct + ?Sized> HostSerializeTupleStruct for &mut _T {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleStruct>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeTupleStruct>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeTupleStruct::serialize_field(*self, self_, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleStruct>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeTupleStruct::end(*self, self_)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeTupleStruct>,
                ) -> wasmtime::Result<()> {
                    HostSerializeTupleStruct::drop(*self, rep)
                }
            }
            pub enum SerializeTupleVariant {}
            pub trait HostSerializeTupleVariant {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleVariant>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeTupleVariant>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleVariant>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeTupleVariant>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeTupleVariant + ?Sized> HostSerializeTupleVariant for &mut _T {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleVariant>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeTupleVariant>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeTupleVariant::serialize_field(*self, self_, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeTupleVariant>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeTupleVariant::end(*self, self_)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeTupleVariant>,
                ) -> wasmtime::Result<()> {
                    HostSerializeTupleVariant::drop(*self, rep)
                }
            }
            pub enum SerializeMap {}
            pub trait HostSerializeMap {
                fn serialize_key(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeMap>,
                    key: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeMap>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn serialize_value(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeMap>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeMap>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeMap>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeMap>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeMap + ?Sized> HostSerializeMap for &mut _T {
                fn serialize_key(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeMap>,
                    key: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeMap>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeMap::serialize_key(*self, self_, key)
                }
                fn serialize_value(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeMap>,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeMap>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeMap::serialize_value(*self, self_, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeMap>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeMap::end(*self, self_)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeMap>,
                ) -> wasmtime::Result<()> {
                    HostSerializeMap::drop(*self, rep)
                }
            }
            pub enum SerializeStruct {}
            pub trait HostSerializeStruct {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStruct>,
                    key: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStruct>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStruct>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn skip_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStruct>,
                    key: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStruct>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeStruct>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeStruct + ?Sized> HostSerializeStruct for &mut _T {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStruct>,
                    key: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStruct>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeStruct::serialize_field(*self, self_, key, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStruct>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeStruct::end(*self, self_)
                }
                fn skip_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStruct>,
                    key: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStruct>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeStruct::skip_field(*self, self_, key)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeStruct>,
                ) -> wasmtime::Result<()> {
                    HostSerializeStruct::drop(*self, rep)
                }
            }
            pub enum SerializeStructVariant {}
            pub trait HostSerializeStructVariant {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStructVariant>,
                    key: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStructVariant>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStructVariant>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                >;
                fn skip_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStructVariant>,
                    key: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStructVariant>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeStructVariant>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostSerializeStructVariant + ?Sized> HostSerializeStructVariant for &mut _T {
                fn serialize_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStructVariant>,
                    key: wasmtime::component::__internal::String,
                    value: BorrowedSerializeHandle,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStructVariant>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeStructVariant::serialize_field(*self, self_, key, value)
                }
                fn end(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStructVariant>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<SerOk>,
                        wasmtime::component::Resource<SerError>,
                    >,
                > {
                    HostSerializeStructVariant::end(*self, self_)
                }
                fn skip_field(
                    &mut self,
                    self_: wasmtime::component::Resource<SerializeStructVariant>,
                    key: wasmtime::component::__internal::String,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<SerializeStructVariant>,
                    Result<(), wasmtime::component::Resource<SerError>>,
                )> {
                    HostSerializeStructVariant::skip_field(*self, self_, key)
                }

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SerializeStructVariant>,
                ) -> wasmtime::Result<()> {
                    HostSerializeStructVariant::drop(*self, rep)
                }
            }
            /*pub trait Host:
                /* HostSerializer
                +*/ HostSerOk
                + HostSerError
                + HostSerializeSeq
                + HostSerializeTuple
                + HostSerializeTupleStruct
                + HostSerializeTupleVariant
                + HostSerializeMap
                + HostSerializeStruct
                + HostSerializeStructVariant
            {
            }*/

            pub trait GetHost<T>:
                for<'a> Fn(&'a mut T) -> /*<Self as GetHost<T>>::Host*/&'a mut crate::ser::wasmtime_host::provider::HostsideSerializerProviderState + Send + Sync + Copy + 'static
            {
                // type Host: Host;
            }

            impl<F, T/*, O*/> GetHost<T> for F
            where
                F: for<'a> Fn(&'a mut T) -> /*O*/&'a mut crate::ser::wasmtime_host::provider::HostsideSerializerProviderState + Send + Sync + Copy + 'static,
                // O: Host,
            {
                // type Host = O;
            }

            pub fn add_to_linker_get_host<T>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: impl /*for<'a>*/ GetHost</*&'a mut*/ T>,
            ) -> wasmtime::Result<()>
where {
                let mut inst = linker.instance("serde:serde/serde-serializer")?;
                inst.resource(
                    "serializer",
                    wasmtime::component::ResourceType::host::<Serializer>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::new_own(rep);
                        let _serializer = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                // inst.resource(
                //     "ser-ok",
                //     wasmtime::component::ResourceType::host::<SerOk>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerOk::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "ser-error",
                //     wasmtime::component::ResourceType::host::<SerError>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerError::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-seq",
                //     wasmtime::component::ResourceType::host::<SerializeSeq>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeSeq::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-tuple",
                //     wasmtime::component::ResourceType::host::<SerializeTuple>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeTuple::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-tuple-struct",
                //     wasmtime::component::ResourceType::host::<SerializeTupleStruct>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeTupleStruct::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-tuple-variant",
                //     wasmtime::component::ResourceType::host::<SerializeTupleVariant>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeTupleVariant::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-map",
                //     wasmtime::component::ResourceType::host::<SerializeMap>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeMap::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-struct",
                //     wasmtime::component::ResourceType::host::<SerializeStruct>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeStruct::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                // inst.resource(
                //     "serialize-struct-variant",
                //     wasmtime::component::ResourceType::host::<SerializeStructVariant>(),
                //     move |mut store, rep| -> wasmtime::Result<()> {
                //         HostSerializeStructVariant::drop(
                //             host_getter(store.data_mut()),
                //             wasmtime::component::Resource::new_own(rep),
                //         )
                //     },
                // )?;
                inst.func_wrap("[static]serializer.serialize-bool", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, bool, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_bool(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i8", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i8, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_i8(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i16", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i16, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_i16(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i32", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i32, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_i32(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i64", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i64, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_i64(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i128", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, S128, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let le_hi = v.le_hi.to_le_bytes();
                    let le_lo = v.le_lo.to_le_bytes();
                    let bytes = [
                        le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
                        le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
                    ];
                    let r = serializer
                        .serializer
                        .erased_serialize_i128(i128::from_le_bytes(bytes))
                        .wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u8", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u8, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_u8(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u16", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u16, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_u16(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u32", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u32, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_u32(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u64", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u64, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_u64(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u128", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, U128, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let le_hi = v.le_hi.to_le_bytes();
                    let le_lo = v.le_lo.to_le_bytes();
                    let bytes = [
                        le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
                        le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
                    ];
                    let r = serializer
                        .serializer
                        .erased_serialize_u128(u128::from_le_bytes(bytes))
                        .wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-f32", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, f32, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_f32(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-f64", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, f64, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_f64(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-char", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, char, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_char(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serializer.serialize-str",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, v) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let serializer = host.table.delete(this)?;
                        let r = serializer.serializer.erased_serialize_str(&v).wrap(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-bytes",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::Vec<u8>,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, v) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let serializer = host.table.delete(this)?;
                        let r = serializer.serializer.erased_serialize_bytes(&v).wrap(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[static]serializer.serialize-none", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<Serializer>, ) | { 
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_none().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serializer.serialize-some",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        BorrowedSerializeHandle,
                    )| {
                        // let caller = &mut caller;
                        // // TODO: store ctx in SerializableSerialize instead of host
                        // let ctx = core::ptr::from_mut(caller);
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let serializer = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = serializer
                            .serializer
                            .erased_serialize_some(&crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &mut caller,
                                host_getter,
                                Box::new(move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                }),
                                &value,
                            ));
                        let r = {
                            let host = host_getter(caller.data_mut());
                            r.wrap(host)
                        };
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[static]serializer.serialize-unit", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<Serializer>, ) | { 
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.erased_serialize_unit().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serializer.serialize-unit-struct",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, name) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let serializer = host.table.delete(this)?;
                        let r = serializer.serializer.erased_serialize_unit_struct(crate::intern::intern_string(name)).wrap(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-unit-variant",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        u32,
                        wasmtime::component::__internal::String,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, name, variant_index, variant) = (arg0, arg1, arg2, arg3);
                        anyhow::ensure!(this.owned());
                        let serializer = host.table.delete(this)?;
                        let r = serializer.serializer.erased_serialize_unit_variant(crate::intern::intern_string(name), variant_index, crate::intern::intern_string(variant)).wrap(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-newtype-struct",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        BorrowedSerializeHandle,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_newtype_struct(host, arg0, arg1, arg2);
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-newtype-variant",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3, arg4): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        u32,
                        wasmtime::component::__internal::String,
                        BorrowedSerializeHandle,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_newtype_variant(
                        //     host, arg0, arg1, arg2, arg3, arg4,
                        // );
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-seq",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        Option<Usize>,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_seq(host, arg0, arg1);
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap("[static]serializer.serialize-tuple", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, Usize, ) | { 
                    // let host = host_getter(caller.data_mut());
                    // let r = HostSerializer::serialize_tuple(host, arg0,arg1,);
                    // Ok((r?,))
                    todo!()
                })?;
                inst.func_wrap(
                    "[static]serializer.serialize-tuple-struct",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        Usize,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_tuple_struct(host, arg0, arg1, arg2);
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-tuple-variant",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3, arg4): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        u32,
                        wasmtime::component::__internal::String,
                        Usize,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_tuple_variant(
                        //     host, arg0, arg1, arg2, arg3, arg4,
                        // );
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-map",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        Option<Usize>,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_map(host, arg0, arg1);
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-struct",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        Usize,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_struct(host, arg0, arg1, arg2);
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-struct-variant",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3, arg4): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        u32,
                        wasmtime::component::__internal::String,
                        Usize,
                    )| {
                        // let host = host_getter(caller.data_mut());
                        // let r = HostSerializer::serialize_struct_variant(
                        //     host, arg0, arg1, arg2, arg3, arg4,
                        // );
                        // Ok((r?,))
                        todo!()
                    },
                )?;
                inst.func_wrap("[static]serializer.is-human-readable", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<Serializer>, ) | { 
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(!this.owned());
                    let serializer = host.table.get(&this)?;
                    let r = serializer.serializer.erased_is_human_readable();
                    Ok((r,))
                })?;
                // inst.func_wrap(
                //     "[method]ser-error.display",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0,): (wasmtime::component::Resource<SerError>,)| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerError::display(host, arg0);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap(
                //     "[method]ser-error.debug",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0,): (wasmtime::component::Resource<SerError>,)| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerError::debug(host, arg0);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap(
                //     "[static]ser-error.custom",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0,): (wasmtime::component::__internal::String,)| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerError::custom(host, arg0);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap(
                //     "[static]serialize-seq.serialize-element",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeSeq>,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeSeq::serialize_element(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-seq.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeSeq>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeSeq::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-tuple.serialize-element",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeTuple>,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeTuple::serialize_element(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-tuple.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeTuple>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeTuple::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-tuple-struct.serialize-field",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeTupleStruct>,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeTupleStruct::serialize_field(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-tuple-struct.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeTupleStruct>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeTupleStruct::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-tuple-variant.serialize-field",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeTupleVariant>,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeTupleVariant::serialize_field(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-tuple-variant.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeTupleVariant>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeTupleVariant::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-map.serialize-key",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeMap>,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeMap::serialize_key(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap(
                //     "[static]serialize-map.serialize-value",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeMap>,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeMap::serialize_value(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-map.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeMap>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeMap::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-struct.serialize-field",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1, arg2): (
                //         wasmtime::component::Resource<SerializeStruct>,
                //         wasmtime::component::__internal::String,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeStruct::serialize_field(host, arg0, arg1, arg2);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-struct.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeStruct>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeStruct::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-struct.skip-field",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeStruct>,
                //         wasmtime::component::__internal::String,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeStruct::skip_field(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap(
                //     "[static]serialize-struct-variant.serialize-field",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1, arg2): (
                //         wasmtime::component::Resource<SerializeStructVariant>,
                //         wasmtime::component::__internal::String,
                //         BorrowedSerializeHandle,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeStructVariant::serialize_field(host, arg0, arg1, arg2);
                //         Ok((r?,))
                //     },
                // )?;
                // inst.func_wrap("[static]serialize-struct-variant.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeStructVariant>, ) | { 
                //               let host = host_getter(caller.data_mut());
                //               let r = HostSerializeStructVariant::end(host, arg0,);
                //               Ok((r?,))
                //             })?;
                // inst.func_wrap(
                //     "[static]serialize-struct-variant.skip-field",
                //     move |mut caller: wasmtime::StoreContextMut<'_, T>,
                //           (arg0, arg1): (
                //         wasmtime::component::Resource<SerializeStructVariant>,
                //         wasmtime::component::__internal::String,
                //     )| {
                //         let host = host_getter(caller.data_mut());
                //         let r = HostSerializeStructVariant::skip_field(host, arg0, arg1);
                //         Ok((r?,))
                //     },
                // )?;
                Ok(())
            }

            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut /*U*/ crate::ser::wasmtime_host::provider::HostsideSerializerProviderState + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                // U: Host,
            {
                add_to_linker_get_host(linker, get)
            }

            // impl<_T: Host + ?Sized> Host for &mut _T {}
        }
    }
}
pub mod exports {
    pub mod serde {
        pub mod serde {

            #[allow(clippy::all)]
            pub mod serde_serialize {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;

                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct OwnedSerializerHandle {
                    #[component(name = "owned-handle")]
                    pub owned_handle: u32,
                }
                impl core::fmt::Debug for OwnedSerializerHandle {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("OwnedSerializerHandle")
                            .field("owned-handle", &self.owned_handle)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        4 == <OwnedSerializerHandle as wasmtime::component::ComponentType>::SIZE32
                    );
                    assert!(
                        4 == <OwnedSerializerHandle as wasmtime::component::ComponentType>::ALIGN32
                    );
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct OwnedSerOkHandle {
                    #[component(name = "owned-handle")]
                    pub owned_handle: u32,
                }
                impl core::fmt::Debug for OwnedSerOkHandle {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("OwnedSerOkHandle")
                            .field("owned-handle", &self.owned_handle)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(4 == <OwnedSerOkHandle as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <OwnedSerOkHandle as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct OwnedSerErrorHandle {
                    #[component(name = "owned-handle")]
                    pub owned_handle: u32,
                }
                impl core::fmt::Debug for OwnedSerErrorHandle {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("OwnedSerErrorHandle")
                            .field("owned-handle", &self.owned_handle)
                            .finish()
                    }
                }
                impl core::fmt::Display for OwnedSerErrorHandle {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl std::error::Error for OwnedSerErrorHandle {}
                const _: () = {
                    assert!(
                        4 == <OwnedSerErrorHandle as wasmtime::component::ComponentType>::SIZE32
                    );
                    assert!(
                        4 == <OwnedSerErrorHandle as wasmtime::component::ComponentType>::ALIGN32
                    );
                };

                pub type Serialize = wasmtime::component::ResourceAny;

                pub struct GuestSerialize<'a> {
                    funcs: &'a Guest,
                }

                pub struct Guest {
                    method_serialize_serialize: wasmtime::component::Func,
                }
                impl Guest {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Guest> {
                        let method_serialize_serialize = *__exports.typed_func::<(wasmtime::component::ResourceAny, OwnedSerializerHandle, ), (Result<OwnedSerOkHandle,OwnedSerErrorHandle>, )>("[method]serialize.serialize")?.func();
                        Ok(Guest {
                            method_serialize_serialize,
                        })
                    }
                    pub fn serialize(&self) -> GuestSerialize<'_> {
                        GuestSerialize { funcs: self }
                    }
                }
                impl GuestSerialize<'_> {
                    pub fn call_serialize<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                        arg1: OwnedSerializerHandle,
                    ) -> wasmtime::Result<Result<OwnedSerOkHandle, OwnedSerErrorHandle>>
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny, OwnedSerializerHandle),
                                (Result<OwnedSerOkHandle, OwnedSerErrorHandle>,),
                            >::new_unchecked(
                                self.funcs.method_serialize_serialize
                            )
                        };
                        let (ret0,) = callee.call(store.as_context_mut(), (arg0, arg1))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}
