//! A utility for better serialization/deserialization support of Temporal values.

use std::{borrow::Cow, marker::PhantomData};

use serde_core::{de::IntoDeserializer, ser::SerializeStruct};

use crate::{
    DuperTemporal, DuperTemporalDuration, DuperTemporalInstant, DuperTemporalPlainDate,
    DuperTemporalPlainDateTime, DuperTemporalPlainMonthDay, DuperTemporalPlainTime,
    DuperTemporalPlainYearMonth, DuperTemporalUnspecified, DuperTemporalZonedDateTime,
};

/// An internal string to identify a value as a [`TemporalString`].
pub const STRUCT: &str = "$__duper_private_TemporalString";
/// An internal string to identify a [`TemporalString`]'s type.
pub const FIELD_TYPE: &str = "$__duper_private_type";
/// An internal string to identify a [`TemporalString`]'s value.
pub const FIELD_VALUE: &str = "$__duper_private_value";

/// A parsed Duper Temporal value.
///
/// This structure represents the Temporal primitive that can be encoded into
/// Duper values. This type is a parsed version that contains all metadata
/// internally. You can use this for where you're expecting a Temporal value
/// to be specified.
///
/// Also note that, while this type implements `Serialize` and `Deserialize`,
/// it's only recommended to use this type with the Duper format. Otherwise,
/// the metadata encoded in other formats may look a little odd.
///
/// This can represent any of the Temporal values (`Instant`, `ZonedDateTime`,
/// `PlainDate`, `PlainTime`, `PlainDateTime`, `PlainYearMonth`,
/// `PlainMonthDay`, `Duration`), including unspecified ones.
///
/// # Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use duper::{DuperTemporalPlainYearMonth, serde::temporal::TemporalString};
///
/// #[derive(Serialize, Deserialize)]
/// struct MyType<'a> {
///     inner: TemporalString<'a>,
/// }
///
/// let item = MyType {
///     inner: TemporalString::from(DuperTemporalPlainYearMonth::try_from(
///         std::borrow::Cow::Borrowed("2023-10-05T14:30:00+00:00")
///     ).unwrap()),
/// };
///
/// let output = duper::serde::ser::to_string(&item).unwrap();
/// let deserialized: MyType<'_> = duper::serde::de::from_string(&output).unwrap();
/// assert!(matches!(deserialized.inner, TemporalString::PlainYearMonth(_)));
/// ```
#[derive(Debug)]
pub enum TemporalString<'a> {
    Instant(DuperTemporalInstant<'a>),
    ZonedDateTime(DuperTemporalZonedDateTime<'a>),
    PlainDate(DuperTemporalPlainDate<'a>),
    PlainTime(DuperTemporalPlainTime<'a>),
    PlainDateTime(DuperTemporalPlainDateTime<'a>),
    PlainYearMonth(DuperTemporalPlainYearMonth<'a>),
    PlainMonthDay(DuperTemporalPlainMonthDay<'a>),
    Duration(DuperTemporalDuration<'a>),
    Unspecified(DuperTemporalUnspecified<'a>),
}

impl TemporalString<'_> {
    pub fn name(&self) -> &'static str {
        match self {
            TemporalString::Instant(_) => "Instant",
            TemporalString::ZonedDateTime(_) => "ZonedDateTime",
            TemporalString::PlainDate(_) => "PlainDate",
            TemporalString::PlainTime(_) => "PlainTime",
            TemporalString::PlainDateTime(_) => "PlainDateTime",
            TemporalString::PlainYearMonth(_) => "PlainYearMonth",
            TemporalString::PlainMonthDay(_) => "PlainMonthDay",
            TemporalString::Duration(_) => "Duration",
            TemporalString::Unspecified(_) => "Temporal",
        }
    }
}

impl<'a> From<DuperTemporal<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporal<'a>) -> Self {
        match value {
            DuperTemporal::Instant { inner } => Self::Instant(inner),
            DuperTemporal::ZonedDateTime { inner } => Self::ZonedDateTime(inner),
            DuperTemporal::PlainDate { inner } => Self::PlainDate(inner),
            DuperTemporal::PlainTime { inner } => Self::PlainTime(inner),
            DuperTemporal::PlainDateTime { inner } => Self::PlainDateTime(inner),
            DuperTemporal::PlainYearMonth { inner } => Self::PlainYearMonth(inner),
            DuperTemporal::PlainMonthDay { inner } => Self::PlainMonthDay(inner),
            DuperTemporal::Duration { inner } => Self::Duration(inner),
            DuperTemporal::Unspecified { inner, .. } => Self::Unspecified(inner),
        }
    }
}

impl<'a> From<DuperTemporalInstant<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalInstant<'a>) -> Self {
        Self::Instant(value)
    }
}

impl<'a> From<DuperTemporalZonedDateTime<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalZonedDateTime<'a>) -> Self {
        Self::ZonedDateTime(value)
    }
}

impl<'a> From<DuperTemporalPlainDate<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalPlainDate<'a>) -> Self {
        Self::PlainDate(value)
    }
}

impl<'a> From<DuperTemporalPlainTime<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalPlainTime<'a>) -> Self {
        Self::PlainTime(value)
    }
}

impl<'a> From<DuperTemporalPlainDateTime<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalPlainDateTime<'a>) -> Self {
        Self::PlainDateTime(value)
    }
}

impl<'a> From<DuperTemporalPlainYearMonth<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalPlainYearMonth<'a>) -> Self {
        Self::PlainYearMonth(value)
    }
}

impl<'a> From<DuperTemporalPlainMonthDay<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalPlainMonthDay<'a>) -> Self {
        Self::PlainMonthDay(value)
    }
}

impl<'a> From<DuperTemporalDuration<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalDuration<'a>) -> Self {
        Self::Duration(value)
    }
}

impl<'a> From<DuperTemporalUnspecified<'a>> for TemporalString<'a> {
    fn from(value: DuperTemporalUnspecified<'a>) -> Self {
        Self::Unspecified(value)
    }
}

impl<'a> serde_core::Serialize for TemporalString<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let (typ, value) = match self {
            TemporalString::Instant(inner) => ("Instant", inner.as_ref()),
            TemporalString::ZonedDateTime(inner) => ("ZonedDateTime", inner.as_ref()),
            TemporalString::PlainDate(inner) => ("PlainDate", inner.as_ref()),
            TemporalString::PlainTime(inner) => ("PlainTime", inner.as_ref()),
            TemporalString::PlainDateTime(inner) => ("PlainDateTime", inner.as_ref()),
            TemporalString::PlainYearMonth(inner) => ("PlainYearMonth", inner.as_ref()),
            TemporalString::PlainMonthDay(inner) => ("PlainMonthDay", inner.as_ref()),
            TemporalString::Duration(inner) => ("Duration", inner.as_ref()),
            TemporalString::Unspecified(inner) => ("Temporal", inner.as_ref()),
        };
        let mut s = serializer.serialize_struct(STRUCT, 2)?;
        s.serialize_field(FIELD_TYPE, typ)?;
        s.serialize_field(FIELD_VALUE, value)?;
        s.end()
    }
}

impl<'a, 'de> serde_core::Deserialize<'de> for TemporalString<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        struct TemporalStringVisitor<'a> {
            _marker: PhantomData<TemporalString<'a>>,
        }

        impl<'a, 'de> serde_core::de::Visitor<'de> for TemporalStringVisitor<'a> {
            type Value = TemporalString<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Temporal string")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde_core::de::MapAccess<'de>,
            {
                let mut typ: Option<String> = None;
                let mut value: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        FIELD_TYPE => typ = Some(map.next_value()?),
                        FIELD_VALUE => value = Some(map.next_value()?),
                        key => {
                            return Err(serde_core::de::Error::unknown_field(
                                key,
                                &[FIELD_TYPE, FIELD_VALUE],
                            ));
                        }
                    }
                }

                let typ = typ.ok_or_else(|| serde_core::de::Error::missing_field(FIELD_TYPE))?;
                let value =
                    value.ok_or_else(|| serde_core::de::Error::missing_field(FIELD_VALUE))?;

                match typ.as_str() {
                    "Instant" => Ok(TemporalString::Instant(
                        DuperTemporalInstant::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "ZonedDateTime" => Ok(TemporalString::ZonedDateTime(
                        DuperTemporalZonedDateTime::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "PlainDate" => Ok(TemporalString::PlainDate(
                        DuperTemporalPlainDate::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "PlainTime" => Ok(TemporalString::PlainTime(
                        DuperTemporalPlainTime::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "PlainDateTime" => Ok(TemporalString::PlainDateTime(
                        DuperTemporalPlainDateTime::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "PlainYearMonth" => Ok(TemporalString::PlainYearMonth(
                        DuperTemporalPlainYearMonth::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "PlainMonthDay" => Ok(TemporalString::PlainMonthDay(
                        DuperTemporalPlainMonthDay::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "Duration" => Ok(TemporalString::Duration(
                        DuperTemporalDuration::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    "Temporal" => Ok(TemporalString::Unspecified(
                        DuperTemporalUnspecified::try_from(Cow::Owned(value))
                            .map_err(serde_core::de::Error::custom)?,
                    )),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ),
                        &"one of: Instant, ZonedDateTime, PlainDate, PlainTime, PlainDateTime, PlainYearMonth, PlainMonthDay, Duration, Temporal",
                    )),
                }
            }
        }

        deserializer.deserialize_struct(
            STRUCT,
            &[FIELD_TYPE, FIELD_VALUE],
            TemporalStringVisitor {
                _marker: Default::default(),
            },
        )
    }
}

impl<'de, 'a, E> IntoDeserializer<'de, E> for TemporalString<'a>
where
    E: serde_core::de::Error,
{
    type Deserializer = TemporalStringDeserializer<'a, E>;

    fn into_deserializer(self) -> Self::Deserializer {
        TemporalStringDeserializer {
            temporal: self,
            _error: Default::default(),
        }
    }
}

pub struct TemporalStringDeserializer<'de, E> {
    temporal: TemporalString<'de>,
    _error: core::marker::PhantomData<E>,
}

impl<'de, 'a, E> serde_core::Deserializer<'de> for TemporalStringDeserializer<'a, E>
where
    E: serde_core::de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let map = TemporalStringMapDeserializer::new(self.temporal);
        visitor.visit_map(map)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let cow = match self.temporal {
            TemporalString::Instant(inner) => inner.into_inner(),
            TemporalString::ZonedDateTime(inner) => inner.into_inner(),
            TemporalString::PlainDate(inner) => inner.into_inner(),
            TemporalString::PlainTime(inner) => inner.into_inner(),
            TemporalString::PlainDateTime(inner) => inner.into_inner(),
            TemporalString::PlainYearMonth(inner) => inner.into_inner(),
            TemporalString::PlainMonthDay(inner) => inner.into_inner(),
            TemporalString::Duration(inner) => inner.into_inner(),
            TemporalString::Unspecified(inner) => inner.into_inner(),
        };
        match cow {
            Cow::Borrowed(borrowed) => visitor.visit_str(borrowed),
            Cow::Owned(owned) => visitor.visit_string(owned),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    serde_core::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct TemporalStringMapDeserializer<'a, E> {
    typ: Option<&'static str>,
    value: Option<Cow<'a, str>>,
    _error: core::marker::PhantomData<E>,
}

impl<'a, E> TemporalStringMapDeserializer<'a, E> {
    fn new(temporal: TemporalString<'a>) -> Self {
        Self {
            typ: Some(temporal.name()),
            value: Some(match temporal {
                TemporalString::Instant(inner) => inner.into_inner(),
                TemporalString::ZonedDateTime(inner) => inner.into_inner(),
                TemporalString::PlainDate(inner) => inner.into_inner(),
                TemporalString::PlainTime(inner) => inner.into_inner(),
                TemporalString::PlainDateTime(inner) => inner.into_inner(),
                TemporalString::PlainYearMonth(inner) => inner.into_inner(),
                TemporalString::PlainMonthDay(inner) => inner.into_inner(),
                TemporalString::Duration(inner) => inner.into_inner(),
                TemporalString::Unspecified(inner) => inner.into_inner(),
            }),
            _error: Default::default(),
        }
    }
}

impl<'de, 'a, E> serde_core::de::MapAccess<'de> for TemporalStringMapDeserializer<'a, E>
where
    E: serde_core::de::Error,
{
    type Error = E;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde_core::de::DeserializeSeed<'de>,
    {
        if self.typ.is_some() {
            seed.deserialize(FIELD_TYPE.into_deserializer()).map(Some)
        } else if self.value.is_some() {
            seed.deserialize(FIELD_VALUE.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::DeserializeSeed<'de>,
    {
        if let Some(typ) = self.typ.take() {
            seed.deserialize(typ.into_deserializer())
        } else if let Some(value) = self.value.take() {
            seed.deserialize(value.as_ref().into_deserializer())
        } else {
            Err(serde_core::de::Error::custom("map is exhausted"))
        }
    }
}
