#![allow(clippy::module_inception)] // FIXME
#![allow(clippy::indexing_slicing)] // FIXME
#![allow(clippy::todo)] // FIXME

#[doc(hidden)]
pub use super::HostsideSerializeSeqProvider as __with_name3;
#[doc(hidden)]
pub use super::HostsideSerializeTupleProvider as __with_name4;
#[doc(hidden)]
pub use super::HostsideSerializeTupleStructProvider as __with_name5;
#[doc(hidden)]
pub use super::HostsideSerializeTupleVariantProvider as __with_name6;
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
            use send_wrapper::SendWrapper;
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

            pub use super::super::super::__with_name0 as SerError;
            pub use super::super::super::__with_name1 as SerOk;
            pub use super::super::super::__with_name2 as Serializer;
            pub use super::super::super::__with_name3 as SerializeSeq;
            pub use super::super::super::__with_name4 as SerializeTuple;
            pub use super::super::super::__with_name5 as SerializeTupleStruct;
            pub use super::super::super::__with_name6 as SerializeTupleVariant;
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
                HostSerializeMap
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

            #[allow(clippy::too_many_lines)] // FIXME
            pub fn add_to_linker_get_host<T>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: impl GetHost</*&'a mut*/ T>,
            ) -> wasmtime::Result<()>
where {
                let mut inst = linker.instance("serde:serde/serde-serializer")?;
                inst.resource(
                    "serializer",
                    wasmtime::component::ResourceType::host::<Serializer>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<Serializer>::new_own(rep);
                        let _serializer = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "ser-ok",
                    wasmtime::component::ResourceType::host::<SerOk>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<SerOk>::new_own(rep);
                        let _ok = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "ser-error",
                    wasmtime::component::ResourceType::host::<SerError>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<SerError>::new_own(rep);
                        let _error = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "serialize-seq",
                    wasmtime::component::ResourceType::host::<SerializeSeq>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<SerializeSeq>::new_own(rep);
                        let _seq = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "serialize-tuple",
                    wasmtime::component::ResourceType::host::<SerializeTuple>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<SerializeTuple>::new_own(rep);
                        let _tuple = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "serialize-tuple-struct",
                    wasmtime::component::ResourceType::host::<SerializeTupleStruct>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this =
                            wasmtime::component::Resource::<SerializeTupleStruct>::new_own(rep);
                        let _tuple_struct = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "serialize-tuple-variant",
                    wasmtime::component::ResourceType::host::<SerializeTupleVariant>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this =
                            wasmtime::component::Resource::<SerializeTupleVariant>::new_own(rep);
                        let _tuple_variant = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
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
                    let r = serializer.serializer.take().erased_serialize_bool(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i8", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i8, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_i8(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i16", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i16, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_i16(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i32", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i32, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_i32(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-i64", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, i64, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_i64(v).wrap(host);
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
                        .take()
                        .erased_serialize_i128(i128::from_le_bytes(bytes))
                        .wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u8", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u8, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_u8(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u16", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u16, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_u16(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u32", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u32, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_u32(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-u64", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, u64, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_u64(v).wrap(host);
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
                        .take()
                        .erased_serialize_u128(u128::from_le_bytes(bytes))
                        .wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-f32", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, f32, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_f32(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-f64", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, f64, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_f64(v).wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap("[static]serializer.serialize-char", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Serializer>, char, ) | { 
                    let host = host_getter(caller.data_mut());
                    let (this, v) = (arg0, arg1);
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_char(v).wrap(host);
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
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_str(&v)
                            .wrap(host);
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
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_bytes(&v)
                            .wrap(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[static]serializer.serialize-none", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<Serializer>, ) | { 
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let serializer = host.table.delete(this)?;
                    let r = serializer.serializer.take().erased_serialize_none().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serializer.serialize-some",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let serializer = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = serializer.serializer.take().erased_serialize_some(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &mut caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
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
                    let r = serializer.serializer.take().erased_serialize_unit().wrap(host);
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
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_unit_struct(crate::intern::intern_string(name))
                            .wrap(host);
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
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_unit_variant(
                                crate::intern::intern_string(name),
                                variant_index,
                                crate::intern::intern_string(variant),
                            )
                            .wrap(host);
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
                        let (this, name, value) = (arg0, arg1, arg2);
                        anyhow::ensure!(this.owned());
                        let serializer = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_newtype_struct(
                                crate::intern::intern_string(name),
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &mut caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let r = {
                            let host = host_getter(caller.data_mut());
                            r.wrap(host)
                        };
                        Ok((r?,))
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
                        let (this, name, variant_index, variant, value) =
                            (arg0, arg1, arg2, arg3, arg4);
                        anyhow::ensure!(this.owned());
                        let serializer = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_newtype_variant(
                                crate::intern::intern_string(name),
                                variant_index,
                                crate::intern::intern_string(variant),
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &mut caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let r = {
                            let host = host_getter(caller.data_mut());
                            r.wrap(host)
                        };
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-seq",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        Option<Usize>,
                    )|
                          -> anyhow::Result<(
                        Result<
                            wasmtime::component::Resource<SerializeSeq>,
                            wasmtime::component::Resource<SerError>,
                        >,
                    )> {
                        let host = host_getter(caller.data_mut());
                        let (this, len) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let Serializer {
                            serializer, scope,
                        } = host.table.delete(this)?;
                        let r = match serializer
                            .take()
                            .erased_serialize_seq(len.map(|len| crate::wit_to_usize(len.val)))
                        {
                            Ok(serialize_seq) => Ok(host.table.push(SerializeSeq {
                                serialize_seq: SendWrapper::new(serialize_seq),
                                _scope: scope,
                            })?),
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-tuple",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (wasmtime::component::Resource<Serializer>, Usize)| {
                        let host = host_getter(caller.data_mut());
                        let (this, len) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let Serializer {
                            serializer, scope,
                        } = host.table.delete(this)?;
                        let r = match serializer
                            .take()
                            .erased_serialize_tuple(crate::wit_to_usize(len.val))
                        {
                            Ok(serialize_tuple) => Ok(host.table.push(SerializeTuple {
                                serialize_tuple: SendWrapper::new(serialize_tuple),
                                _scope: scope,
                            })?),
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-tuple-struct",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Serializer>,
                        wasmtime::component::__internal::String,
                        Usize,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, name, len) = (arg0, arg1, arg2);
                        anyhow::ensure!(this.owned());
                        let Serializer {
                            serializer, scope,
                        } = host.table.delete(this)?;
                        let r = match serializer.take().erased_serialize_tuple_struct(
                            crate::intern::intern_string(name),
                            crate::wit_to_usize(len.val),
                        ) {
                            Ok(serialize_tuple_struct) => {
                                Ok(host.table.push(SerializeTupleStruct {
                                    serialize_tuple_struct: SendWrapper::new(
                                        serialize_tuple_struct,
                                    ),
                                    _scope: scope,
                                })?)
                            }
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
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
                    )|
                          -> anyhow::Result<(
                        Result<
                            wasmtime::component::Resource<SerializeTupleVariant>,
                            wasmtime::component::Resource<SerError>,
                        >,
                    )> {
                        let host = host_getter(caller.data_mut());
                        let (this, name, variant_index, variant, len) =
                            (arg0, arg1, arg2, arg3, arg4);
                        anyhow::ensure!(this.owned());
                        let Serializer {
                            serializer, scope,
                        } = host.table.delete(this)?;
                        let r = match serializer.take().erased_serialize_tuple_variant(
                            crate::intern::intern_string(name),
                            variant_index,
                            crate::intern::intern_string(variant),
                            crate::wit_to_usize(len.val),
                        ) {
                            Ok(serialize_tuple_variant) => {
                                Ok(host.table.push(SerializeTupleVariant {
                                    serialize_tuple_variant: SendWrapper::new(
                                        serialize_tuple_variant,
                                    ),
                                    _scope: scope,
                                })?)
                            }
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serializer.serialize-map",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Serializer>,
                        Option<Usize>,
                    )|
                          -> anyhow::Result<(
                        Result<
                            wasmtime::component::Resource<SerializeMap>,
                            wasmtime::component::Resource<SerError>,
                        >,
                    )> {
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
                    )|
                          -> anyhow::Result<(
                        Result<
                            wasmtime::component::Resource<SerializeStruct>,
                            wasmtime::component::Resource<SerError>,
                        >,
                    )> {
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
                    )|
                          -> anyhow::Result<(
                        Result<
                            wasmtime::component::Resource<SerializeStructVariant>,
                            wasmtime::component::Resource<SerError>,
                        >,
                    )> {
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
                inst.func_wrap(
                    "[method]ser-error.display",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<SerError>,)| {
                        let host = host_getter(caller.data_mut());
                        let this = arg0;
                        anyhow::ensure!(!this.owned());
                        let error = host.table.get(&this)?;
                        let r = match &error.inner {
                            crate::ser::wasmtime_host::provider::SerErrorOrCustom::Error {
                                display,
                                ..
                            } => String::from(display),
                            crate::ser::wasmtime_host::provider::SerErrorOrCustom::Custom(msg) => {
                                String::from(msg)
                            }
                        };
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[method]ser-error.debug",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<SerError>,)| {
                        let host = host_getter(caller.data_mut());
                        let this = arg0;
                        anyhow::ensure!(!this.owned());
                        let error = host.table.get(&this)?;
                        let r = match &error.inner {
                            crate::ser::wasmtime_host::provider::SerErrorOrCustom::Error {
                                debug,
                                ..
                            } => {
                                format!("serde_wit::ser::Error {{ err: {debug} }}")
                            }
                            crate::ser::wasmtime_host::provider::SerErrorOrCustom::Custom(msg) => {
                                format!("serde_wit::ser::Error {{ err: Custom({msg}) }}")
                            }
                        };
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[static]ser-error.custom",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::__internal::String,)| {
                        let host = host_getter(caller.data_mut());
                        let msg = arg0;
                        let error = crate::ser::wasmtime_host::provider::SerError {
                            inner: crate::ser::wasmtime_host::provider::SerErrorOrCustom::Custom(
                                msg,
                            ),
                        };
                        let r = host.table.push(error);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]serialize-seq.serialize-element",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeSeq>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let mut seq = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = seq.serialize_seq.erased_serialize_element(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &mut caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(seq)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-seq.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeSeq>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let seq = host.table.delete(this)?;
                    let r = seq.serialize_seq.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serialize-tuple.serialize-element",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeTuple>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let mut tuple = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = tuple.serialize_tuple.erased_serialize_element(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &mut caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(tuple)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-tuple.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeTuple>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let tuple = host.table.delete(this)?;
                    let r = tuple.serialize_tuple.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serialize-tuple-struct.serialize-field",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeTupleStruct>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let mut tuple_struct = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = tuple_struct.serialize_tuple_struct.erased_serialize_field(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &mut caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(tuple_struct)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-tuple-struct.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeTupleStruct>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let tuple_struct = host.table.delete(this)?;
                    let r = tuple_struct.serialize_tuple_struct.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serialize-tuple-variant.serialize-field",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeTupleVariant>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let mut tuple_variant = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let r = tuple_variant
                            .serialize_tuple_variant
                            .erased_serialize_field(
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &mut caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(tuple_variant)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-tuple-variant.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeTupleVariant>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let tuple_variant = host.table.delete(this)?;
                    let r = tuple_variant.serialize_tuple_variant.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
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
                get: impl Fn(
                        &mut T,
                    ) -> &mut /*U*/
                    crate::ser::wasmtime_host::provider::HostsideSerializerProviderState
                    + Send
                    + Sync
                    + Copy
                    + 'static,
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
                        write!(f, "{self:?}")
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
