use super::*;

pub mod DuperInstant {
    use super::*;
    use ::temporal_rs::Instant as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::Instant(
            ::duper::DuperTemporalInstant::try_from(::std::borrow::Cow::Owned(
                value
                    .to_ixdtf_string(None, Default::default())
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            ))
            .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::Instant(inner) => <WrappedType>::deserialize(
                serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
            ),
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"Instant",
            )),
        }
    }
}
pub mod DuperOptionInstant {
    use super::*;
    use ::temporal_rs::Instant as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::Instant(
                ::duper::DuperTemporalInstant::try_from(::std::borrow::Cow::Owned(
                    value
                        .to_ixdtf_string(None, Default::default())
                        .map_err(|err| {
                            <S::Error as serde_core::ser::Error>::custom(err.to_string())
                        })?,
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "Instant",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::Instant(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
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

pub mod DuperZonedDateTime {
    use super::*;
    use ::temporal_rs::ZonedDateTime as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::ZonedDateTime(
            ::duper::DuperTemporalZonedDateTime::try_from(::std::borrow::Cow::Owned(
                value.to_string(),
            ))
            .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::ZonedDateTime(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainTime",
            )),
        }
    }
}
pub mod DuperOptionZonedDateTime {
    use super::*;
    use ::temporal_rs::ZonedDateTime as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::ZonedDateTime(
                ::duper::DuperTemporalZonedDateTime::try_from(::std::borrow::Cow::Owned(
                    value.to_string(),
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "ZonedDateTime",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::ZonedDateTime(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
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

pub mod DuperPlainDate {
    use super::*;
    use ::temporal_rs::PlainDate as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::PlainDate(
            ::duper::DuperTemporalPlainDate::try_from(::std::borrow::Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::PlainDate(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainTime",
            )),
        }
    }
}
pub mod DuperOptionPlainDate {
    use super::*;
    use ::temporal_rs::PlainDate as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::PlainDate(
                ::duper::DuperTemporalPlainDate::try_from(::std::borrow::Cow::Owned(
                    value.to_string(),
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "PlainDate",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::PlainDate(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
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

pub mod DuperPlainTime {
    use super::*;
    use ::temporal_rs::PlainTime as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::PlainTime(
            ::duper::DuperTemporalPlainTime::try_from(::std::borrow::Cow::Owned(
                value
                    .to_ixdtf_string(Default::default())
                    .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            ))
            .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::PlainTime(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainTime",
            )),
        }
    }
}
pub mod DuperOptionPlainTime {
    use super::*;
    use ::temporal_rs::PlainTime as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::PlainTime(
                ::duper::DuperTemporalPlainTime::try_from(::std::borrow::Cow::Owned(
                    value.to_ixdtf_string(Default::default()).map_err(|err| {
                        <S::Error as serde_core::ser::Error>::custom(err.to_string())
                    })?,
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "PlainTime",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::PlainTime(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
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

pub mod DuperPlainDateTime {
    use super::*;
    use ::temporal_rs::PlainDateTime as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::PlainDateTime(
            ::duper::DuperTemporalPlainDateTime::try_from(::std::borrow::Cow::Owned(
                value.to_string(),
            ))
            .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::PlainDateTime(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainDateTime",
            )),
        }
    }
}
pub mod DuperOptionPlainDateTime {
    use super::*;
    use ::temporal_rs::PlainDateTime as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::PlainDateTime(
                ::duper::DuperTemporalPlainDateTime::try_from(::std::borrow::Cow::Owned(
                    value.to_string(),
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "PlainDateTime",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::PlainDateTime(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
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

pub mod DuperPlainYearMonth {
    use super::*;
    use ::temporal_rs::PlainYearMonth as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::PlainYearMonth(
            ::duper::DuperTemporalPlainYearMonth::try_from(::std::borrow::Cow::Owned(
                value.to_string(),
            ))
            .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::PlainYearMonth(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainYearMonth",
            )),
        }
    }
}
pub mod DuperOptionPlainYearMonth {
    use super::*;
    use ::temporal_rs::PlainYearMonth as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::PlainYearMonth(
                ::duper::DuperTemporalPlainYearMonth::try_from(::std::borrow::Cow::Owned(
                    value.to_string(),
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "PlainYearMonth",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                formatter.write_str("an optional Temporal PlainYearMonth")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::PlainYearMonth(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"PlainYearMonth",
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

pub mod DuperPlainMonthDay {
    use super::*;
    use ::temporal_rs::PlainMonthDay as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::PlainMonthDay(
            ::duper::DuperTemporalPlainMonthDay::try_from(::std::borrow::Cow::Owned(
                value.to_string(),
            ))
            .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::PlainMonthDay(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"PlainMonthDay",
            )),
        }
    }
}
pub mod DuperOptionPlainMonthDay {
    use super::*;
    use ::temporal_rs::PlainMonthDay as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::PlainMonthDay(
                ::duper::DuperTemporalPlainMonthDay::try_from(::std::borrow::Cow::Owned(
                    value.to_string(),
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "PlainMonthDay",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                formatter.write_str("an optional Temporal PlainMonthDay")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::PlainMonthDay(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
                    typ => Err(serde_core::de::Error::invalid_value(
                        serde_core::de::Unexpected::Str(typ.name()),
                        &"PlainMonthDay",
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

pub mod DuperDuration {
    use super::*;
    use ::temporal_rs::Duration as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::duper::serde::temporal::TemporalString::Duration(
            ::duper::DuperTemporalDuration::try_from(::std::borrow::Cow::Owned(value.to_string()))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
        )
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
        WrappedType: Deserialize<'de>,
    {
        match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
            ::duper::serde::temporal::TemporalString::Duration(inner) => {
                <WrappedType>::deserialize(serde_core::de::IntoDeserializer::into_deserializer(
                    inner.as_ref(),
                ))
            }
            typ => Err(serde_core::de::Error::invalid_value(
                serde_core::de::Unexpected::Str(typ.name()),
                &"Duration",
            )),
        }
    }
}
pub mod DuperOptionDuration {
    use super::*;
    use ::temporal_rs::Duration as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => ::duper::serde::temporal::TemporalString::Duration(
                ::duper::DuperTemporalDuration::try_from(::std::borrow::Cow::Owned(
                    value.to_string(),
                ))
                .map_err(|err| <S::Error as serde_core::ser::Error>::custom(err.to_string()))?,
            )
            .serialize(serializer),
            None => serializer.serialize_newtype_struct(
                "Duration",
                &Option::<::duper::serde::temporal::TemporalString>::None,
            ),
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
                match ::duper::serde::temporal::TemporalString::deserialize(deserializer)? {
                    ::duper::serde::temporal::TemporalString::Duration(inner) => {
                        Some(<WrappedType>::deserialize(
                            serde_core::de::IntoDeserializer::into_deserializer(inner.as_ref()),
                        ))
                        .transpose()
                    }
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
