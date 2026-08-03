use super::*;

pub mod DuperZoned {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalZonedDateTime, serde::temporal::TemporalString};
    use ::jiff::Zoned as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::ZonedDateTime(
            DuperTemporalZonedDateTime::try_from(Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::ZonedDateTime(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"ZonedDateTime",
            )),
        }
    }
}
pub mod DuperOptionZoned {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalZonedDateTime, serde::temporal::TemporalString};
    use ::jiff::Zoned as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => TemporalString::ZonedDateTime(
                DuperTemporalZonedDateTime::try_from(Cow::Owned(value.to_string()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer
                .serialize_newtype_struct("ZonedDateTime", &Option::<TemporalString>::None),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor
        where
            WrappedType: Deserialize<'de>,
        {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal ZonedDateTime")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::ZonedDateTime(inner) => Some(<WrappedType>::deserialize(
                        serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                    ))
                    .transpose(),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"ZonedDateTime",
                    )),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {})
    }
}

pub mod DuperTimestamp {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalInstant, serde::temporal::TemporalString};
    use ::jiff::Timestamp as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::Instant(
            DuperTemporalInstant::try_from(Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::Instant(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"Instant",
            )),
        }
    }
}
pub mod DuperOptionTimestamp {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalInstant, serde::temporal::TemporalString};
    use ::jiff::Timestamp as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => TemporalString::Instant(
                DuperTemporalInstant::try_from(Cow::Owned(value.to_string()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct("Instant", &Option::<TemporalString>::None),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor
        where
            WrappedType: Deserialize<'de>,
        {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal Instant")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::Instant(inner) => Some(<WrappedType>::deserialize(
                        serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                    ))
                    .transpose(),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"Instant",
                    )),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {})
    }
}

pub mod DuperSpan {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalDuration, serde::temporal::TemporalString};
    use ::jiff::Span as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::Duration(
            DuperTemporalDuration::try_from(Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::Duration(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"Duration",
            )),
        }
    }
}
pub mod DuperOptionSpan {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalDuration, serde::temporal::TemporalString};
    use ::jiff::Span as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => TemporalString::Duration(
                DuperTemporalDuration::try_from(Cow::Owned(value.to_string()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => {
                serializer.serialize_newtype_struct("Duration", &Option::<TemporalString>::None)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor
        where
            WrappedType: Deserialize<'de>,
        {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal Duration")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::Duration(inner) => Some(<WrappedType>::deserialize(
                        serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                    ))
                    .transpose(),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"Duration",
                    )),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {})
    }
}

pub mod DuperDate {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::serde::temporal::TemporalString;
    use ::jiff::civil::Date as WrappedType;
    use duper::DuperTemporalPlainDate;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::PlainDate(
            DuperTemporalPlainDate::try_from(Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::PlainDate(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainDate",
            )),
        }
    }
}
pub mod DuperOptionDate {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalPlainDate, serde::temporal::TemporalString};
    use ::jiff::civil::Date as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => TemporalString::PlainDate(
                DuperTemporalPlainDate::try_from(Cow::Owned(value.to_string()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => {
                serializer.serialize_newtype_struct("PlainDate", &Option::<TemporalString>::None)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor
        where
            WrappedType: Deserialize<'de>,
        {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal PlainDate")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::PlainDate(inner) => Some(<WrappedType>::deserialize(
                        serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                    ))
                    .transpose(),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"PlainDate",
                    )),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {})
    }
}

pub mod DuperTime {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::serde::temporal::TemporalString;
    use ::jiff::civil::Time as WrappedType;
    use duper::DuperTemporalPlainTime;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::PlainTime(
            DuperTemporalPlainTime::try_from(Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::PlainTime(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainTime",
            )),
        }
    }
}
pub mod DuperOptionTime {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalPlainTime, serde::temporal::TemporalString};
    use ::jiff::civil::Time as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => TemporalString::PlainTime(
                DuperTemporalPlainTime::try_from(Cow::Owned(value.to_string()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => {
                serializer.serialize_newtype_struct("PlainTime", &Option::<TemporalString>::None)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor
        where
            WrappedType: Deserialize<'de>,
        {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal PlainTime")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::PlainTime(inner) => Some(<WrappedType>::deserialize(
                        serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                    ))
                    .transpose(),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"PlainTime",
                    )),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {})
    }
}

pub mod DuperDateTime {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::serde::temporal::TemporalString;
    use ::jiff::civil::DateTime as WrappedType;
    use duper::DuperTemporalPlainDateTime;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::PlainDateTime(
            DuperTemporalPlainDateTime::try_from(Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::PlainDateTime(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainDateTime",
            )),
        }
    }
}
pub mod DuperOptionDateTime {
    use ::std::borrow::Cow;

    use super::*;
    use ::duper::{DuperTemporalPlainDateTime, serde::temporal::TemporalString};
    use ::jiff::civil::DateTime as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => TemporalString::PlainDateTime(
                DuperTemporalPlainDateTime::try_from(Cow::Owned(value.to_string()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer
                .serialize_newtype_struct("PlainDateTime", &Option::<TemporalString>::None),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor
        where
            WrappedType: Deserialize<'de>,
        {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal PlainDateTime")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::PlainDateTime(inner) => Some(<WrappedType>::deserialize(
                        serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                    ))
                    .transpose(),
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"PlainDateTime",
                    )),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {})
    }
}
