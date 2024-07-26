#![allow(clippy::module_inception)] // FIXME
#![allow(clippy::indexing_slicing)] // FIXME

pub struct SerdeSerializerClient {
    interface0: exports::serde::serde::serde_serialize::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;

    impl SerdeSerializerClient {
        pub fn add_to_linker<T>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(
                    &mut T,
                )
                    -> &mut crate::ser::wasmtime_host::provider::HostsideSerializerProviderState
                + Send
                + Sync
                + Copy
                + 'static,
        ) -> wasmtime::Result<()> {
            serde::serde::serde_serializer::add_to_linker(linker, get)
        }

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
        }

        #[allow(clippy::all)]
        pub mod serde_serializer {
            use std::sync::Mutex;

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

            use crate::ser::wasmtime_host::provider::HostsideSerializeMapProvider as SerializeMap;
            use crate::ser::wasmtime_host::provider::HostsideSerializeSeqProvider as SerializeSeq;
            use crate::ser::wasmtime_host::provider::HostsideSerializeStructProvider as SerializeStruct;
            use crate::ser::wasmtime_host::provider::HostsideSerializeStructVariantProvider as SerializeStructVariant;
            use crate::ser::wasmtime_host::provider::HostsideSerializeTupleProvider as SerializeTuple;
            use crate::ser::wasmtime_host::provider::HostsideSerializeTupleStructProvider as SerializeTupleStruct;
            use crate::ser::wasmtime_host::provider::HostsideSerializeTupleVariantProvider as SerializeTupleVariant;
            use crate::ser::wasmtime_host::provider::HostsideSerializerProvider as Serializer;
            use crate::ser::wasmtime_host::provider::SerError;
            use crate::ser::wasmtime_host::provider::SerOk;

            pub trait GetHost<T>:
                for<'a> Fn(&'a mut T) -> &'a mut crate::ser::wasmtime_host::provider::HostsideSerializerProviderState + Send + Sync + Copy + 'static
            {}

            impl<F, T> GetHost<T> for F
            where
                F: for<'a> Fn(&'a mut T) -> &'a mut crate::ser::wasmtime_host::provider::HostsideSerializerProviderState + Send + Sync + Copy + 'static,
            {}

            #[allow(clippy::too_many_lines)] // FIXME
            #[allow(clippy::missing_errors_doc)] // FIXME
            pub fn add_to_linker_get_host<T>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: impl GetHost<T>,
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
                inst.resource(
                    "serialize-map",
                    wasmtime::component::ResourceType::host::<SerializeMap>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<SerializeMap>::new_own(rep);
                        let _map = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "serialize-struct",
                    wasmtime::component::ResourceType::host::<SerializeStruct>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this = wasmtime::component::Resource::<SerializeStruct>::new_own(rep);
                        let _struct = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
                inst.resource(
                    "serialize-struct-variant",
                    wasmtime::component::ResourceType::host::<SerializeStructVariant>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        let host = host_getter(store.data_mut());
                        let this =
                            wasmtime::component::Resource::<SerializeStructVariant>::new_own(rep);
                        let _struct_variant = host.table.delete(this)?;
                        Ok(())
                    },
                )?;
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
                        let caller = Mutex::new(caller);
                        let r = serializer.serializer.take().erased_serialize_some(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                        let caller = Mutex::new(caller);
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_newtype_struct(
                                crate::intern::intern_string(name),
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                        let caller = Mutex::new(caller);
                        let r = serializer
                            .serializer
                            .take()
                            .erased_serialize_newtype_variant(
                                crate::intern::intern_string(name),
                                variant_index,
                                crate::intern::intern_string(variant),
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                        let Serializer { serializer, scope } = host.table.delete(this)?;
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
                        let Serializer { serializer, scope } = host.table.delete(this)?;
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
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, name, variant_index, variant, len) =
                            (arg0, arg1, arg2, arg3, arg4);
                        anyhow::ensure!(this.owned());
                        let Serializer { serializer, scope } = host.table.delete(this)?;
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
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, len) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let Serializer { serializer, scope } = host.table.delete(this)?;
                        let r = match serializer
                            .take()
                            .erased_serialize_map(len.map(|len| crate::wit_to_usize(len.val)))
                        {
                            Ok(serialize_map) => Ok(host.table.push(SerializeMap {
                                serialize_map: SendWrapper::new(serialize_map),
                                _scope: scope,
                            })?),
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
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
                        let host = host_getter(caller.data_mut());
                        let (this, name, len) = (arg0, arg1, arg2);
                        anyhow::ensure!(this.owned());
                        let Serializer { serializer, scope } = host.table.delete(this)?;
                        let r = match serializer.take().erased_serialize_struct(
                            crate::intern::intern_string(name),
                            crate::wit_to_usize(len.val),
                        ) {
                            Ok(serialize_struct) => Ok(host.table.push(SerializeStruct {
                                serialize_struct: SendWrapper::new(serialize_struct),
                                _scope: scope,
                            })?),
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
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
                        let host = host_getter(caller.data_mut());
                        let (this, name, variant_index, variant, len) =
                            (arg0, arg1, arg2, arg3, arg4);
                        anyhow::ensure!(this.owned());
                        let Serializer { serializer, scope } = host.table.delete(this)?;
                        let r = match serializer.take().erased_serialize_struct_variant(
                            crate::intern::intern_string(name),
                            variant_index,
                            crate::intern::intern_string(variant),
                            crate::wit_to_usize(len.val),
                        ) {
                            Ok(serialize_struct_variant) => {
                                Ok(host.table.push(SerializeStructVariant {
                                    serialize_struct_variant: SendWrapper::new(
                                        serialize_struct_variant,
                                    ),
                                    _scope: scope,
                                })?)
                            }
                            Err(error) => Err(host.table.push(error)?),
                        };
                        Ok((r,))
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
                        let caller = Mutex::new(caller);
                        let r = seq.serialize_seq.erased_serialize_element(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                        let caller = Mutex::new(caller);
                        let r = tuple.serialize_tuple.erased_serialize_element(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                        let caller = Mutex::new(caller);
                        let r = tuple_struct.serialize_tuple_struct.erased_serialize_field(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                        let caller = Mutex::new(caller);
                        let r = tuple_variant
                            .serialize_tuple_variant
                            .erased_serialize_field(
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
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
                inst.func_wrap(
                    "[static]serialize-map.serialize-key",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeMap>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, key) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let mut map = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let caller = Mutex::new(caller);
                        let r = map.serialize_map.erased_serialize_key(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &key,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(map)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap(
                    "[static]serialize-map.serialize-value",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeMap>,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, value) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let mut map = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let caller = Mutex::new(caller);
                        let r = map.serialize_map.erased_serialize_value(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(map)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap(
                    "[static]serialize-map.serialize-entry",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<SerializeMap>,
                        BorrowedSerializeHandle,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, key, value) = (arg0, arg1, arg2);
                        anyhow::ensure!(this.owned());
                        let mut map = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let caller = Mutex::new(caller);
                        let r = map.serialize_map.erased_serialize_entry(
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &key,
                            ),
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(map)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-map.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeMap>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let map = host.table.delete(this)?;
                    let r = map.serialize_map.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serialize-struct.serialize-field",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<SerializeStruct>,
                        wasmtime::component::__internal::String,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, key, value) = (arg0, arg1, arg2);
                        anyhow::ensure!(this.owned());
                        let mut r#struct = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let caller = Mutex::new(caller);
                        let r = r#struct.serialize_struct.erased_serialize_field(
                            crate::intern::intern_string(key),
                            &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                &caller,
                                host_getter,
                                move |mut ctx, guest, serialize, serializer| {
                                    let serialize = serialize.try_into_resource_any(&mut ctx)?;
                                    guest.call_serialize(ctx, serialize, serializer)
                                },
                                &value,
                            ),
                        );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(r#struct)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-struct.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeStruct>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let r#struct = host.table.delete(this)?;
                    let r = r#struct.serialize_struct.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serialize-struct.skip-field",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeStruct>,
                        wasmtime::component::__internal::String,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, field) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let r#struct = host.table.get_mut(&this)?;
                        let r = r#struct
                            .serialize_struct
                            .erased_skip_field(crate::intern::intern_string(field))
                            .wrap(host);
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap(
                    "[static]serialize-struct-variant.serialize-field",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<SerializeStructVariant>,
                        wasmtime::component::__internal::String,
                        BorrowedSerializeHandle,
                    )| {
                        let (this, key, value) = (arg0, arg1, arg2);
                        anyhow::ensure!(this.owned());
                        let mut struct_variant = {
                            let host = host_getter(caller.data_mut());
                            host.table.delete(this)
                        }?;
                        let caller = Mutex::new(caller);
                        let r = struct_variant
                            .serialize_struct_variant
                            .erased_serialize_field(
                                crate::intern::intern_string(key),
                                &crate::ser::wasmtime_host::provider::SerializableSerialize::new(
                                    &caller,
                                    host_getter,
                                    move |mut ctx, guest, serialize, serializer| {
                                        let serialize =
                                            serialize.try_into_resource_any(&mut ctx)?;
                                        guest.call_serialize(ctx, serialize, serializer)
                                    },
                                    &value,
                                ),
                            );
                        let mut caller = caller.into_inner().map_err(|_| {
                            anyhow::anyhow!("SerializableSerialize should not be poisoned")
                        })?;
                        let host = host_getter(caller.data_mut());
                        let r = r.wrap(host);
                        let this = host.table.push(struct_variant)?;
                        Ok((this, r?))
                    },
                )?;
                inst.func_wrap("[static]serialize-struct-variant.end", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<SerializeStructVariant>, ) | {
                    let host = host_getter(caller.data_mut());
                    let this = arg0;
                    anyhow::ensure!(this.owned());
                    let struct_variant = host.table.delete(this)?;
                    let r = struct_variant.serialize_struct_variant.take().erased_end().wrap(host);
                    Ok((r?,))
                })?;
                inst.func_wrap(
                    "[static]serialize-struct-variant.skip-field",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<SerializeStructVariant>,
                        wasmtime::component::__internal::String,
                    )| {
                        let host = host_getter(caller.data_mut());
                        let (this, field) = (arg0, arg1);
                        anyhow::ensure!(this.owned());
                        let struct_variant = host.table.get_mut(&this)?;
                        let r = struct_variant
                            .serialize_struct_variant
                            .erased_skip_field(crate::intern::intern_string(field))
                            .wrap(host);
                        Ok((this, r?))
                    },
                )?;
                Ok(())
            }

            #[allow(clippy::missing_errors_doc)] // FIXME
            pub fn add_to_linker<T>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(
                        &mut T,
                    ) -> &mut crate::ser::wasmtime_host::provider::HostsideSerializerProviderState
                    + Send
                    + Sync
                    + Copy
                    + 'static,
            ) -> wasmtime::Result<()> {
                add_to_linker_get_host(linker, get)
            }
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
