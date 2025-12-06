use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use napi::{
    Env,
    bindgen_prelude::{Array, BigInt, Null, Object, Uint8Array},
};
use serde_core::ser::Error;

#[derive(Debug, thiserror::Error)]
pub enum SerdeError {
    #[error("{0}")]
    Custom(String),
    #[error("NAPI error: {0}")]
    Napi(#[from] napi::Error),
}

impl serde_core::ser::Error for SerdeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SerdeError::Custom(msg.to_string())
    }
}

#[repr(transparent)]
pub(crate) struct DuperMetaSerializer<'a, 'ser> {
    _marker: PhantomData<Object<'a>>,
    env: &'ser Env,
}

impl<'ser> DuperMetaSerializer<'_, 'ser> {
    pub(crate) fn new(env: &'ser Env) -> Self {
        Self {
            _marker: Default::default(),
            env,
        }
    }
}

impl<'ser, 'a> serde_core::Serializer for &'ser DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    type SerializeSeq = DuperMetaSerializer<'a, 'ser>;
    type SerializeTuple = DuperMetaSerializer<'a, 'ser>;
    type SerializeTupleStruct = DuperMetaSerializer<'a, 'ser>;
    type SerializeTupleVariant = DuperMetaSerializer<'a, 'ser>;
    type SerializeMap = DuperMetaSerializer<'a, 'ser>;
    type SerializeStruct = DuperMetaStructSerializer<'a, 'ser>;
    type SerializeStructVariant = DuperMetaSerializer<'a, 'ser>;

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if name == "DuperValue" || len == 3 {
            Ok(Self::SerializeStruct {
                env: self.env,
                identifier: None,
                typ: None,
                inner: None,
            })
        } else {
            Err(SerdeError::custom(format!(
                "expected DuperValue struct with 3 fields, found {name} struct with {len} field(s)"
            )))
        }
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected bool".into()))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected i8".into()))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected i16".into()))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected i32".into()))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected i64".into()))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected u8".into()))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected u16".into()))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected u32".into()))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected u64".into()))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected f32".into()))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected f64".into()))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected char".into()))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected str".into()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected bytes".into()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected none".into()))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Err(SerdeError::Custom("unexpected some".into()))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected unit".into()))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom(format!("unexpected unit struct {name}")))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected unit variant {name}::{variant}"
        )))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Err(SerdeError::Custom(format!(
            "unexpected newtype struct {name}"
        )))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Err(SerdeError::Custom(format!(
            "unexpected newtype variant {name}::{variant}"
        )))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerdeError::Custom("unexpected seq".into()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerdeError::Custom("unexpected tuple".into()))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected tuple struct {name}"
        )))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected tuple variant {name}::{variant}"
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerdeError::Custom("unexpected map".into()))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected struct variant {name}::{variant}"
        )))
    }
}

impl<'a, 'ser> serde_core::ser::SerializeSeq for DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeTuple for DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeTupleStruct for DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeTupleVariant for DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeMap for DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeStructVariant for DuperMetaSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

pub(crate) enum DuperMetaInner<'a> {
    Object(Object<'a>),
    Array(Array<'a>),
    String(String),
    Bytes(Uint8Array),
    Integer(BigInt),
    Float(f64),
    Boolean(bool),
    Null(Null),
}

impl<'a> Debug for DuperMetaInner<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(_) => f.debug_tuple("Object").field(&"...").finish(),
            Self::Array(_) => f.debug_tuple("Array").field(&"...").finish(),
            Self::String(string) => f.debug_tuple("String").field(string).finish(),
            Self::Bytes(_) => f.debug_tuple("Bytes").field(&"...").finish(),
            Self::Integer(integer) => f.debug_tuple("Integer").field(integer).finish(),
            Self::Float(float) => f.debug_tuple("Float").field(float).finish(),
            Self::Boolean(boolean) => f.debug_tuple("Boolean").field(boolean).finish(),
            Self::Null(null) => f.debug_tuple("Null").field(null).finish(),
        }
    }
}

pub(crate) struct DuperMetaStructSerializer<'a, 'ser> {
    env: &'ser Env,
    identifier: Option<String>,
    typ: Option<String>,
    inner: Option<DuperMetaInner<'a>>,
}

impl<'a, 'ser> serde_core::ser::SerializeStruct for DuperMetaStructSerializer<'a, 'ser> {
    type Ok = Object<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        match key {
            "identifier" => {
                self.identifier = match value.serialize(&DuperMetaInnerSerializer::new(self.env))? {
                    DuperMetaInner::String(identifier) => Some(identifier),
                    DuperMetaInner::Null(Null) => None,
                    other => {
                        return Err(SerdeError::custom(format!(
                            "expected string or null, found {other:?}"
                        )));
                    }
                }
            }
            "type" => {
                self.typ = match value.serialize(&DuperMetaInnerSerializer::new(self.env))? {
                    DuperMetaInner::String(typ) => Some(typ),
                    other => {
                        return Err(SerdeError::custom(format!(
                            "expected string, found {other:?}"
                        )));
                    }
                }
            }
            "inner" => {
                self.inner = Some(value.serialize(&DuperMetaInnerSerializer::new(self.env))?)
            }
            _ => return Err(SerdeError::custom(format!("unexpected key {key}"))),
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut object = Object::new(self.env)?;
        object.set("identifier", self.identifier)?;
        object.set(
            "type",
            self.typ
                .ok_or_else(|| SerdeError::custom("missing type for Duper meta value"))?,
        )?;
        match self
            .inner
            .ok_or_else(|| SerdeError::custom("missing inner for Duper meta value"))?
        {
            DuperMetaInner::Object(inner) => object.set("inner", inner)?,
            DuperMetaInner::Array(inner) => object.set("inner", inner)?,
            DuperMetaInner::String(inner) => object.set("inner", inner)?,
            DuperMetaInner::Bytes(inner) => object.set("inner", inner)?,
            DuperMetaInner::Integer(inner) => object.set("inner", inner)?,
            DuperMetaInner::Float(inner) => object.set("inner", inner)?,
            DuperMetaInner::Boolean(inner) => object.set("inner", inner)?,
            DuperMetaInner::Null(inner) => object.set("inner", inner)?,
        };
        Ok(object)
    }
}

#[repr(transparent)]
pub(crate) struct DuperMetaInnerSerializer<'a, 'ser> {
    _marker: PhantomData<Object<'a>>,
    env: &'ser Env,
}

impl<'a, 'ser> DuperMetaInnerSerializer<'a, 'ser> {
    pub(crate) fn new(env: &'ser Env) -> Self {
        Self {
            _marker: Default::default(),
            env,
        }
    }
}

impl<'a, 'ser> serde_core::Serializer for &'ser DuperMetaInnerSerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    type SerializeSeq = DuperMetaInnerArraySerializer<'a, 'ser>;
    type SerializeTuple = DuperMetaInnerArraySerializer<'a, 'ser>;
    type SerializeTupleStruct = DuperMetaInnerSerializer<'a, 'ser>;
    type SerializeTupleVariant = DuperMetaInnerSerializer<'a, 'ser>;
    type SerializeMap = DuperMetaInnerMapSerializer<'a, 'ser>;
    type SerializeStruct = DuperMetaInnerSerializer<'a, 'ser>;
    type SerializeStructVariant = DuperMetaInnerSerializer<'a, 'ser>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Integer(BigInt::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected u64".into()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Float(v))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom("unexpected char".into()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Bytes(Uint8Array::new(v.to_vec())))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Null(Null))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Null(Null))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom(format!("unexpected unit struct {name}")))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected unit variant {name}::{variant}"
        )))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Err(SerdeError::Custom(format!(
            "unexpected newtype struct {name}"
        )))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Err(SerdeError::Custom(format!(
            "unexpected newtype variant {name}::{variant}"
        )))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq {
            env: self.env,
            array: Array::from_vec(self.env, Vec::<Object<'a>>::new())?,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::SerializeTuple {
            env: self.env,
            array: Array::from_vec(self.env, Vec::<Object<'a>>::new())?,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected newtype struct {name}"
        )))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected tuple variant {name}::{variant}"
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Self::SerializeMap {
            env: self.env,
            object: Object::new(self.env)?,
            key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerdeError::Custom(format!("unexpected struct {name}")))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerdeError::Custom(format!(
            "unexpected struct variant {name}::{variant}"
        )))
    }
}

impl<'a, 'ser> serde_core::ser::SerializeTupleStruct for DuperMetaInnerSerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeTupleVariant for DuperMetaInnerSerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeStruct for DuperMetaInnerSerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a, 'ser> serde_core::ser::SerializeStructVariant for DuperMetaInnerSerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

pub(crate) struct DuperMetaInnerArraySerializer<'a, 'ser> {
    env: &'ser Env,
    array: Array<'a>,
}

impl<'a, 'ser> serde_core::ser::SerializeSeq for DuperMetaInnerArraySerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Ok(self
            .array
            .insert(value.serialize(&DuperMetaSerializer::new(self.env))?)?)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Array(self.array))
    }
}

impl<'a, 'ser> serde_core::ser::SerializeTuple for DuperMetaInnerArraySerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Ok(self
            .array
            .insert(value.serialize(&DuperMetaSerializer::new(self.env))?)?)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Array(self.array))
    }
}

pub(crate) struct DuperMetaInnerMapSerializer<'a, 'ser> {
    env: &'ser Env,
    object: Object<'a>,
    key: Option<String>,
}

impl<'a, 'ser> serde_core::ser::SerializeMap for DuperMetaInnerMapSerializer<'a, 'ser> {
    type Ok = DuperMetaInner<'a>;
    type Error = SerdeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        self.key = match key.serialize(&DuperMetaInnerSerializer::new(self.env))? {
            DuperMetaInner::String(key) => Some(key),
            other => {
                return Err(SerdeError::custom(format!(
                    "expected string, found {other:?}"
                )));
            }
        };
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        let value = value.serialize(&DuperMetaSerializer::new(self.env))?;
        let Some(key) = self.key.take() else {
            return Err(SerdeError::custom("value was serialized without key"));
        };
        Ok(self.object.set(key, value)?)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperMetaInner::Object(self.object))
    }
}
