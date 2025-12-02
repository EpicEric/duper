use std::{borrow::Cow, fmt::Display};

use serde_core::{
    Deserializer, Serialize,
    de::{Deserialize, Error, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
};

use crate::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTemporal, DuperTuple, DuperValue, serde::error::DuperSerdeError,
};

pub const TYPE_OBJECT: &str = "Object";
pub const TYPE_ARRAY: &str = "Array";
pub const TYPE_TUPLE: &str = "Tuple";
pub const TYPE_STRING: &str = "String";
pub const TYPE_BYTES: &str = "Bytes";
pub const TYPE_TEMPORAL: &str = "Temporal";
pub const TYPE_INTEGER: &str = "Integer";
pub const TYPE_FLOAT: &str = "Float";
pub const TYPE_BOOLEAN: &str = "Boolean";
pub const TYPE_NULL: &str = "Null";

impl<'a> DuperValue<'a> {
    /// A function that serializes the Duper value into a lossless struct
    /// containing the `identifier`, `inner`, and `type` fields.
    pub fn serialize_meta<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut state = serializer.serialize_struct("DuperValue", 3)?;
        state.serialize_field("identifier", &self.identifier)?;
        match &self.inner {
            DuperInner::Object(object) => {
                state.serialize_field("inner", &SerDuperObject(object))?;
                state.serialize_field("type", TYPE_OBJECT)?;
            }
            DuperInner::Array(array) => {
                state.serialize_field("inner", &SerDuperArray(array))?;
                state.serialize_field("type", TYPE_ARRAY)?;
            }
            DuperInner::Tuple(tuple) => {
                state.serialize_field("inner", &SerDuperTuple(tuple))?;
                state.serialize_field("type", TYPE_TUPLE)?;
            }
            DuperInner::String(string) => {
                state.serialize_field("inner", string.as_ref())?;
                state.serialize_field("type", TYPE_STRING)?;
            }
            DuperInner::Bytes(bytes) => {
                state.serialize_field("inner", &SerDuperBytes(bytes))?;
                state.serialize_field("type", TYPE_BYTES)?;
            }
            DuperInner::Temporal(temporal) => {
                state.serialize_field("inner", temporal.as_ref())?;
                state.serialize_field("type", TYPE_TEMPORAL)?;
            }
            DuperInner::Integer(integer) => {
                state.serialize_field("inner", integer)?;
                state.serialize_field("type", TYPE_INTEGER)?;
            }
            DuperInner::Float(float) => {
                state.serialize_field("inner", float)?;
                state.serialize_field("type", TYPE_FLOAT)?;
            }
            DuperInner::Boolean(bool) => {
                state.serialize_field("inner", bool)?;
                state.serialize_field("type", TYPE_BOOLEAN)?;
            }
            DuperInner::Null => {
                state.serialize_field("inner", &self.inner)?;
                state.serialize_field("type", TYPE_NULL)?;
            }
        }
        state.end()
    }

    /// A function that attempts to deserialize a struct containing
    /// `identifier`, `inner`, and `type` fields into the appropriate Duper
    /// value.
    pub fn deserialize_meta<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        'de: 'a,
    {
        let duper_value_meta = DeDuperValue::deserialize(deserializer)?;
        duper_value_meta.try_into().map_err(Error::custom)
    }
}

struct SerDuperValue<'b>(&'b DuperValue<'b>);

impl<'b> Serialize for SerDuperValue<'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        self.0.serialize_meta(serializer)
    }
}

struct SerDuperObject<'b>(&'b DuperObject<'b>);

impl<'b> Serialize for SerDuperObject<'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in self.0.iter() {
            map.serialize_entry(key, &SerDuperValue(value))?;
        }
        map.end()
    }
}

struct SerDuperArray<'b>(&'b DuperArray<'b>);

impl<'b> Serialize for SerDuperArray<'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for element in self.0.iter() {
            seq.serialize_element(&SerDuperValue(element))?;
        }
        seq.end()
    }
}

struct SerDuperTuple<'b>(&'b DuperTuple<'b>);

impl<'b> Serialize for SerDuperTuple<'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut tup = serializer.serialize_tuple(self.0.len())?;
        for element in self.0.iter() {
            tup.serialize_element(&SerDuperValue(element))?;
        }
        tup.end()
    }
}

struct SerDuperBytes<'b>(&'b DuperBytes<'b>);

impl<'b> Serialize for SerDuperBytes<'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serializer.serialize_bytes(self.0.as_ref())
    }
}

struct DeDuperValue<'b> {
    identifier: Option<DuperIdentifier<'b>>,
    inner: DeDuperInner<'b>,
}

enum DeDuperArrayValue<'b> {
    DuperValue(DeDuperValue<'b>),
    U8(u8),
}

impl<'b> TryFrom<DeDuperArrayValue<'b>> for DuperValue<'b> {
    type Error = DuperSerdeError;

    fn try_from(value: DeDuperArrayValue<'b>) -> Result<Self, Self::Error> {
        match value {
            DeDuperArrayValue::DuperValue(duper_value) => duper_value.try_into(),
            DeDuperArrayValue::U8(byte) => Ok(DuperValue {
                identifier: None,
                inner: DuperInner::Integer(byte.into()),
            }),
        }
    }
}

struct DeDuperObject<'b>(Vec<(DuperKey<'b>, DeDuperValue<'b>)>);
struct DeDuperArray<'b>(Vec<DeDuperArrayValue<'b>>);
struct DeDuperTuple<'b>(Vec<DeDuperValue<'b>>);
enum DeDuperTemporal<'b> {
    Instant(Cow<'b, str>),
    ZonedDateTime(Cow<'b, str>),
    PlainDate(Cow<'b, str>),
    PlainTime(Cow<'b, str>),
    PlainDateTime(Cow<'b, str>),
    PlainYearMonth(Cow<'b, str>),
    PlainMonthDay(Cow<'b, str>),
    Duration(Cow<'b, str>),
    Unspecified(Cow<'b, str>),
}

enum DeDuperInner<'b> {
    Object(DeDuperObject<'b>),
    Array(DeDuperArray<'b>),
    Tuple(DeDuperTuple<'b>),
    String(DuperString<'b>),
    Bytes(DuperBytes<'b>),
    Temporal(DeDuperTemporal<'b>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Display for DeDuperInner<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DeDuperInner::Object(_) => TYPE_OBJECT,
            DeDuperInner::Array(_) => TYPE_ARRAY,
            DeDuperInner::Tuple(_) => TYPE_TUPLE,
            DeDuperInner::String(_) => TYPE_STRING,
            DeDuperInner::Bytes(_) => TYPE_BYTES,
            DeDuperInner::Temporal(_) => TYPE_TEMPORAL,
            DeDuperInner::Integer(_) => TYPE_INTEGER,
            DeDuperInner::Float(_) => TYPE_FLOAT,
            DeDuperInner::Boolean(_) => TYPE_BOOLEAN,
            DeDuperInner::Null => TYPE_NULL,
        })
    }
}

impl<'b> TryFrom<DeDuperValue<'b>> for DuperValue<'b> {
    type Error = DuperSerdeError;

    fn try_from(value: DeDuperValue<'b>) -> Result<Self, Self::Error> {
        let DeDuperValue { identifier, inner } = value;
        match inner {
            DeDuperInner::Object(object) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Object(DuperObject::try_from(
                    object
                        .0
                        .into_iter()
                        .map(|(key, value)| DuperValue::try_from(value).map(|value| (key, value)))
                        .collect::<Result<Vec<_>, _>>()?,
                )?),
            }),
            DeDuperInner::Array(array) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Array(DuperArray::from(
                    array
                        .0
                        .into_iter()
                        .map(DuperValue::try_from)
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            }),
            DeDuperInner::Tuple(tuple) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Tuple(DuperTuple::from(
                    tuple
                        .0
                        .into_iter()
                        .map(DuperValue::try_from)
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            }),
            DeDuperInner::String(string) => Ok(DuperValue {
                identifier,
                inner: DuperInner::String(string),
            }),
            DeDuperInner::Bytes(bytes) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Bytes(bytes),
            }),
            DeDuperInner::Temporal(temporal) => Ok(DuperValue {
                inner: DuperInner::Temporal(match temporal {
                    DeDuperTemporal::Instant(inner) => DuperTemporal::try_instant_from(inner)?,
                    DeDuperTemporal::ZonedDateTime(inner) => {
                        DuperTemporal::try_zoned_date_time_from(inner)?
                    }
                    DeDuperTemporal::PlainDate(inner) => DuperTemporal::try_plain_date_from(inner)?,
                    DeDuperTemporal::PlainTime(inner) => DuperTemporal::try_plain_time_from(inner)?,
                    DeDuperTemporal::PlainDateTime(inner) => {
                        DuperTemporal::try_plain_date_time_from(inner)?
                    }
                    DeDuperTemporal::PlainYearMonth(inner) => {
                        DuperTemporal::try_plain_year_month_from(inner)?
                    }
                    DeDuperTemporal::PlainMonthDay(inner) => {
                        DuperTemporal::try_plain_month_day_from(inner)?
                    }
                    DeDuperTemporal::Duration(inner) => DuperTemporal::try_duration_from(inner)?,
                    DeDuperTemporal::Unspecified(inner) => {
                        DuperTemporal::try_unspecified_from(inner)?
                    }
                }),
                identifier,
            }),
            DeDuperInner::Integer(integet) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Integer(integet),
            }),
            DeDuperInner::Float(float) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Float(float),
            }),
            DeDuperInner::Boolean(boolean) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Boolean(boolean),
            }),
            DeDuperInner::Null => Ok(DuperValue {
                identifier,
                inner: DuperInner::Null,
            }),
        }
    }
}

struct DeDuperArrayValueVisitor;

impl<'de> Visitor<'de> for DeDuperArrayValueVisitor {
    type Value = DeDuperArrayValue<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a meta Duper array value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_bool(v)?,
        ))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if (0..=255).contains(&v) {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_i64(v)?,
            ))
        }
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if (0..=255).contains(&v) {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_i128(v)?,
            ))
        }
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::U8(v))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v <= 255 {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_u16(v)?,
            ))
        }
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v <= 255 {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_u32(v)?,
            ))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v <= 255 {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_u64(v)?,
            ))
        }
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v <= 255 {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_u128(v)?,
            ))
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if (0.0..=255.0).contains(&v) && v.fract() == 0.0 {
            Ok(DeDuperArrayValue::U8(v as u8))
        } else {
            Ok(DeDuperArrayValue::DuperValue(
                DeDuperValueVisitor.visit_f64(v)?,
            ))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_str(v)?,
        ))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_borrowed_str(v)?,
        ))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_string(v)?,
        ))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_bytes(v)?,
        ))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_borrowed_bytes(v)?,
        ))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_byte_buf(v)?,
        ))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_none()?,
        ))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_some(deserializer)?,
        ))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_unit()?,
        ))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_newtype_struct(deserializer)?,
        ))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_seq(seq)?,
        ))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(DeDuperArrayValue::DuperValue(
            DeDuperValueVisitor.visit_map(map)?,
        ))
    }
}

impl<'de> Deserialize<'de> for DeDuperArrayValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeDuperArrayValueVisitor {})
    }
}

struct DeDuperInnerVisitor;

impl<'de> Visitor<'de> for DeDuperInnerVisitor {
    type Value = DeDuperInner<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a meta Duper inner value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Boolean(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Integer(v))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i64::try_from(v) {
            Ok(DeDuperInner::Integer(v))
        } else if let float = v as f64
            && float as i128 == v
        {
            Ok(DeDuperInner::Float(float))
        } else {
            Ok(DeDuperInner::String(DuperString::from(v.to_string())))
        }
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i64::try_from(v) {
            Ok(DeDuperInner::Integer(v))
        } else if let float = v as f64
            && float as u64 == v
        {
            Ok(DeDuperInner::Float(float))
        } else {
            Ok(DeDuperInner::String(DuperString::from(v.to_string())))
        }
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i64::try_from(v) {
            Ok(DeDuperInner::Integer(v))
        } else if let float = v as f64
            && float as u128 == v
        {
            Ok(DeDuperInner::Float(float))
        } else {
            Ok(DeDuperInner::String(DuperString::from(v.to_string())))
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::String(DuperString::from(Cow::Borrowed(v))))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::String(DuperString::from(v)))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_byte_buf(v.to_vec())
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Bytes(DuperBytes::from(Cow::Borrowed(v))))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Bytes(DuperBytes::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperInner::Tuple(DeDuperTuple(vec![])))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = seq
            .size_hint()
            .map(|len| Vec::with_capacity(len))
            .unwrap_or_default();
        while let Some(element) = seq.next_element()? {
            vec.push(element);
        }
        Ok(DeDuperInner::Array(DeDuperArray(vec)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut vec = map
            .size_hint()
            .map(|len| Vec::with_capacity(len))
            .unwrap_or_default();
        while let Some(element) = map.next_entry()? {
            vec.push(element);
        }
        Ok(DeDuperInner::Object(DeDuperObject(vec)))
    }
}

impl<'de> Deserialize<'de> for DeDuperInner<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeDuperInnerVisitor {})
    }
}

struct DeDuperValueVisitor;

enum DeDuperType {
    Object,
    Array,
    Tuple,
    String,
    Bytes,
    Temporal,
    Integer,
    Float,
    Boolean,
    Null,
}

impl Display for DeDuperType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DeDuperType::Object => TYPE_OBJECT,
            DeDuperType::Array => TYPE_ARRAY,
            DeDuperType::Tuple => TYPE_TUPLE,
            DeDuperType::String => TYPE_STRING,
            DeDuperType::Bytes => TYPE_BYTES,
            DeDuperType::Temporal => TYPE_TEMPORAL,
            DeDuperType::Integer => TYPE_INTEGER,
            DeDuperType::Float => TYPE_FLOAT,
            DeDuperType::Boolean => TYPE_BOOLEAN,
            DeDuperType::Null => TYPE_NULL,
        })
    }
}

impl<'b> TryFrom<&'b str> for DeDuperType {
    type Error = &'b str;

    fn try_from(value: &'b str) -> Result<Self, Self::Error> {
        match value {
            TYPE_OBJECT => Ok(DeDuperType::Object),
            TYPE_ARRAY => Ok(DeDuperType::Array),
            TYPE_TUPLE => Ok(DeDuperType::Tuple),
            TYPE_STRING => Ok(DeDuperType::String),
            TYPE_BYTES => Ok(DeDuperType::Bytes),
            TYPE_TEMPORAL => Ok(DeDuperType::Temporal),
            TYPE_INTEGER => Ok(DeDuperType::Integer),
            TYPE_FLOAT => Ok(DeDuperType::Float),
            TYPE_BOOLEAN => Ok(DeDuperType::Boolean),
            TYPE_NULL => Ok(DeDuperType::Null),
            _ => Err(value),
        }
    }
}

enum DeDuperIdentifier<'de> {
    None,
    Some(Cow<'de, str>),
}

struct DeDuperIdentifierVisitor;

impl<'de> Visitor<'de> for DeDuperIdentifierVisitor {
    type Value = DeDuperIdentifier<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a meta Duper identifier")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperIdentifier::None)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperIdentifier::Some(Cow::Owned(v)))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DeDuperIdentifier::Some(Cow::Borrowed(v)))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(self)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_string())
    }
}

impl<'de> Deserialize<'de> for DeDuperIdentifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeDuperIdentifierVisitor)
    }
}

impl<'de> Visitor<'de> for DeDuperValueVisitor {
    type Value = DeDuperValue<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a meta Duper value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut found_identifier = false;
        let mut identifier: Option<DuperIdentifier<'de>> = None;
        let mut inner: Option<DeDuperInner<'de>> = None;
        let mut typ: Option<DeDuperType> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "identifier" => {
                    if found_identifier {
                        return Err(Error::duplicate_field("identifier"));
                    }
                    found_identifier = true;
                    identifier = match map.next_value::<DeDuperIdentifier>()? {
                        DeDuperIdentifier::None => None,
                        DeDuperIdentifier::Some(identifier) => {
                            Some(DuperIdentifier::try_from(identifier).map_err(|error| {
                                A::Error::custom(format!("failed to parse identifier: {error}"))
                            })?)
                        }
                    };
                }
                "inner" => {
                    if inner.is_some() {
                        return Err(Error::duplicate_field("inner"));
                    }
                    let wrapper: DeDuperInner = map.next_value()?;
                    inner = Some(wrapper);
                }
                "type" => {
                    if typ.is_some() {
                        return Err(Error::duplicate_field("type"));
                    }
                    let typ_tag: String = map.next_value()?;
                    typ = Some(DeDuperType::try_from(typ_tag.as_str()).map_err(|_|
                            Error::invalid_value(
                                serde_core::de::Unexpected::Str(&typ_tag),
                                &"one of: Object, Array, Tuple, String, Bytes, Temporal, Integer, Float, Boolean, Null",
                            ))?);
                }
                key => {
                    return Err(Error::unknown_field(key, &["identifier", "inner", "type"]));
                }
            }
        }

        let inner = inner.ok_or_else(|| Error::missing_field("inner"))?;
        let typ = typ.ok_or_else(|| Error::missing_field("type"))?;

        let inner = match (inner, typ) {
            // Direct matches
            (DeDuperInner::Object(object), DeDuperType::Object) => DeDuperInner::Object(object),
            (DeDuperInner::Array(array), DeDuperType::Array) => DeDuperInner::Array(array),
            (DeDuperInner::Tuple(tuple), DeDuperType::Tuple) => DeDuperInner::Tuple(tuple),
            (DeDuperInner::String(string), DeDuperType::String) => DeDuperInner::String(string),
            (DeDuperInner::Bytes(bytes), DeDuperType::Bytes) => DeDuperInner::Bytes(bytes),
            (DeDuperInner::Temporal(temporal), DeDuperType::Temporal) => {
                DeDuperInner::Temporal(temporal)
            }
            (DeDuperInner::Integer(integer), DeDuperType::Integer) => {
                DeDuperInner::Integer(integer)
            }
            (DeDuperInner::Float(float), DeDuperType::Float) => DeDuperInner::Float(float),
            (DeDuperInner::Boolean(boolean), DeDuperType::Boolean) => {
                DeDuperInner::Boolean(boolean)
            }
            (DeDuperInner::Null, DeDuperType::Null) => DeDuperInner::Null,
            // Swapped arrays/tuples
            (DeDuperInner::Array(array), DeDuperType::Tuple) => DeDuperInner::Tuple(DeDuperTuple(
                array
                    .0
                    .into_iter()
                    .map(|element| match element {
                        DeDuperArrayValue::DuperValue(duper_value) => duper_value,
                        DeDuperArrayValue::U8(byte) => DeDuperValue {
                            identifier: None,
                            inner: DeDuperInner::Integer(byte.into()),
                        },
                    })
                    .collect(),
            )),
            (DeDuperInner::Tuple(tuple), DeDuperType::Array) => DeDuperInner::Array(DeDuperArray(
                tuple
                    .0
                    .into_iter()
                    .map(DeDuperArrayValue::DuperValue)
                    .collect(),
            )),
            // Safe integer wrappers
            (DeDuperInner::Float(float), DeDuperType::Integer)
                if matches!(
                    identifier.as_ref().map(AsRef::as_ref),
                    Some("I128") | Some("U64") | Some("U128")
                ) =>
            {
                DeDuperInner::Float(float)
            }
            (DeDuperInner::String(string), DeDuperType::Integer)
                if matches!(
                    identifier.as_ref().map(AsRef::as_ref),
                    Some("I128") | Some("U64") | Some("U128")
                ) =>
            {
                DeDuperInner::String(string)
            }
            // Temporal from string
            (DeDuperInner::String(string), DeDuperType::Temporal) => match &identifier {
                Some(ident) if ident.as_ref() == "Instant" => {
                    DeDuperInner::Temporal(DeDuperTemporal::Instant(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "ZonedDateTime" => {
                    DeDuperInner::Temporal(DeDuperTemporal::ZonedDateTime(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "PlainDate" => {
                    DeDuperInner::Temporal(DeDuperTemporal::PlainDate(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "PlainTime" => {
                    DeDuperInner::Temporal(DeDuperTemporal::PlainTime(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "PlainDateTime" => {
                    DeDuperInner::Temporal(DeDuperTemporal::PlainDateTime(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "PlainYearMonth" => {
                    DeDuperInner::Temporal(DeDuperTemporal::PlainYearMonth(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "PlainMonthDay" => {
                    DeDuperInner::Temporal(DeDuperTemporal::PlainMonthDay(string.into_inner()))
                }
                Some(ident) if ident.as_ref() == "Duration" => {
                    DeDuperInner::Temporal(DeDuperTemporal::Duration(string.into_inner()))
                }
                Some(_) | None => {
                    DeDuperInner::Temporal(DeDuperTemporal::Unspecified(string.into_inner()))
                }
            },
            // Bytes from array of numbers
            (DeDuperInner::Array(array), DeDuperType::Bytes) => {
                let mut bytes = Vec::with_capacity(array.0.len());
                for element in array.0 {
                    if let DeDuperArrayValue::U8(byte) = element {
                        bytes.push(byte);
                    } else {
                        return Err(Error::custom(
                            "'Array' contains non-byte values".to_string(),
                        ));
                    }
                }
                DeDuperInner::Bytes(DuperBytes(Cow::Owned(bytes)))
            }
            // Array from bytes
            (DeDuperInner::Bytes(bytes), DeDuperType::Array) => DeDuperInner::Array(DeDuperArray(
                bytes
                    .0
                    .iter()
                    .map(|byte| {
                        DeDuperArrayValue::DuperValue(DeDuperValue {
                            identifier: None,
                            inner: DeDuperInner::Integer((*byte).into()),
                        })
                    })
                    .collect(),
            )),
            // Null from unit tuple
            (DeDuperInner::Tuple(tuple), DeDuperType::Null) if tuple.0.is_empty() => {
                DeDuperInner::Null
            }
            // Fallback
            (inner, typ) => {
                return Err(Error::custom(format!(
                    "type '{typ}' doesn't match inner type '{inner}'"
                )));
            }
        };

        Ok(DeDuperValue { identifier, inner })
    }
}

impl<'de> Deserialize<'de> for DeDuperValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "DuperValue",
            &["identifier", "inner", "type"],
            DeDuperValueVisitor,
        )
    }
}

#[cfg(test)]
mod serde_meta_tests {
    use std::borrow::Cow;

    use insta::assert_snapshot;

    use crate::{
        DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
        DuperTemporal, DuperTemporalInner, DuperTuple, DuperValue, PrettyPrinter,
        serde::{de::Deserializer, ser::Serializer},
    };

    fn serialize_meta(value: &DuperValue<'_>) -> String {
        let ser = value
            .serialize_meta(&mut Serializer::new())
            .expect("should serialize");
        PrettyPrinter::new(false, "  ").unwrap().pretty_print(ser)
    }

    fn deserialize_meta(value: &str) -> DuperValue<'_> {
        DuperValue::deserialize_meta(&mut Deserializer::from_string(value).expect("should parse"))
            .expect("should deserialize")
    }

    #[test]
    fn serialize_object() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(deserialized.identifier, None);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperInner::Object(DuperObject(vec![(
                DuperKey::from("foo"),
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                    inner: DuperInner::Object(DuperObject(vec![])),
                },
            )])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(
            deserialized.identifier,
            Some(DuperIdentifier(Cow::Borrowed("Outer")))
        );
    }

    #[test]
    fn serialize_array() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Array(DuperArray(vec![])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(deserialized.identifier, None);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperInner::Array(DuperArray(vec![DuperValue {
                identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                inner: DuperInner::Array(DuperArray(vec![])),
            }])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(
            deserialized.identifier,
            Some(DuperIdentifier(Cow::Borrowed("Outer")))
        );
    }

    #[test]
    fn serialize_tuple() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Tuple(DuperTuple(vec![])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(deserialized.identifier, None);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("Outer").expect("valid identifier")),
            inner: DuperInner::Tuple(DuperTuple(vec![DuperValue {
                identifier: Some(DuperIdentifier::try_from("Inner").expect("valid identifier")),
                inner: DuperInner::Tuple(DuperTuple(vec![])),
            }])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(
            deserialized.identifier,
            Some(DuperIdentifier(Cow::Borrowed("Outer")))
        );
    }

    #[test]
    fn serialize_scalars() {
        let value = DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject(vec![
                (
                    DuperKey::from("string"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::String(DuperString::from("Hello world!")),
                    },
                ),
                (
                    DuperKey::from("bytes"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Bytes(DuperBytes::from(&br"/\"[..])),
                    },
                ),
                (
                    DuperKey::from("temporal"),
                    DuperValue {
                        identifier: Some(DuperIdentifier(Cow::Borrowed("Instant"))),
                        inner: DuperInner::Temporal(DuperTemporal::Instant(DuperTemporalInner(
                            Cow::Borrowed("2022-02-28T03:06:00.092121729Z"),
                        ))),
                    },
                ),
                (
                    DuperKey::from("integer"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Integer(1337),
                    },
                ),
                (
                    DuperKey::from("float"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Float(8.25),
                    },
                ),
                (
                    DuperKey::from("boolean"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Boolean(true),
                    },
                ),
                (
                    DuperKey::from("null"),
                    DuperValue {
                        identifier: None,
                        inner: DuperInner::Null,
                    },
                ),
            ])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(deserialized.identifier, None);

        let value = DuperValue {
            identifier: Some(DuperIdentifier::try_from("MyScalars").expect("valid identifier")),
            inner: DuperInner::Array(DuperArray(vec![
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyString").expect("valid identifier"),
                    ),
                    inner: DuperInner::String(DuperString::from("Hello world!")),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyBytes").expect("valid identifier"),
                    ),
                    inner: DuperInner::Bytes(DuperBytes::from(&br"/\"[..])),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyTemporal").expect("valid identifier"),
                    ),
                    inner: DuperInner::Temporal(DuperTemporal::Unspecified(DuperTemporalInner(
                        Cow::Borrowed("2022-02-28T03:06:00.092121729Z"),
                    ))),
                },
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("MyInt").expect("valid identifier")),
                    inner: DuperInner::Integer(1337),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyFloat").expect("valid identifier"),
                    ),
                    inner: DuperInner::Float(8.25),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("MyBool").expect("valid identifier"),
                    ),
                    inner: DuperInner::Boolean(true),
                },
                DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from("Mysterious").expect("valid identifier"),
                    ),
                    inner: DuperInner::Null,
                },
            ])),
        };
        let serialized = serialize_meta(&value);
        assert_snapshot!(serialized);
        let deserialized = deserialize_meta(&serialized);
        assert_eq!(value, deserialized);
        assert_eq!(
            deserialized.identifier,
            Some(DuperIdentifier(Cow::Borrowed("MyScalars")))
        );
    }
}
