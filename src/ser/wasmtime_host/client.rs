use core::{cell::RefCell, fmt};

use serde::serde_if_integer128;

wasmtime_component_macro::bindgen!({ world: "serde-serializer-provider" });

pub struct HostsideSerializerClient<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut> {
    serialize: &'a S,
    store: RefCell<T>,
}

impl<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut> serialize::Serialize
    for HostsideSerializerClient<'a, S, T>
{
    fn test(
        &mut self,
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> anyhow::Result<(serde_types::U128, serde_types::Usize)> {
        todo!() // self.serializer.test(&mut self.store, x, y)
    }
}

impl<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>
    HostsideSerializerClient<'a, S, T>
{
    pub fn new(serialize: &'a S, store: T) -> Self {
        Self {
            serialize,
            store: RefCell::new(store),
        }
    }

    fn serialize(&self, serializer: serializer::Serializer) -> Result<SerOk, SerError> {
        let Ok(mut store) = self.store.try_borrow_mut() else {
            return Err(serde::ser::Error::custom("bug: double Serialize::serialize"));
        };

        self.serialize
            .serialize(SerializerableSerializer::new(serializer, &mut *store))
    }
}

struct SerializerableSerializer<T: wasmtime::AsContextMut> {
    serializer: serializer::Serializer,
    store: T,
}

impl<T: wasmtime::AsContextMut> SerializerableSerializer<T> {
    fn new(serializer: serializer::Serializer, store: T) -> Self {
        Self { serializer, store }
    }
}

impl<T: wasmtime::AsContextMut> serde::Serializer for SerializerableSerializer<T> {
    type Ok = SerOk;
    type Error = SerError;

    type SerializeSeq = SerializerableSerializeSeq<T>;
    type SerializeTuple = SerializerableSerializeTuple<T>;
    type SerializeTupleStruct = SerializerableSerializeTupleStruct<T>;
    type SerializeTupleVariant = SerializerableSerializeTupleVariant<T>;
    type SerializeMap = SerializerableSerializeMap<T>;
    type SerializeStruct = SerializerableSerializeStruct<T>;
    type SerializeStructVariant = SerializerableSerializeStructVariant<T>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i64(v)
    }

    serde_if_integer128! {
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            let bytes = v.to_le_bytes();

            let le_hi = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let le_lo = [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ];

            self.serializer.serialize_i128(serde_types::S128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            })
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u64(v)
    }

    serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            let bytes = v.to_le_bytes();

            let le_hi = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let le_lo = [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ];

            self.serializer.serialize_u128(serde_types::U128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            })
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_none()
    }

    fn serialize_some<V: ?Sized + serde::Serialize>(
        self,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer
            .serialize_some(&HostsideSerializerClient::new(value, self.store))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<V: ?Sized + serde::Serialize>(
        self,
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer
            .serialize_newtype_struct(name, &HostsideSerializerClient::new(value, self.store))
    }

    fn serialize_newtype_variant<V: ?Sized + serde::Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            &HostsideSerializerClient::new(value, self.store),
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = match len {
            Some(len) => match u32::try_from(len) {
                Ok(len) => Some(serde_types::Usize { val: len }),
                Err(_) => {
                    return Err(serde::ser::Error::custom(
                        "Serializer::serialize_seq len exceeds u32",
                    ))
                }
            },
            None => None,
        };

        self.serializer
            .serialize_seq(len)
            .map(|serialize_seq| SerializerableSerializeSeq {
                serialize_seq: Some(serialize_seq),
                store: self.store,
            })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_tuple len exceeds u32",
            ));
        };

        self.serializer
            .serialize_tuple(serde_types::Usize { val: len })
            .map(|serialize_tuple| SerializerableSerializeTuple {
                serialize_tuple: Some(serialize_tuple),
                store: self.store,
            })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_tuple_struct len exceeds u32",
            ));
        };

        self.serializer
            .serialize_tuple_struct(name, serde_types::Usize { val: len })
            .map(
                |serialize_tuple_struct| SerializerableSerializeTupleStruct {
                    serialize_tuple_struct: Some(serialize_tuple_struct),
                    store: self.store,
                },
            )
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_tuple_variant len exceeds u32",
            ));
        };

        self.serializer
            .serialize_tuple_variant(
                name,
                variant_index,
                variant,
                serde_types::Usize { val: len },
            )
            .map(
                |serialize_tuple_variant| SerializerableSerializeTupleVariant {
                    serialize_tuple_variant: Some(serialize_tuple_variant),
                    store: self.store,
                },
            )
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = match len {
            Some(len) => match u32::try_from(len) {
                Ok(len) => Some(serde_types::Usize { val: len }),
                Err(_) => {
                    return Err(serde::ser::Error::custom(
                        "Serializer::serialize_map len exceeds u32",
                    ))
                }
            },
            None => None,
        };

        self.serializer
            .serialize_map(len)
            .map(|serialize_map| SerializerableSerializeMap {
                serialize_map: Some(serialize_map),
                store: self.store,
            })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_struct len exceeds u32",
            ));
        };

        self.serializer
            .serialize_struct(name, serde_types::Usize { val: len })
            .map(|serialize_struct| SerializerableSerializeStruct {
                serialize_struct: Some(serialize_struct),
                store: self.store,
            })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let Ok(len) = u32::try_from(len) else {
            return Err(serde::ser::Error::custom(
                "Serializer::serialize_struct_variant len exceeds u32",
            ));
        };

        self.serializer
            .serialize_struct_variant(
                name,
                variant_index,
                variant,
                serde_types::Usize { val: len },
            )
            .map(
                |serialize_struct_variant| SerializerableSerializeStructVariant {
                    serialize_struct_variant: Some(serialize_struct_variant),
                    store: self.store,
                },
            )
    }

    fn is_human_readable(&self) -> bool {
        self.serializer.is_human_readable()
    }
}

struct SerializerableSerializeSeq<T: wasmtime::AsContextMut> {
    serialize_seq: Option<SerializeSeq>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeSeq for SerializerableSerializeSeq<T> {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_element<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_seq) = self.serialize_seq.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeSeq::serialize_element after free"));
        };

        let (serialize_seq, result) =
            serialize_seq.serialize_element(&HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_seq = Some(serialize_seq);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_seq) = self.serialize_seq.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeSeq::end after free"));
        };

        serialize_seq.end()
    }
}

struct SerializerableSerializeTuple<T: wasmtime::AsContextMut> {
    serialize_tuple: Option<SerializeTuple>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeTuple for SerializerableSerializeTuple<T> {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_element<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple) = self.serialize_tuple.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTuple::serialize_element after free"));
        };

        let (serialize_tuple, result) = serialize_tuple
            .serialize_element(&HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_tuple = Some(serialize_tuple);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple) = self.serialize_tuple.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTuple::end after free"));
        };

        serialize_tuple.end()
    }
}

struct SerializerableSerializeTupleStruct<T: wasmtime::AsContextMut> {
    serialize_tuple_struct: Option<SerializeTupleStruct>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeTupleStruct
    for SerializerableSerializeTupleStruct<T>
{
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple_struct) = self.serialize_tuple_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleStruct::serialize_field after free"));
        };

        let (serialize_tuple_struct, result) = serialize_tuple_struct
            .serialize_field(&HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_tuple_struct = Some(serialize_tuple_struct);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple_struct) = self.serialize_tuple_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleStruct::end after free"));
        };

        serialize_tuple_struct.end()
    }
}

struct SerializerableSerializeTupleVariant<T: wasmtime::AsContextMut> {
    serialize_tuple_variant: Option<SerializeTupleVariant>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeTupleVariant
    for SerializerableSerializeTupleVariant<T>
{
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_tuple_variant) = self.serialize_tuple_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleVariant::serialize_field after free"));
        };

        let (serialize_tuple_variant, result) = serialize_tuple_variant
            .serialize_field(&HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_tuple_variant = Some(serialize_tuple_variant);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_tuple_variant) = self.serialize_tuple_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeTupleVariant::end after free"));
        };

        serialize_tuple_variant.end()
    }
}

struct SerializerableSerializeMap<T: wasmtime::AsContextMut> {
    serialize_map: Option<SerializeMap>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeMap for SerializerableSerializeMap<T> {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_key<K: ?Sized + serde::Serialize>(&mut self, key: &K) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::serialize_key after free"));
        };

        let (serialize_map, result) =
            serialize_map.serialize_key(&HostsideSerializerClient::new(key, &mut self.store));
        self.serialize_map = Some(serialize_map);

        result
    }

    fn serialize_value<V: ?Sized + serde::Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::serialize_value after free"));
        };

        let (serialize_map, result) =
            serialize_map.serialize_value(&HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_map = Some(serialize_map);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::end after free"));
        };

        serialize_map.end()
    }

    fn serialize_entry<K: ?Sized + serde::Serialize, V: ?Sized + serde::Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_map) = self.serialize_map.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeMap::serialize_map after free"));
        };

        let (serialize_map, result) =
            serialize_map.serialize_key(&HostsideSerializerClient::new(key, &mut self.store));

        if let Err(err) = result {
            self.serialize_map = Some(serialize_map);
            return Err(err);
        }

        let (serialize_map, result) =
            serialize_map.serialize_value(&HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_map = Some(serialize_map);

        result
    }
}

struct SerializerableSerializeStruct<T: wasmtime::AsContextMut> {
    serialize_struct: Option<SerializeStruct>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeStruct for SerializerableSerializeStruct<T> {
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStruct::serialize_field after free"));
        };

        let (serialize_struct, result) = serialize_struct
            .serialize_field(key, &HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_struct = Some(serialize_struct);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStruct::end after free"));
        };

        serialize_struct.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let Some(serialize_struct) = self.serialize_struct.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStruct::skip_field after free"));
        };

        let (serialize_struct, result) = serialize_struct.skip_field(key);
        self.serialize_struct = Some(serialize_struct);

        result
    }
}

struct SerializerableSerializeStructVariant<T: wasmtime::AsContextMut> {
    serialize_struct_variant: Option<SerializeStructVariant>,
    store: T,
}

impl<T: wasmtime::AsContextMut> serde::ser::SerializeStructVariant
    for SerializerableSerializeStructVariant<T>
{
    type Ok = SerOk;
    type Error = SerError;

    fn serialize_field<V: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<(), Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStructVariant::serialize_field after free"));
        };

        let (serialize_struct_variant, result) = serialize_struct_variant
            .serialize_field(key, &HostsideSerializerClient::new(value, &mut self.store));
        self.serialize_struct_variant = Some(serialize_struct_variant);

        result
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStructVariant::end after free"));
        };

        serialize_struct_variant.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let Some(serialize_struct_variant) = self.serialize_struct_variant.take() else {
            return Err(serde::ser::Error::custom("bug: SerializeStructVariant::skip_field after free"));
        };

        let (serialize_struct_variant, result) = serialize_struct_variant.skip_field(key);
        self.serialize_struct_variant = Some(serialize_struct_variant);

        result
    }
}

// serializer::SerOk
struct SerOk {
    _private: (),
}

// serializer::SerError
struct SerError {
    _private: (),
}

impl serde::ser::Error for SerError {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        todo!("wit-bindgen")
    }
}

impl serde::ser::StdError for SerError {}

impl fmt::Debug for SerError {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        todo!() // fmt.write_str("SerError")
    }
}

impl fmt::Display for SerError {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        todo!() // fmt.write_str("SerError")
    }
}

// TODO: remove
impl serializer::Serializer {
    fn serialize_bool(self, _v: bool) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i8(self, _v: i8) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i16(self, _v: i16) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i32(self, _v: i32) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_i64(self, _v: i64) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    serde_if_integer128! {
        fn serialize_i128(self, _v: serde_types::S128) -> Result<SerOk, SerError> {
            todo!("wit-bindgen")
        }
    }

    fn serialize_u8(self, _v: u8) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_u16(self, _v: u16) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_u32(self, _v: u32) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_u64(self, _v: u64) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    serde_if_integer128! {
        fn serialize_u128(self, _v: serde_types::U128) -> Result<SerOk, SerError> {
            todo!("wit-bindgen")
        }
    }

    fn serialize_f32(self, _v: f32) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_f64(self, _v: f64) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_char(self, _v: char) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_str(self, _v: &str) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_none(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_some<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_unit(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_unit_struct(self, _name: &str) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_newtype_struct<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _name: &str,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_newtype_variant<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_seq(self, _len: Option<serde_types::Usize>) -> Result<SerializeSeq, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_tuple(self, _len: serde_types::Usize) -> Result<SerializeTuple, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_tuple_struct(
        self,
        _name: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeTupleStruct, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_tuple_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeTupleVariant, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_map(self, _len: Option<serde_types::Usize>) -> Result<SerializeMap, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_struct(
        self,
        _name: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeStruct, SerError> {
        todo!("wit-bindgen")
    }

    fn serialize_struct_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: serde_types::Usize,
    ) -> Result<SerializeStructVariant, SerError> {
        todo!("wit-bindgen")
    }

    fn is_human_readable(&self) -> bool {
        todo!("wit-bindgen")
    }
}

struct SerializeSeq {
    _private: (),
}

struct SerializeTuple {
    _private: (),
}

struct SerializeTupleStruct {
    _private: (),
}

struct SerializeTupleVariant {
    _private: (),
}

struct SerializeMap {
    _private: (),
}

struct SerializeStruct {
    _private: (),
}

struct SerializeStructVariant {
    _private: (),
}

impl SerializeSeq {
    fn serialize_element<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeTuple {
    fn serialize_element<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeTupleStruct {
    fn serialize_field<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeTupleVariant {
    fn serialize_field<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeMap {
    fn serialize_key<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _key: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn serialize_value<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }
}

impl SerializeStruct {
    fn serialize_field<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _key: &str,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn skip_field(self, _key: &str) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }
}

impl SerializeStructVariant {
    fn serialize_field<'a, S: ?Sized + serde::Serialize, T: wasmtime::AsContextMut>(
        self,
        _key: &str,
        _value: &HostsideSerializerClient<'a, S, T>,
    ) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }

    fn end(self) -> Result<SerOk, SerError> {
        todo!("wit-bindgen")
    }

    fn skip_field(self, _key: &str) -> (Self, Result<(), SerError>) {
        todo!("wit-bindgen")
    }
}
