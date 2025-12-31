use std::str::FromStr;

use base64::{Engine, prelude::BASE64_STANDARD};
use duper::{
    DuperFloat, DuperIdentifier, DuperObject, DuperTemporal, DuperValue, visitor::DuperVisitor,
};
use toml::{Value, value::Datetime};

use crate::temporal::{Instant, PlainDate, PlainDateTime, PlainTime};

// A visitor that serializes Duper into a TOML value.
pub(crate) struct TomlVisitor {}

impl DuperVisitor for TomlVisitor {
    type Value = Result<Option<Value>, String>;

    fn visit_object<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let table: Result<_, _> = object
            .iter()
            .filter_map(|(key, value)| {
                value
                    .accept(self)
                    .map(|value| value.map(|value| (key.as_ref().to_string(), value)))
                    .transpose()
            })
            .collect();
        Ok(Some(Value::Table(table?)))
    }

    fn visit_array<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value {
        let array: Result<_, _> = array
            .iter()
            .filter_map(|value| value.accept(self).transpose())
            .collect();
        Ok(Some(Value::Array(array?)))
    }

    fn visit_tuple<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value {
        let array: Result<_, _> = tuple
            .iter()
            .filter_map(|value| value.accept(self).transpose())
            .collect();
        Ok(Some(Value::Array(array?)))
    }

    fn visit_string<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        string: &'a str,
    ) -> Self::Value {
        Ok(Some(Value::String(string.to_string())))
    }

    fn visit_bytes<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value {
        Ok(Some(Value::String(BASE64_STANDARD.encode(bytes))))
    }

    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value {
        match temporal {
            DuperTemporal::Instant { inner } => {
                let datetime_str = Instant::from(super::clean_temporal(inner.as_ref())).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::PlainDateTime { inner } => {
                let datetime_str =
                    PlainDateTime::from(super::clean_temporal(inner.as_ref())).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::PlainDate { inner } => {
                let datetime_str =
                    PlainDate::from(super::clean_temporal(inner.as_ref())).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::PlainTime { inner } => {
                let datetime_str =
                    PlainTime::from(super::clean_temporal(inner.as_ref())).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::ZonedDateTime { inner } => {
                Ok(Some(Value::String(inner.as_ref().trim().to_string())))
            }
            DuperTemporal::PlainYearMonth { inner } => {
                Ok(Some(Value::String(inner.as_ref().trim().to_string())))
            }
            DuperTemporal::PlainMonthDay { inner } => {
                Ok(Some(Value::String(inner.as_ref().trim().to_string())))
            }
            DuperTemporal::Duration { inner } => {
                Ok(Some(Value::String(inner.as_ref().trim().to_string())))
            }
            DuperTemporal::Unspecified { inner, .. } => {
                Ok(Some(Value::String(inner.as_ref().trim().to_string())))
            }
        }
    }

    fn visit_integer<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        Ok(Some(Value::Integer(integer)))
    }

    fn visit_float<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        float: DuperFloat,
    ) -> Self::Value {
        Ok(Some(Value::Float(float.into_inner())))
    }

    fn visit_boolean<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        Ok(Some(Value::Boolean(boolean)))
    }

    fn visit_null<'a>(&mut self, _identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        Ok(None)
    }
}
