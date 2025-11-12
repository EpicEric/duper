use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTemporal, DuperTuple, DuperValue,
};

use crate::{DuperError, DuperValue as Value};

impl Value {
    pub(crate) fn serialize(self) -> Result<DuperValue<'static>, DuperError> {
        match self {
            Value::Object { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Object(DuperObject::try_from(
                    value
                        .into_iter()
                        .map(|(key, val)| val.serialize().map(|val| (DuperKey::from(key), val)))
                        .collect::<Result<Vec<_>, _>>()?,
                )?),
            }),
            Value::Array { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Array(DuperArray::from(
                    value
                        .into_iter()
                        .map(|val| val.serialize())
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            }),
            Value::Tuple { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Tuple(DuperTuple::from(
                    value
                        .into_iter()
                        .map(|val| val.serialize())
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            }),
            Value::String { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::String(DuperString::from(value)),
            }),
            Value::Bytes { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Bytes(DuperBytes::from(value)),
            }),
            Value::Temporal { identifier, value } => Ok(DuperValue {
                inner: DuperInner::Temporal(match identifier.as_ref().map(AsRef::as_ref) {
                    Some("Instant") => DuperTemporal::try_instant_from(Cow::Owned(value))?,
                    Some("ZonedDateTime") => {
                        DuperTemporal::try_zoned_date_time_from(Cow::Owned(value))?
                    }
                    Some("PlainDate") => DuperTemporal::try_plain_date_from(Cow::Owned(value))?,
                    Some("PlainTime") => DuperTemporal::try_plain_time_from(Cow::Owned(value))?,
                    Some("PlainDateTime") => {
                        DuperTemporal::try_plain_date_time_from(Cow::Owned(value))?
                    }
                    Some("PlainYearMonth") => {
                        DuperTemporal::try_plain_year_month_from(Cow::Owned(value))?
                    }
                    Some("PlainMonthDay") => {
                        DuperTemporal::try_plain_month_day_from(Cow::Owned(value))?
                    }
                    Some("Duration") => DuperTemporal::try_duration_from(Cow::Owned(value))?,
                    Some(_) | None => DuperTemporal::try_unspecified_from(Cow::Owned(value))?,
                }),
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
            }),
            Value::Integer { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Integer(value),
            }),
            Value::Float { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Float(value),
            }),
            Value::Boolean { identifier, value } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Boolean(value),
            }),
            Value::Null { identifier } => Ok(DuperValue {
                identifier: identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Null,
            }),
        }
    }
}
