use std::{borrow::Cow, marker::PhantomData};

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue, PrettyPrinter as DuperPrettyPrinter, Serializer as DuperSerializer,
};
use serde_core::{Serialize, ser};

use crate::Error;

#[derive(Clone)]
pub struct Serializer<'a> {
    _phantom: PhantomData<DuperValue<'a>>,
}

impl<'a> Serializer<'a> {
    fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

pub fn to_duper<'a, T>(value: &'a T) -> Result<DuperValue<'a>, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    Ok(DuperSerializer::new(false).serialize(to_duper(value)?))
}

pub fn to_string_minified<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    Ok(DuperSerializer::new(true).serialize(to_duper(value)?))
}

pub fn to_string_pretty<T>(value: &T, indent: usize) -> Result<String, Error>
where
    T: Serialize,
{
    Ok(DuperPrettyPrinter::new(false, indent).pretty_print(to_duper(value)?))
}

impl<'a, 'b> ser::Serializer for &'a mut Serializer<'b> {
    type Ok = DuperValue<'b>;

    type Error = Error;

    type SerializeSeq = SerializeSeq<'a, 'b>;
    type SerializeTuple = SerializeTuple<'a, 'b>;
    type SerializeTupleStruct = SerializeTupleStruct<'a, 'b>;
    type SerializeTupleVariant = SerializeTupleVariant<'a, 'b>;
    type SerializeMap = SerializeMap<'a, 'b>;
    type SerializeStruct = SerializeStruct<'a, 'b>;
    type SerializeStructVariant = SerializeStructVariant<'a, 'b>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Boolean(v),
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v.into()),
        })
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v.into()),
        })
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v.into()),
        })
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v),
        })
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        if let Ok(integer) = v.try_into() {
            Ok(DuperValue {
                identifier: None,
                inner: DuperInner::Integer(integer),
            })
        } else if let float = v as f64
            && float as i128 == v
        {
            Ok(DuperValue {
                identifier: Some(DuperIdentifier::from(Cow::Borrowed("I128"))),
                inner: DuperInner::Float(float),
            })
        } else {
            Ok(DuperValue {
                identifier: Some(DuperIdentifier::from(Cow::Borrowed("I128"))),
                inner: DuperInner::String(DuperString::from(Cow::Owned(v.to_string()))),
            })
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v.into()),
        })
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v.into()),
        })
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Integer(v.into()),
        })
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if let Ok(integer) = v.try_into() {
            Ok(DuperValue {
                identifier: None,
                inner: DuperInner::Integer(integer),
            })
        } else if let float = v as f64
            && float.round() as u64 == v
        {
            Ok(DuperValue {
                identifier: Some(DuperIdentifier::from(Cow::Borrowed("U64"))),
                inner: DuperInner::Float(float),
            })
        } else {
            Ok(DuperValue {
                identifier: Some(DuperIdentifier::from(Cow::Borrowed("U64"))),
                inner: DuperInner::String(DuperString::from(Cow::Owned(v.to_string()))),
            })
        }
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        if let Ok(integer) = v.try_into() {
            Ok(DuperValue {
                identifier: None,
                inner: DuperInner::Integer(integer),
            })
        } else if let float = v as f64
            && float as u128 == v
        {
            Ok(DuperValue {
                identifier: Some(DuperIdentifier::from(Cow::Borrowed("U128"))),
                inner: DuperInner::Float(float),
            })
        } else {
            Ok(DuperValue {
                identifier: Some(DuperIdentifier::from(Cow::Borrowed("U128"))),
                inner: DuperInner::String(DuperString::from(Cow::Owned(v.to_string()))),
            })
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Float(v.into()),
        })
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Float(v),
        })
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: Some(DuperIdentifier::from(Cow::Borrowed("Char"))),
            inner: DuperInner::String(DuperString::from(Cow::Owned(v.into()))),
        })
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::String(DuperString::from(Cow::Owned(v.into()))),
        })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(v.into()))),
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Null,
        })
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Tuple(DuperTuple::from(Vec::new())),
        })
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: (!name.is_empty()).then_some(DuperIdentifier::from(Cow::Borrowed(name))),
            inner: DuperInner::Tuple(DuperTuple::from(Vec::new())),
        })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: (!name.is_empty()).then_some(DuperIdentifier::from(Cow::Borrowed(name))),
            inner: DuperInner::String(DuperString::from(Cow::Borrowed(variant))),
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(self)?;
        Ok(DuperValue {
            identifier: (!name.is_empty())
                .then_some(DuperIdentifier::from(Cow::Borrowed(name)))
                .or(value.identifier),
            inner: value.inner,
        })
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(self)?;
        Ok(DuperValue {
            identifier: (!name.is_empty()).then_some(DuperIdentifier::from(Cow::Borrowed(name))),
            inner: DuperInner::Object(DuperObject::from(vec![(
                DuperKey::from(Cow::Borrowed(variant)),
                value,
            )])),
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq {
            serializer: self,
            elements: len.map(|len| Vec::with_capacity(len)).unwrap_or_default(),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::SerializeTuple {
            serializer: self,
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Self::SerializeTupleStruct {
            serializer: self,
            name,
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Self::SerializeTupleVariant {
            serializer: self,
            name,
            variant,
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Self::SerializeMap {
            serializer: self,
            identifier: None,
            entries: len.map(|len| Vec::with_capacity(len)).unwrap_or_default(),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::SerializeStruct {
            serializer: self,
            name,
            fields: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Self::SerializeStructVariant {
            serializer: self,
            name,
            variant,
            fields: Vec::with_capacity(len),
        })
    }
}

pub struct SerializeSeq<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    elements: Vec<DuperValue<'a>>,
}

impl<'ser, 'a> ser::SerializeSeq for SerializeSeq<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray::from(self.elements)),
        })
    }
}

pub struct SerializeTuple<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    elements: Vec<DuperValue<'a>>,
}

impl<'ser, 'a> ser::SerializeTuple for SerializeTuple<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: None,
            inner: DuperInner::Tuple(DuperTuple::from(self.elements)),
        })
    }
}

pub struct SerializeTupleStruct<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    name: &'static str,
    elements: Vec<DuperValue<'a>>,
}

// Serialize struct Rgb(u8, u8, u8) as Rgb((..., ..., ...))
impl<'ser, 'a> ser::SerializeTupleStruct for SerializeTupleStruct<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::from(Cow::Borrowed(self.name))),
            inner: DuperInner::Tuple(DuperTuple::from(self.elements)),
        })
    }
}

pub struct SerializeTupleVariant<'ser, 'b> {
    serializer: &'ser mut Serializer<'b>,
    name: &'static str,
    variant: &'static str,
    elements: Vec<DuperValue<'b>>,
}

// Serialize enum E { T(u8, u8) } as E({T: (..., ...)})
impl<'ser, 'b> ser::SerializeTupleVariant for SerializeTupleVariant<'ser, 'b> {
    type Ok = DuperValue<'b>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::from(Cow::Borrowed(self.name))),
            inner: DuperInner::Object(DuperObject::from(vec![(
                DuperKey::from(Cow::Borrowed(self.variant)),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Tuple(DuperTuple::from(self.elements)),
                },
            )])),
        })
    }
}

pub struct SerializeMap<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    identifier: Option<DuperIdentifier<'a>>,
    entries: Vec<(DuperKey<'a>, DuperValue<'a>)>,
    next_key: Option<DuperKey<'a>>,
}

impl<'ser, 'a> ser::SerializeMap for SerializeMap<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key_value = key.serialize(&mut *self.serializer)?;
        match key_value {
            DuperValue {
                identifier,
                inner: DuperInner::String(s),
            } => {
                self.identifier = self.identifier.take().or(identifier);
                self.next_key = Some(DuperKey::from(s.into_inner()));
                Ok(())
            }
            _ => Err(Error::serialization("map key must be a string")),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if let Some(key) = self.next_key.take() {
            let value = value.serialize(&mut *self.serializer)?;
            self.entries.push((key, value));
            Ok(())
        } else {
            Err(Error::serialization(
                "serialize_value called before serialize_key",
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: self.identifier.take(),
            inner: DuperInner::Object(DuperObject::from(self.entries)),
        })
    }
}

pub struct SerializeStruct<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    name: &'static str,
    fields: Vec<(DuperKey<'a>, DuperValue<'a>)>,
}

// Serialize struct Rgb { r: u8, g: u8, b: u8 } as Rgb({r: ..., g: ..., b: ...})
impl<'ser, 'a> ser::SerializeStruct for SerializeStruct<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.fields
            .push((DuperKey::from(Cow::Borrowed(key)), value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::from(Cow::Borrowed(self.name))),
            inner: DuperInner::Object(DuperObject::from(self.fields)),
        })
    }
}

pub struct SerializeStructVariant<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    name: &'static str,
    variant: &'static str,
    fields: Vec<(DuperKey<'a>, DuperValue<'a>)>,
}

// Serialize enum E { S { x: i32, y: String } } as E({S: {x: ..., y: ...}})
impl<'ser, 'a> ser::SerializeStructVariant for SerializeStructVariant<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.fields
            .push((DuperKey::from(Cow::Borrowed(key)), value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::from(Cow::Borrowed(self.name))),
            inner: DuperInner::Object(DuperObject::from(vec![(
                DuperKey::from(Cow::Borrowed(self.variant)),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Object(DuperObject::from(self.fields)),
                },
            )])),
        })
    }
}
