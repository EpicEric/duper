use super::*;

pub mod DuperNaiveDateTime {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::NaiveDateTime as WrappedType;
    use duper::{DuperTemporalPlainDateTime, serde::temporal::TemporalString};

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::PlainDateTime(
            DuperTemporalPlainDateTime::try_from(Cow::Owned(format!("{value:?}")))
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
pub mod DuperOptionNaiveDateTime {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::NaiveDateTime as WrappedType;
    use duper::{DuperTemporalPlainDateTime, serde::temporal::TemporalString};

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        Option<WrappedType>: Serialize,
    {
        match value {
            Some(value) => TemporalString::PlainDateTime(
                DuperTemporalPlainDateTime::try_from(Cow::Owned(format!("{value:?}")))
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

pub mod DuperNaiveDate {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::NaiveDate as WrappedType;
    use duper::{DuperTemporalPlainDate, serde::temporal::TemporalString};

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::PlainDate(
            DuperTemporalPlainDate::try_from(Cow::Owned(format!("{value:?}")))
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
pub mod DuperOptionNaiveDate {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::NaiveDate as WrappedType;
    use duper::{DuperTemporalPlainDate, serde::temporal::TemporalString};

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        Option<WrappedType>: Serialize,
    {
        match value {
            Some(value) => TemporalString::PlainDate(
                DuperTemporalPlainDate::try_from(Cow::Owned(format!("{value:?}")))
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

pub mod DuperNaiveTime {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::NaiveTime as WrappedType;
    use duper::{DuperTemporalPlainTime, serde::temporal::TemporalString};

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalString::PlainTime(
            DuperTemporalPlainTime::try_from(Cow::Owned(format!("{value:?}")))
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
pub mod DuperOptionNaiveTime {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::NaiveTime as WrappedType;
    use duper::{DuperTemporalPlainTime, serde::temporal::TemporalString};

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        Option<WrappedType>: Serialize,
    {
        match value {
            Some(value) => TemporalString::PlainTime(
                DuperTemporalPlainTime::try_from(Cow::Owned(format!("{value:?}")))
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

duper_serde_module!(
    DuperTimeDelta,
    DuperOptionTimeDelta,
    ::chrono::TimeDelta,
    "TimeDelta"
);

pub mod DuperDateTime {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::{DateTime as WrappedType, TimeZone};
    use duper::{DuperTemporalInstant, serde::temporal::TemporalString};

    pub fn serialize<S, T>(value: &WrappedType<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: TimeZone,
    {
        TemporalString::Instant(
            DuperTemporalInstant::try_from(Cow::Owned(value.to_rfc3339()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<WrappedType<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: TimeZone,
        WrappedType<T>: Deserialize<'de>,
    {
        match TemporalString::deserialize(deserializer)? {
            TemporalString::Instant(inner) => <WrappedType<T>>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"Instant",
            )),
        }
    }
}
pub mod DuperOptionDateTime {
    use std::borrow::Cow;

    use super::*;
    use ::chrono::{DateTime as WrappedType, TimeZone};
    use duper::{DuperTemporalInstant, serde::temporal::TemporalString};

    pub fn serialize<S, T>(value: &Option<WrappedType<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: TimeZone,
        Option<WrappedType<T>>: Serialize,
    {
        match value {
            Some(value) => TemporalString::Instant(
                DuperTemporalInstant::try_from(Cow::Owned(value.to_rfc3339()))
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct("Instant", &None),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<WrappedType<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: TimeZone,
        WrappedType<T>: Deserialize<'de>,
    {
        struct Visitor<T: TimeZone> {
            _marker: ::std::marker::PhantomData<T>,
        }

        impl<'de, T> de::Visitor<'de> for Visitor<T>
        where
            T: TimeZone,
            WrappedType<T>: Deserialize<'de>,
        {
            type Value = Option<WrappedType<T>>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional Temporal Instant")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match TemporalString::deserialize(deserializer)? {
                    TemporalString::Instant(inner) => Some(<WrappedType<T>>::deserialize(
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

        deserializer.deserialize_option(Visitor {
            _marker: Default::default(),
        })
    }
}
