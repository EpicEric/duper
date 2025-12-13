use std::borrow::Cow;

use duper::{DuperFloat, DuperIdentifier, DuperKey, DuperObject, DuperTemporal, DuperValue};

use crate::{DuperError, DuperObjectEntry, DuperValue as Value};

impl Value {
    pub(crate) fn serialize(self) -> Result<DuperValue<'static>, DuperError> {
        match self {
            Value::Object { identifier, value } => Ok(DuperValue::Object {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: DuperObject::try_from(
                    value
                        .into_iter()
                        .map(|DuperObjectEntry { key, value }| {
                            value.serialize().map(|val| (DuperKey::from(key), val))
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )?,
            }),
            Value::Array { identifier, value } => Ok(DuperValue::Array {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: value
                    .into_iter()
                    .map(|val| val.serialize())
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            Value::Tuple { identifier, value } => Ok(DuperValue::Tuple {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: value
                    .into_iter()
                    .map(|val| val.serialize())
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            Value::String { identifier, value } => Ok(DuperValue::String {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: Cow::Owned(value),
            }),
            Value::Bytes { identifier, value } => Ok(DuperValue::Bytes {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: Cow::Owned(value),
            }),
            Value::Temporal { identifier, value } => Ok(DuperValue::Temporal(
                match identifier.as_ref().map(AsRef::as_ref) {
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
                    identifier => DuperTemporal::try_unspecified_from(
                        identifier
                            .map(|identifier| {
                                DuperIdentifier::try_from(Cow::Owned(identifier.to_string()))
                            })
                            .transpose()?,
                        Cow::Owned(value),
                    )?,
                },
            )),
            Value::Integer { identifier, value } => Ok(DuperValue::Integer {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: value,
            }),
            Value::Float { identifier, value } => Ok(DuperValue::Float {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: DuperFloat::try_new(value)?,
            }),
            Value::Boolean { identifier, value } => Ok(DuperValue::Boolean {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
                inner: value,
            }),
            Value::Null { identifier } => Ok(DuperValue::Null {
                identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
            }),
        }
    }
}
