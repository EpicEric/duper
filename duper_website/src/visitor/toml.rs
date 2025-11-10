use std::str::FromStr;

use base64::{Engine, prelude::BASE64_STANDARD};
use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTemporal, DuperTuple,
    visitor::DuperVisitor,
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
        array: &DuperArray<'a>,
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
        tuple: &DuperTuple<'a>,
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
        string: &DuperString<'a>,
    ) -> Self::Value {
        Ok(Some(Value::String(string.as_ref().to_string())))
    }

    fn visit_bytes<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        Ok(Some(Value::String(BASE64_STANDARD.encode(bytes.as_ref()))))
    }

    fn visit_temporal<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        temporal: &DuperTemporal<'a>,
    ) -> Self::Value {
        match temporal {
            DuperTemporal::Instant(inner) => {
                let datetime_str = Instant::from(inner.as_ref()).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::PlainDateTime(inner) => {
                let datetime_str = PlainDateTime::from(inner.as_ref()).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::PlainDate(inner) => {
                let datetime_str = PlainDate::from(inner.as_ref()).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::PlainTime(inner) => {
                let datetime_str = PlainTime::from(inner.as_ref()).to_string();
                Ok(Some(Value::Datetime(
                    Datetime::from_str(&datetime_str).map_err(|err| err.to_string())?,
                )))
            }
            DuperTemporal::ZonedDateTime(inner)
            | DuperTemporal::PlainYearMonth(inner)
            | DuperTemporal::PlainMonthDay(inner)
            | DuperTemporal::Duration(inner)
            | DuperTemporal::Unspecified(inner) => {
                Ok(Some(Value::String(inner.as_ref().to_string())))
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
        float: f64,
    ) -> Self::Value {
        Ok(Some(Value::Float(float)))
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
