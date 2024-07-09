use ::serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use scoped_reference::{ScopedBorrowMut, ScopedReference};

mod bindings;
// mod bindings {
//     #![allow(clippy::indexing_slicing)] // FIXME
//     wasmtime::component::bindgen!({
//         world: "serde-serializer-client",
//         with: {
//             "serde:serde/serde-serializer/serializer": super::HostsideSerializerProvider,
//             "serde:serde/serde-serializer/ser-ok": super::SerOk,
//             "serde:serde/serde-serializer/ser-error": super::SerError,
//         },
//         trappable_imports: true,
//         // include_generated_code_from_file: true,
//     });
// }

use crate::any::Any;
use crate::intern::intern_string;
use crate::wit_to_usize;

pub struct HostsideSerializerProviderState {
    table: wasmtime::component::ResourceTable,
}

pub struct HostsideSerializerProvider {
    serializer: Box<dyn ErasedSerializer>,
    is_human_readable: bool,
    scope: ScopedBorrowMut<()>,
}

trait WrapSerResult {
    type Ok;

    fn wrap(
        self,
        state: &mut HostsideSerializerProviderState,
    ) -> anyhow::Result<
        Result<
            Self::Ok,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    >;
}

impl WrapSerResult for Result<SerOk, SerError> {
    type Ok = wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>;

    fn wrap(
        self,
        state: &mut HostsideSerializerProviderState,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        match self {
            Ok(ok) => Ok(Ok(state.table.push(ok)?)),
            Err(error) => Ok(Err(state.table.push(error)?)),
        }
    }
}

// impl WrapSerResult for Result<(), SerError> {
//     type Ok = ();

//     fn wrap(self) -> Result<(), bindings::exports::serde::serde::serde_serializer::SerError> {
//         self.map_err(bindings::exports::serde::serde::serde_serializer::SerError::new)
//     }
// }

impl bindings::serde::serde::serde_serializer::HostSerializer for HostsideSerializerProviderState {
    fn serialize_bool(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: bool,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_bool(v).wrap(self)
    }

    fn serialize_i8(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: i8,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_i8(v).wrap(self)
    }

    fn serialize_i16(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: i16,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_i16(v).wrap(self)
    }

    fn serialize_i32(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: i32,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_i32(v).wrap(self)
    }

    fn serialize_i64(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: i64,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_i64(v).wrap(self)
    }

    fn serialize_i128(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: bindings::serde::serde::serde_types::S128,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;

        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        serializer
            .serializer
            .erased_serialize_i128(i128::from_le_bytes(bytes))
            .wrap(self)
    }

    fn serialize_u8(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: u8,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_u8(v).wrap(self)
    }

    fn serialize_u16(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: u16,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_u16(v).wrap(self)
    }

    fn serialize_u32(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: u32,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_u32(v).wrap(self)
    }

    fn serialize_u64(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: u64,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_u64(v).wrap(self)
    }

    fn serialize_u128(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: bindings::serde::serde::serde_types::U128,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;

        let le_hi = v.le_hi.to_le_bytes();
        let le_lo = v.le_lo.to_le_bytes();

        let bytes = [
            le_hi[0], le_hi[1], le_hi[2], le_hi[3], le_hi[4], le_hi[5], le_hi[6], le_hi[7],
            le_lo[0], le_lo[1], le_lo[2], le_lo[3], le_lo[4], le_lo[5], le_lo[6], le_lo[7],
        ];

        serializer
            .serializer
            .erased_serialize_u128(u128::from_le_bytes(bytes))
            .wrap(self)
    }

    fn serialize_f32(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: f32,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_f32(v).wrap(self)
    }

    fn serialize_f64(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: f64,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_f64(v).wrap(self)
    }

    fn serialize_char(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: char,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_char(v).wrap(self)
    }

    fn serialize_str(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: String,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_str(&v).wrap(self)
    }

    fn serialize_bytes(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        v: Vec<u8>,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_bytes(&v).wrap(self)
    }

    fn serialize_none(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_none().wrap(self)
    }

    fn serialize_some<T>(
        &mut self,
        ctx: *mut wasmtime::StoreContextMut<'_, T>,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        value: bindings::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> anyhow::Result<Result<
        wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
        wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
    >> {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        let ctx: *mut () = ctx.cast();
        serializer.serializer
            .erased_serialize_some(&SerializableSerialize::new(self, Box::new(move |guest, serialize, serializer| {
                let mut ctx = unsafe { &mut *(ctx.cast::<wasmtime::StoreContextMut<'_, T>>()) };
                let serialize = serialize.try_into_resource_any(&mut ctx)?;
                guest.call_serialize(ctx, serialize, serializer)
            }), &value))
            .wrap(self)
    }

    fn serialize_unit(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer.erased_serialize_unit().wrap(self)
    }

    fn serialize_unit_struct(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        name: String,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer
            .serializer
            .erased_serialize_unit_struct(intern_string(name))
            .wrap(self)
    }

    fn serialize_unit_variant(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        name: String,
        variant_index: u32,
        variant: String,
    ) -> anyhow::Result<
        Result<
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
            wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
        >,
    > {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer
            .serializer
            .erased_serialize_unit_variant(
                intern_string(name),
                variant_index,
                intern_string(variant),
            )
            .wrap(self)
    }

    /*fn serialize_newtype_struct(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        name: String,
        value: bindings::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> anyhow::Result<Result<
        wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
        wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
    >> {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer
            .erased_serialize_newtype_struct(intern_string(name), &SerializableSerialize::new(self, &value))
            .wrap(self)
    }

    fn serialize_newtype_variant(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
        name: String,
        variant_index: u32,
        variant: String,
        value: bindings::serde::serde::serde_serializer::BorrowedSerializeHandle,
    ) -> anyhow::Result<Result<
        wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerOk>,
        wasmtime::component::Resource<bindings::serde::serde::serde_serializer::SerError>,
    >> {
        anyhow::ensure!(this.owned());
        let serializer = self.table.delete(this)?;
        serializer.serializer
            .erased_serialize_newtype_variant(intern_string(name), variant_index, intern_string(variant), &SerializableSerialize::new(self, &value))
            .wrap(self)
    }*/

    /*fn serialize_seq(
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
                HostsideSerializeSeqProvider {
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
                HostsideSerializeTupleProvider {
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
                HostsideSerializeTupleStructProvider {
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
                HostsideSerializeTupleVariantProvider {
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
                HostsideSerializeMapProvider {
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
                HostsideSerializeStructProvider {
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
                HostsideSerializeStructVariantProvider {
                    serialize_struct_variant,
                    _scope: scope,
                },
            ),
        )
    }*/

    fn is_human_readable(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
    ) -> anyhow::Result<bool> {
        anyhow::ensure!(!this.owned());
        let serializer = self.table.get(&this)?;
        Ok(serializer.serializer.erased_is_human_readable())
    }

    fn drop(
        &mut self,
        this: wasmtime::component::Resource<HostsideSerializerProvider>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(this.owned());
        let _serializer = self.table.delete(this)?;
        Ok(())
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
    // fn erased_serialize_seq<'a>(
    //     self: Box<Self>,
    //     len: Option<usize>,
    // ) -> Result<Box<dyn ErasedSerializeSeq + 'a>, SerError>
    // where
    //     Self: 'a;
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

// trait ErasedSerializeSeq {
//     fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
// }

// trait ErasedSerializeTuple {
//     fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
// }

// trait ErasedSerializeTupleStruct {
//     fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
// }

// trait ErasedSerializeTupleVariant {
//     fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
// }

// trait ErasedSerializeMap {
//     fn erased_serialize_key(&mut self, key: &SerializableSerialize) -> Result<(), SerError>;
//     fn erased_serialize_value(&mut self, value: &SerializableSerialize) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
// }

// trait ErasedSerializeStruct {
//     fn erased_serialize_field(
//         &mut self,
//         key: &'static str,
//         value: &SerializableSerialize,
//     ) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
//     fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError>;
// }

// trait ErasedSerializeStructVariant {
//     fn erased_serialize_field(
//         &mut self,
//         key: &'static str,
//         value: &SerializableSerialize,
//     ) -> Result<(), SerError>;
//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError>;
//     fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError>;
// }

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

    // fn erased_serialize_seq<'a>(
    //     self: Box<Self>,
    //     len: Option<usize>,
    // ) -> Result<Box<dyn ErasedSerializeSeq + 'a>, SerError>
    // where
    //     Self: 'a,
    // {
    //     self.serialize_seq(len)
    //         .map(|serialize_seq| {
    //             let serialize_seq: Box<dyn ErasedSerializeSeq + 'a> = Box::new(serialize_seq);
    //             serialize_seq
    //         })
    //         .map_err(SerError::wrap)
    // }

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

// impl<T: SerializeSeq> ErasedSerializeSeq for T {
//     fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
//         self.serialize_element(value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }
// }

// impl<T: SerializeTuple> ErasedSerializeTuple for T {
//     fn erased_serialize_element(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
//         self.serialize_element(value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }
// }

// impl<T: SerializeTupleStruct> ErasedSerializeTupleStruct for T {
//     fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
//         self.serialize_field(value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }
// }

// impl<T: SerializeTupleVariant> ErasedSerializeTupleVariant for T {
//     fn erased_serialize_field(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
//         self.serialize_field(value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }
// }

// impl<T: SerializeMap> ErasedSerializeMap for T {
//     fn erased_serialize_key(&mut self, key: &SerializableSerialize) -> Result<(), SerError> {
//         self.serialize_key(key).map_err(SerError::wrap)
//     }

//     fn erased_serialize_value(&mut self, value: &SerializableSerialize) -> Result<(), SerError> {
//         self.serialize_value(value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }
// }

// impl<T: SerializeStruct> ErasedSerializeStruct for T {
//     fn erased_serialize_field(
//         &mut self,
//         key: &'static str,
//         value: &SerializableSerialize,
//     ) -> Result<(), SerError> {
//         self.serialize_field(key, value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }

//     fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError> {
//         self.skip_field(key).map_err(SerError::wrap)
//     }
// }

// impl<T: SerializeStructVariant> ErasedSerializeStructVariant for T {
//     fn erased_serialize_field(
//         &mut self,
//         key: &'static str,
//         value: &SerializableSerialize,
//     ) -> Result<(), SerError> {
//         self.serialize_field(key, value).map_err(SerError::wrap)
//     }

//     fn erased_end(self: Box<Self>) -> Result<SerOk, SerError> {
//         self.end().map(SerOk::wrap).map_err(SerError::wrap)
//     }

//     fn erased_skip_field(&mut self, key: &'static str) -> Result<(), SerError> {
//         self.skip_field(key).map_err(SerError::wrap)
//     }
// }

pub struct SerOk {
    value: Any,
}

// TODO: safety
unsafe impl Send for SerOk {}

impl SerOk {
    fn wrap<T>(value: T) -> Self {
        // Safety: TODO
        Self {
            value: unsafe { Any::new(value) },
        }
    }
}

// impl bindings::exports::serde::serde::serde_serializer::HostSerOk for SerOk {}

pub struct SerError {
    inner: SerErrorOrCustom,
}

// TODO: safety
unsafe impl Send for SerError {}

enum SerErrorOrCustom {
    Error {
        err: Any,
        display: String,
        debug: String,
    },
    Custom(String),
}

// impl bindings::exports::serde::serde::serde_serializer::HostSerError for SerError {
//     fn display(&self) -> String {
//         match &self.inner {
//             SerErrorOrCustom::Error { display, .. } => String::from(display),
//             SerErrorOrCustom::Custom(msg) => String::from(msg),
//         }
//     }

//     fn debug(&self) -> String {
//         match &self.inner {
//             SerErrorOrCustom::Error { debug, .. } => {
//                 format!("serde_wit::ser::Error {{ err: {debug} }}")
//             }
//             SerErrorOrCustom::Custom(msg) => {
//                 format!("serde_wit::ser::Error {{ err: Custom({msg}) }}")
//             }
//         }
//     }

//     fn custom(msg: String) -> bindings::exports::serde::serde::serde_serializer::SerError {
//         let error = Self {
//             inner: SerErrorOrCustom::Custom(msg),
//         };

//         bindings::exports::serde::serde::serde_serializer::SerError::new(error)
//     }
// }

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

// pub struct HostsideSerializeSeqProvider {
//     serialize_seq: Box<dyn ErasedSerializeSeq>,
//     _scope: ScopedBorrowMut<()>,
// }

// pub struct HostsideSerializeTupleProvider {
//     serialize_tuple: Box<dyn ErasedSerializeTuple>,
//     _scope: ScopedBorrowMut<()>,
// }

// pub struct HostsideSerializeTupleStructProvider {
//     serialize_tuple_struct: Box<dyn ErasedSerializeTupleStruct>,
//     _scope: ScopedBorrowMut<()>,
// }

// pub struct HostsideSerializeTupleVariantProvider {
//     serialize_tuple_variant: Box<dyn ErasedSerializeTupleVariant>,
//     _scope: ScopedBorrowMut<()>,
// }

// pub struct HostsideSerializeMapProvider {
//     serialize_map: Box<dyn ErasedSerializeMap>,
//     _scope: ScopedBorrowMut<()>,
// }

// pub struct HostsideSerializeStructProvider {
//     serialize_struct: Box<dyn ErasedSerializeStruct>,
//     _scope: ScopedBorrowMut<()>,
// }

// pub struct HostsideSerializeStructVariantProvider {
//     serialize_struct_variant: Box<dyn ErasedSerializeStructVariant>,
//     _scope: ScopedBorrowMut<()>,
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeSeq
//     for HostsideSerializeSeqProvider
// {
//     fn serialize_element(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeSeq,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeSeq,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_seq
//             .erased_serialize_element(&SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeSeq,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>().serialize_seq.erased_end().wrap()
//     }
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeTuple
//     for HostsideSerializeTupleProvider
// {
//     fn serialize_element(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeTuple,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeTuple,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_tuple
//             .erased_serialize_element(&SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeTuple,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>()
//             .serialize_tuple
//             .erased_end()
//             .wrap()
//     }
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeTupleStruct
//     for HostsideSerializeTupleStructProvider
// {
//     fn serialize_field(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_tuple_struct
//             .erased_serialize_field(&SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeTupleStruct,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>()
//             .serialize_tuple_struct
//             .erased_end()
//             .wrap()
//     }
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeTupleVariant
//     for HostsideSerializeTupleVariantProvider
// {
//     fn serialize_field(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_tuple_variant
//             .erased_serialize_field(&SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeTupleVariant,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>()
//             .serialize_tuple_variant
//             .erased_end()
//             .wrap()
//     }
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeMap
//     for HostsideSerializeMapProvider
// {
//     fn serialize_key(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
//         key: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeMap,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let key = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(key.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_map
//             .erased_serialize_key(&SerializableSerialize::new(&key))
//             .wrap();

//         (this, result)
//     }

//     fn serialize_value(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeMap,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_map
//             .erased_serialize_value(&SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeMap,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>().serialize_map.erased_end().wrap()
//     }
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeStruct
//     for HostsideSerializeStructProvider
// {
//     fn serialize_field(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeStruct,
//         key: String,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeStruct,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_struct
//             .erased_serialize_field(intern_string(key), &SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeStruct,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>()
//             .serialize_struct
//             .erased_end()
//             .wrap()
//     }

//     fn skip_field(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeStruct,
//         key: String,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeStruct,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         let result = this
//             .get_mut::<Self>()
//             .serialize_struct
//             .erased_skip_field(intern_string(key))
//             .wrap();

//         (this, result)
//     }
// }

// impl bindings::exports::serde::serde::serde_serializer::HostSerializeStructVariant
//     for HostsideSerializeStructVariantProvider
// {
//     fn serialize_field(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
//         key: String,
//         value: bindings::exports::serde::serde::serde_serializer::BorrowedSerializeHandle,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         // TODO: Safety
//         let value = unsafe {
//             bindings::serde::serde::serde_serialize::Serialize::from_handle(value.borrowed_handle)
//         };

//         let result = this
//             .get_mut::<Self>()
//             .serialize_struct_variant
//             .erased_serialize_field(intern_string(key), &SerializableSerialize::new(&value))
//             .wrap();

//         (this, result)
//     }

//     fn end(
//         this: bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
//     ) -> Result<
//         bindings::exports::serde::serde::serde_serializer::SerOk,
//         bindings::exports::serde::serde::serde_serializer::SerError,
//     > {
//         this.into_inner::<Self>()
//             .serialize_struct_variant
//             .erased_end()
//             .wrap()
//     }

//     fn skip_field(
//         mut this: bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
//         key: String,
//     ) -> (
//         bindings::exports::serde::serde::serde_serializer::SerializeStructVariant,
//         Result<(), bindings::exports::serde::serde::serde_serializer::SerError>,
//     ) {
//         let result = this
//             .get_mut::<Self>()
//             .serialize_struct_variant
//             .erased_skip_field(intern_string(key))
//             .wrap();

//         (this, result)
//     }
// }

struct SerializableSerialize<'a> {
    state: &'a mut HostsideSerializerProviderState,
    do_serialize: Box<dyn Fn(&bindings::exports::serde::serde::serde_serialize::GuestSerialize, wasmtime::component::Resource<GuestSerialize>, bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle) -> anyhow::Result<Result<bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle, bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle>>>,
    borrowed_serialize_handle: u32,
    _borrow: core::marker::PhantomData<&'a GuestSerialize>,
}

enum GuestSerialize {}

impl<'a> SerializableSerialize<'a> {
    fn new(state: &'a mut HostsideSerializerProviderState, do_serialize: Box<dyn Fn(&bindings::exports::serde::serde::serde_serialize::GuestSerialize, wasmtime::component::Resource<GuestSerialize>, bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle) -> anyhow::Result<Result<bindings::exports::serde::serde::serde_serialize::OwnedSerOkHandle, bindings::exports::serde::serde::serde_serialize::OwnedSerErrorHandle>>>, serialize: &'a bindings::serde::serde::serde_serializer::BorrowedSerializeHandle) -> Self {
        Self {
            state,
            do_serialize,
            borrowed_serialize_handle: serialize.borrowed_handle,
            _borrow: core::marker::PhantomData::<&'a GuestSerialize>,
        }
    }
}

impl<'a> ::serde::Serialize for SerializableSerialize<'a> {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let result = HostsideSerializerProvider::with_new(serializer, |serializer| -> anyhow::Result<_> {
            let serializer = self.state.table.push(serializer).map_err(|err| anyhow::anyhow!("bug: failed to create a Serializer resource: {err}"))?;
            let serializer =
                bindings::exports::serde::serde::serde_serialize::OwnedSerializerHandle { owned_handle: serializer.rep() };
            let guest = todo!();
            (self.do_serialize)(&guest,  wasmtime::component::Resource::new_borrow(self.borrowed_serialize_handle), serializer)
        }).map_err(|err| ::serde::ser::Error::custom(err))?;

        match result {
            Ok(value) => {
                let SerOk { value } = self.state.table.delete(wasmtime::component::Resource::new_own(value.owned_handle)).map_err(|err| ::serde::ser::Error::custom(
                    format!("bug: invalid Serializer::Ok handle: {err}"),
                ))?;
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
                let SerError { inner: err } = self.state.table.delete(wasmtime::component::Resource::new_own(err.owned_handle)).map_err(|err| ::serde::ser::Error::custom(
                    format!("bug: invalid Serializer::Error handle: {err}"),
                ))?;
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
