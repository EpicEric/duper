use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperString, DuperTemporal, DuperTuple,
    DuperValue,
};

use crate::filter::{DuperFilter, IsTruthyFilter};

#[derive(Debug, Clone)]
pub(crate) enum DuperType {
    Object,
    Array,
    Tuple,
    String,
    Bytes,
    TemporalInstant,
    TemporalZonedDateTime,
    TemporalPlainDate,
    TemporalPlainTime,
    TemporalPlainDateTime,
    TemporalPlainYearMonth,
    TemporalPlainMonthDay,
    TemporalDuration,
    TemporalUnspecified,
    Integer,
    Float,
    Number,
    Boolean,
    Null,
}

impl DuperType {
    pub(crate) fn cast<'value>(
        &self,
        value: &'value DuperValue<'value>,
    ) -> Option<DuperValue<'value>> {
        match (self, &value.inner) {
            // Trivial casts
            (DuperType::Object, DuperInner::Object(_)) => Some(value.clone()),
            (DuperType::Array, DuperInner::Array(_)) => Some(value.clone()),
            (DuperType::Tuple, DuperInner::Tuple(_)) => Some(value.clone()),
            (DuperType::String, DuperInner::String(_)) => Some(value.clone()),
            (DuperType::Bytes, DuperInner::Bytes(_)) => Some(value.clone()),
            (
                DuperType::TemporalInstant
                | DuperType::TemporalZonedDateTime
                | DuperType::TemporalPlainDate
                | DuperType::TemporalPlainTime
                | DuperType::TemporalPlainDateTime
                | DuperType::TemporalPlainYearMonth
                | DuperType::TemporalPlainMonthDay
                | DuperType::TemporalDuration
                | DuperType::TemporalUnspecified,
                DuperInner::Temporal(_),
            ) => Some(value.clone()),
            (DuperType::Integer, DuperInner::Integer(_)) => Some(value.clone()),
            (DuperType::Float, DuperInner::Float(_)) => Some(value.clone()),
            (DuperType::Number, DuperInner::Integer(_) | DuperInner::Float(_)) => {
                Some(value.clone())
            }
            (DuperType::Boolean, DuperInner::Boolean(_)) => Some(value.clone()),
            (DuperType::Null, DuperInner::Null) => Some(value.clone()),

            // Non-trivial casts
            (DuperType::Array, DuperInner::Tuple(tuple)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Array(DuperArray::from(
                    tuple.iter().cloned().collect::<Vec<_>>(),
                )),
            }),
            (DuperType::Tuple, DuperInner::Array(array)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Tuple(DuperTuple::from(
                    array.iter().cloned().collect::<Vec<_>>(),
                )),
            }),
            (DuperType::String, DuperInner::Bytes(bytes)) => str::from_utf8(bytes.as_ref())
                .ok()
                .map(|string| DuperValue {
                    identifier: value.identifier.clone(),
                    inner: DuperInner::String(DuperString::from(Cow::Borrowed(string))),
                }),
            (DuperType::String, DuperInner::Temporal(temporal)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::String(DuperString::from(Cow::Borrowed(temporal.as_ref()))),
            }),
            (DuperType::Bytes, DuperInner::String(string)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(
                    string.as_ref().as_bytes(),
                ))),
            }),
            (DuperType::Bytes, DuperInner::Temporal(temporal)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Bytes(DuperBytes::from(Cow::Borrowed(
                    temporal.as_ref().as_bytes(),
                ))),
            }),
            (DuperType::TemporalInstant, DuperInner::String(string)) => {
                DuperTemporal::try_instant_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("Instant").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalInstant, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| DuperTemporal::try_instant_from(Cow::Borrowed(string)).ok())
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("Instant").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalZonedDateTime, DuperInner::String(string)) => {
                DuperTemporal::try_zoned_date_time_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("ZonedDateTime").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalZonedDateTime, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_zoned_date_time_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("ZonedDateTime").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainDate, DuperInner::String(string)) => {
                DuperTemporal::try_plain_date_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainDate").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainDate, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_plain_date_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainDate").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainTime, DuperInner::String(string)) => {
                DuperTemporal::try_plain_time_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainTime").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainTime, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_plain_time_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainTime").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainDateTime, DuperInner::String(string)) => {
                DuperTemporal::try_plain_date_time_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainDateTime").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainDateTime, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_plain_date_time_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainDateTime").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainYearMonth, DuperInner::String(string)) => {
                DuperTemporal::try_plain_year_month_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainYearMonth").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainYearMonth, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_plain_year_month_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainYearMonth").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainMonthDay, DuperInner::String(string)) => {
                DuperTemporal::try_plain_month_day_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainMonthDay").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalPlainMonthDay, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_plain_month_day_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("PlainMonthDay").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalDuration, DuperInner::String(string)) => {
                DuperTemporal::try_duration_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("Duration").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalDuration, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| DuperTemporal::try_duration_from(Cow::Borrowed(string)).ok())
                    .map(|temporal| DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from("Duration").expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalUnspecified, DuperInner::String(string)) => {
                DuperTemporal::try_unspecified_from(Cow::Borrowed(string.as_ref()))
                    .ok()
                    .map(|temporal| DuperValue {
                        identifier: None,
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::TemporalUnspecified, DuperInner::Bytes(bytes)) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| {
                        DuperTemporal::try_unspecified_from(Cow::Borrowed(string)).ok()
                    })
                    .map(|temporal| DuperValue {
                        identifier: None,
                        inner: DuperInner::Temporal(temporal),
                    })
            }
            (DuperType::Integer, DuperInner::Float(float)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Integer(*float as i64),
            }),
            (DuperType::Float, DuperInner::Integer(integer)) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Float(*integer as f64),
            }),
            (DuperType::Boolean, _) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Boolean(IsTruthyFilter.filter(value)),
            }),
            (DuperType::Null, _) => Some(DuperValue {
                identifier: value.identifier.clone(),
                inner: DuperInner::Null,
            }),

            // Unsupported casts
            _ => None,
        }
    }
}
