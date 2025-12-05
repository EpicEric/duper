use std::{borrow::Cow, marker::PhantomData};

use crate::{
    DuperIdentifier, DuperKey, DuperObject, DuperValue,
    PrettyPrinter as DuperPrettyPrinter, Serializer as DuperSerializer,
};
use serde_core::{Serialize, ser};

use super::error::DuperSerdeError;

/// A structure for serializing Rust values into Duper values.
#[derive(Clone, Default)]
pub struct Serializer<'a> {
    _marker: PhantomData<DuperValue<'a>>,
}

impl<'a> Serializer<'a> {
    /// Creates a new Duper serializer.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Serialize the given data structure as a Duper value.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of [`Serialize`] decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_duper<'a, T>(value: &'a T) -> Result<DuperValue<'a>, DuperSerdeError>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)
}

/// Serialize the given data structure as a [`String`] of a Duper value.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of [`Serialize`] decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_string<T>(value: &T) -> Result<String, DuperSerdeError>
where
    T: Serialize,
{
    Ok(DuperSerializer::new(false, false).serialize(&to_duper(value)?))
}

/// Serialize the given data structure as a [`String`] of a Duper value, minifying
/// the output.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of [`Serialize`] decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_string_compact<T>(value: &T) -> Result<String, DuperSerdeError>
where
    T: Serialize,
{
    Ok(DuperSerializer::new(false, true).serialize(&to_duper(value)?))
}

/// Serialize the given data structure as a [`String`] of a Duper value, minifying
/// and stripping identifiers from the output.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of [`Serialize`] decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_string_minified<T>(value: &T) -> Result<String, DuperSerdeError>
where
    T: Serialize,
{
    Ok(DuperSerializer::new(true, true).serialize(&to_duper(value)?))
}

/// Serialize the given data structure as a [`String`] of a Duper value,
/// pretty-printing the output.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of [`Serialize`] decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_string_pretty<T>(value: &T, indent: &str) -> Result<String, DuperSerdeError>
where
    T: Serialize,
{
    Ok(DuperPrettyPrinter::new(false, indent)
        .map_err(DuperSerdeError::invalid_value)?
        .pretty_print(&to_duper(value)?))
}

impl<'ser, 'a> ser::Serializer for &'ser mut Serializer<'a> {
    type Ok = DuperValue<'a>;

    type Error = DuperSerdeError;

    type SerializeSeq = SerializeSeq<'ser, 'a>;
    type SerializeTuple = SerializeTuple<'ser, 'a>;
    type SerializeTupleStruct = SerializeTupleStruct<'ser, 'a>;
    type SerializeTupleVariant = SerializeTupleVariant<'ser, 'a>;
    type SerializeMap = SerializeMap<'ser, 'a>;
    type SerializeStruct = SerializeStruct<'ser, 'a>;
    type SerializeStructVariant = SerializeStructVariant<'ser, 'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Boolean {
            identifier: None,
            inner: v,
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v,
        })
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        if let Ok(integer) = v.try_into() {
            Ok(DuperValue::Integer {
                identifier: None,
                inner: integer,
            })
        } else if let float = v as f64
            && float as i128 == v
        {
            Ok(DuperValue::Float {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("I128")).expect("valid identifier"),
                ),
                inner: float,
            })
        } else {
            Ok(DuperValue::String {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("I128")).expect("valid identifier"),
                ),
                inner: Cow::Owned(v.to_string()),
            })
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Integer {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if let Ok(integer) = v.try_into() {
            Ok(DuperValue::Integer {
                identifier: None,
                inner: integer,
            })
        } else if let float = v as f64
            && float.round() as u64 == v
        {
            Ok(DuperValue::Float {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("U64")).expect("valid identifier"),
                ),
                inner: float,
            })
        } else {
            Ok(DuperValue::String {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("U64")).expect("valid identifier"),
                ),
                inner: Cow::Owned(v.to_string()),
            })
        }
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        if let Ok(integer) = v.try_into() {
            Ok(DuperValue::Integer {
                identifier: None,
                inner: integer,
            })
        } else if let float = v as f64
            && float as u128 == v
        {
            Ok(DuperValue::Float {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("U128")).expect("valid identifier"),
                ),
                inner: float,
            })
        } else {
            Ok(DuperValue::String {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("U128")).expect("valid identifier"),
                ),
                inner: Cow::Owned(v.to_string()),
            })
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Float {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Float {
            identifier: None,
            inner: v.into(),
        })
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::String {
            identifier: Some(
                DuperIdentifier::try_from(Cow::Borrowed("Char")).expect("valid identifier"),
            ),
            inner: Cow::Owned(v.into()),
        })
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::String {
            identifier: None,
            inner: Cow::Owned(v.into()),
        })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Bytes {
            identifier: None,
            inner: Cow::Owned(v.into()),
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Null { identifier: None })
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Tuple {
            identifier: None,
            inner: vec![],
        })
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Tuple {
            identifier: (!name.is_empty())
                .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(name))?),
            inner: vec![],
        })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::String {
            identifier: (!name.is_empty())
                .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(name))?),
            inner: Cow::Borrowed(variant),
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
        Ok(value.serialize(self)?.with_identifier(
            (!name.is_empty()).then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(name))?),
        )?)
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
        Ok(DuperValue::Object {
            identifier: (!name.is_empty())
                .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(name))?),
            inner: DuperObject::try_from(vec![(DuperKey::from(Cow::Borrowed(variant)), value)])
                .expect("single item object"),
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
    type Error = DuperSerdeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Array {
            identifier: None,
            inner: self.elements,
        })
    }
}

pub struct SerializeTuple<'ser, 'a> {
    serializer: &'ser mut Serializer<'a>,
    elements: Vec<DuperValue<'a>>,
}

impl<'ser, 'a> ser::SerializeTuple for SerializeTuple<'ser, 'a> {
    type Ok = DuperValue<'a>;
    type Error = DuperSerdeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Tuple {
            identifier: None,
            inner: self.elements,
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
    type Error = DuperSerdeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Tuple {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(self.name))?),
            inner: self.elements,
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
    type Error = DuperSerdeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Object {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(self.name))?),
            inner: DuperObject::try_from(vec![(
                DuperKey::from(Cow::Borrowed(self.variant)),
                DuperValue::Tuple {
                    identifier: None,
                    inner: self.elements,
                },
            )])
            .expect("single item object"),
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
    type Error = DuperSerdeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key_value = key.serialize(&mut *self.serializer)?;
        match key_value {
            DuperValue::String {
                identifier,
                inner: s,
            } => {
                self.identifier = self.identifier.take().or(identifier);
                self.next_key = Some(DuperKey::from(s));
                Ok(())
            }
            _ => Err(DuperSerdeError::serialization("map key must be a string")),
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
            Err(DuperSerdeError::serialization(
                "serialize_value called before serialize_key",
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        Ok(DuperValue::Object {
            identifier: self.identifier.take(),
            inner: DuperObject::try_from(self.entries)?,
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
    type Error = DuperSerdeError;

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
        // Special handling for Temporal values
        if self.name == super::temporal::STRUCT {
            let mut typ: Option<DuperValue<'a>> = None;
            let mut value: Option<DuperValue<'a>> = None;
            for (key, val) in self.fields.into_iter() {
                match key.as_ref() {
                    super::temporal::FIELD_TYPE => typ = Some(val),
                    super::temporal::FIELD_VALUE => value = Some(val),
                    field => {
                        return Err(DuperSerdeError::invalid_value(format!(
                            "unknown field {field} for TemporalString",
                        )));
                    }
                }
            }

            let typ = typ.ok_or_else(|| {
                DuperSerdeError::invalid_value(format!(
                    "missing field {} for TemporalString",
                    super::temporal::FIELD_TYPE
                ))
            })?;
            let value = value.ok_or_else(|| {
                DuperSerdeError::invalid_value(format!(
                    "missing field {} for TemporalString",
                    super::temporal::FIELD_VALUE
                ))
            })?;

            match (typ, value) {
                (
                    DuperValue::String { inner: typ, .. },
                    DuperValue::String { inner: value, .. },
                ) => match typ.as_ref() {
                    "Instant" => Ok(DuperValue::try_instant_from(value)?),
                    "ZonedDateTime" => Ok(DuperValue::try_zoned_date_time_from(value)?),
                    "PlainDate" => Ok(DuperValue::try_plain_date_from(value)?),
                    "PlainTime" => Ok(DuperValue::try_plain_time_from(value)?),
                    "PlainDateTime" => Ok(DuperValue::try_plain_date_time_from(value)?),
                    "PlainYearMonth" => Ok(DuperValue::try_plain_year_month_from(value)?),
                    "PlainMonthDay" => Ok(DuperValue::try_plain_month_day_from(value)?),
                    "Duration" => Ok(DuperValue::try_duration_from(value)?),
                    "Unspecified" => Ok(DuperValue::try_unspecified_from(None, value)?),
                    _ => Err(DuperSerdeError::invalid_value(format!(
                        "invalid type {typ:?} for TemporalString",
                    ))),
                },
                _ => Err(DuperSerdeError::invalid_value(
                    "invalid fields for TemporalString",
                )),
            }
        } else {
            Ok(DuperValue::Object {
                identifier: (!self.name.is_empty())
                    .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(self.name))?),
                inner: DuperObject::try_from(self.fields)?,
            })
        }
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
    type Error = DuperSerdeError;

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
        Ok(DuperValue::Object {
            identifier: (!self.name.is_empty())
                .then_some(DuperIdentifier::try_from_lossy(Cow::Borrowed(self.name))?),
            inner: DuperObject::try_from(vec![(
                DuperKey::from(Cow::Borrowed(self.variant)),
                DuperValue::Object {
                    identifier: None,
                    inner: DuperObject::try_from(self.fields)?,
                },
            )])
            .expect("single item object"),
        })
    }
}
