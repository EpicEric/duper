use std::borrow::Cow;

use duper::{DuperTemporal, DuperValue};

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
        match (self, &value) {
            // Trivial casts
            (DuperType::Object, DuperValue::Object { .. }) => Some(value.clone()),
            (DuperType::Array, DuperValue::Array { .. }) => Some(value.clone()),
            (DuperType::Tuple, DuperValue::Tuple { .. }) => Some(value.clone()),
            (DuperType::String, DuperValue::String { .. }) => Some(value.clone()),
            (DuperType::Bytes, DuperValue::Bytes { .. }) => Some(value.clone()),
            (DuperType::TemporalInstant, DuperValue::Temporal(DuperTemporal::Instant { .. })) => {
                Some(value.clone())
            }
            (
                DuperType::TemporalZonedDateTime,
                DuperValue::Temporal(DuperTemporal::ZonedDateTime { .. }),
            ) => Some(value.clone()),
            (
                DuperType::TemporalPlainDate,
                DuperValue::Temporal(DuperTemporal::PlainDate { .. }),
            ) => Some(value.clone()),
            (
                DuperType::TemporalPlainTime,
                DuperValue::Temporal(DuperTemporal::PlainTime { .. }),
            ) => Some(value.clone()),
            (
                DuperType::TemporalPlainDateTime,
                DuperValue::Temporal(DuperTemporal::PlainDateTime { .. }),
            ) => Some(value.clone()),
            (
                DuperType::TemporalPlainYearMonth,
                DuperValue::Temporal(DuperTemporal::PlainYearMonth { .. }),
            ) => Some(value.clone()),
            (
                DuperType::TemporalPlainMonthDay,
                DuperValue::Temporal(DuperTemporal::PlainMonthDay { .. }),
            ) => Some(value.clone()),
            (DuperType::TemporalDuration, DuperValue::Temporal(DuperTemporal::Duration { .. })) => {
                Some(value.clone())
            }
            (DuperType::TemporalUnspecified, DuperValue::Temporal(_)) => Some(value.clone()),
            (DuperType::Integer, DuperValue::Integer { .. }) => Some(value.clone()),
            (DuperType::Float, DuperValue::Float { .. }) => Some(value.clone()),
            (DuperType::Number, DuperValue::Integer { .. } | DuperValue::Float { .. }) => {
                Some(value.clone())
            }
            (DuperType::Boolean, DuperValue::Boolean { .. }) => Some(value.clone()),
            (DuperType::Null, DuperValue::Null { .. }) => Some(value.clone()),

            // Non-trivial casts
            (
                DuperType::Array,
                DuperValue::Tuple {
                    inner: tuple,
                    identifier,
                },
            ) => Some(DuperValue::Array {
                identifier: identifier.clone(),
                inner: tuple.to_vec(),
            }),
            (
                DuperType::Tuple,
                DuperValue::Array {
                    inner: array,
                    identifier,
                },
            ) => Some(DuperValue::Tuple {
                identifier: identifier.clone(),
                inner: array.to_vec(),
            }),
            (
                DuperType::String,
                DuperValue::Bytes {
                    inner: bytes,
                    identifier,
                },
            ) => str::from_utf8(bytes.as_ref())
                .ok()
                .map(|string| DuperValue::String {
                    identifier: identifier.clone(),
                    inner: Cow::Borrowed(string),
                }),
            (DuperType::String, DuperValue::Temporal(temporal)) => Some(DuperValue::String {
                identifier: temporal.identifier().clone(),
                inner: Cow::Borrowed(temporal.as_ref()),
            }),
            (
                DuperType::Bytes,
                DuperValue::String {
                    inner: string,
                    identifier,
                },
            ) => Some(DuperValue::Bytes {
                identifier: identifier.clone(),
                inner: Cow::Borrowed(string.as_ref().as_bytes()),
            }),
            (DuperType::Bytes, DuperValue::Temporal(temporal)) => Some(DuperValue::Bytes {
                identifier: temporal.identifier().clone(),
                inner: Cow::Borrowed(temporal.as_ref().as_bytes()),
            }),
            (DuperType::TemporalInstant, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_instant_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalInstant, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| DuperValue::try_instant_from(Cow::Borrowed(string)).ok())
            }
            (DuperType::TemporalZonedDateTime, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_zoned_date_time_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalZonedDateTime, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref()).ok().and_then(|string| {
                    DuperValue::try_zoned_date_time_from(Cow::Borrowed(string)).ok()
                })
            }
            (DuperType::TemporalPlainDate, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_plain_date_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalPlainDate, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| DuperValue::try_plain_date_from(Cow::Borrowed(string)).ok())
            }
            (DuperType::TemporalPlainTime, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_plain_time_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalPlainTime, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| DuperValue::try_plain_time_from(Cow::Borrowed(string)).ok())
            }
            (DuperType::TemporalPlainDateTime, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_plain_date_time_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalPlainDateTime, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref()).ok().and_then(|string| {
                    DuperValue::try_plain_date_time_from(Cow::Borrowed(string)).ok()
                })
            }
            (DuperType::TemporalPlainYearMonth, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_plain_year_month_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalPlainYearMonth, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref()).ok().and_then(|string| {
                    DuperValue::try_plain_year_month_from(Cow::Borrowed(string)).ok()
                })
            }
            (DuperType::TemporalPlainMonthDay, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_plain_month_day_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalPlainMonthDay, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref()).ok().and_then(|string| {
                    DuperValue::try_plain_month_day_from(Cow::Borrowed(string)).ok()
                })
            }
            (DuperType::TemporalDuration, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_duration_from(Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalDuration, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref())
                    .ok()
                    .and_then(|string| DuperValue::try_duration_from(Cow::Borrowed(string)).ok())
            }
            (DuperType::TemporalUnspecified, DuperValue::String { inner: string, .. }) => {
                DuperValue::try_unspecified_from(None, Cow::Borrowed(string.as_ref())).ok()
            }
            (DuperType::TemporalUnspecified, DuperValue::Bytes { inner: bytes, .. }) => {
                str::from_utf8(bytes.as_ref()).ok().and_then(|string| {
                    DuperValue::try_unspecified_from(None, Cow::Borrowed(string)).ok()
                })
            }
            (DuperType::TemporalInstant, DuperValue::Temporal(temporal)) => {
                DuperValue::try_instant_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (DuperType::TemporalZonedDateTime, DuperValue::Temporal(temporal)) => {
                DuperValue::try_zoned_date_time_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (DuperType::TemporalPlainDate, DuperValue::Temporal(temporal)) => {
                DuperValue::try_plain_date_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (DuperType::TemporalPlainTime, DuperValue::Temporal(temporal)) => {
                DuperValue::try_plain_time_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (DuperType::TemporalPlainDateTime, DuperValue::Temporal(temporal)) => {
                DuperValue::try_plain_date_time_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (DuperType::TemporalPlainYearMonth, DuperValue::Temporal(temporal)) => {
                DuperValue::try_plain_year_month_from(Cow::Owned(temporal.as_ref().to_string()))
                    .ok()
            }
            (DuperType::TemporalPlainMonthDay, DuperValue::Temporal(temporal)) => {
                DuperValue::try_plain_month_day_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (DuperType::TemporalDuration, DuperValue::Temporal(temporal)) => {
                DuperValue::try_duration_from(Cow::Owned(temporal.as_ref().to_string())).ok()
            }
            (
                DuperType::Integer,
                DuperValue::Float {
                    inner: float,
                    identifier,
                },
            ) => Some(DuperValue::Integer {
                identifier: identifier.clone(),
                inner: *float as i64,
            }),
            (
                DuperType::Float,
                DuperValue::Integer {
                    inner: integer,
                    identifier,
                },
            ) => Some(DuperValue::Float {
                identifier: identifier.clone(),
                inner: *integer as f64,
            }),
            (DuperType::Boolean, value) => Some(DuperValue::Boolean {
                identifier: value.identifier(),
                inner: IsTruthyFilter.filter(value),
            }),
            (DuperType::Null, _) => Some(DuperValue::Null {
                identifier: value.identifier(),
            }),

            // Unsupported casts
            _ => None,
        }
    }
}
