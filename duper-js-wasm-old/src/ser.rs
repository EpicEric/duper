use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTemporal, DuperTuple, DuperValue,
};
use js_sys::{Array, BigInt, Boolean, Date, Object, Uint8Array, try_iter};
use wasm_bindgen::prelude::*;

use crate::repr::JsDuperValue;

pub(crate) fn serialize_jsvalue(value: &JsValue) -> Result<DuperValue<'static>, JsValue> {
    if let Ok(value) = JsDuperValue::from_jsval(value) {
        value.serialize()
    }
    // Not a Duper deserialized value; detect which type it might be
    else if value.is_null() || value.is_undefined() {
        serialize_null(None)
    } else if value.has_type::<Boolean>() {
        serialize_boolean(value, None)
    } else if value.is_bigint() {
        serialize_integer(value, None)
    } else if value.as_f64().is_some() {
        serialize_float(value, None)
    } else if value.has_type::<Uint8Array>() {
        serialize_bytes(value, None)
    } else if value.is_string() {
        serialize_string(value, None)
    } else if value.has_type::<crate::temporal::Instant>()
        || value.has_type::<crate::temporal::ZonedDateTime>()
        || value.has_type::<crate::temporal::PlainDate>()
        || value.has_type::<crate::temporal::PlainTime>()
        || value.has_type::<crate::temporal::PlainDateTime>()
        || value.has_type::<crate::temporal::PlainYearMonth>()
        || value.has_type::<crate::temporal::PlainMonthDay>()
        || value.has_type::<crate::temporal::Duration>()
    {
        serialize_temporal(value, None)
    } else if value.has_type::<Date>() {
        Err(JsValue::from_str(
            "invalid Date value; convert it into a Temporal value first",
        ))
    } else if Array::is_array(value) {
        serialize_array(value, None)
    } else if !value.is_function() && !value.is_symbol() {
        serialize_object(value, None)
    } else {
        Err(JsValue::from_str(&format!("unknown value {value:?}")))
    }
}

pub(crate) fn serialize_object(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let entries = Object::entries(
        value
            .dyn_ref()
            .ok_or_else(|| format!("expected object, found {value:?}"))?,
    );
    let values: Result<Vec<_>, JsValue> = entries
        .into_iter()
        .map(|tup| {
            let tup: Array = tup.dyn_into().expect("expected key-value tuple");
            let key = tup
                .get(0)
                .as_string()
                .ok_or_else(|| format!("expected key string, found {tup:?}"))?;
            let value = serialize_jsvalue(&tup.get(1))?;
            Ok((DuperKey::from(key), value))
        })
        .collect();
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Object(DuperObject::try_from(values?).map_err(|err| err.to_string())?),
    })
}

pub(crate) fn serialize_array(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let iterator = try_iter(value)?.ok_or("cannot iterate over array")?;
    let elements: Result<Vec<_>, JsValue> =
        iterator.map(|value| serialize_jsvalue(&value?)).collect();
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Array(DuperArray::from(elements?)),
    })
}

pub(crate) fn serialize_tuple(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let iterator = try_iter(value)?.ok_or("cannot iterate over tuple")?;
    let elements: Result<Vec<_>, JsValue> =
        iterator.map(|value| serialize_jsvalue(&value?)).collect();
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Tuple(DuperTuple::from(elements?)),
    })
}

pub(crate) fn serialize_string(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let string = value
        .as_string()
        .ok_or_else(|| format!("expected string, found {value:?}"))?;
    Ok(DuperValue {
        identifier,
        inner: DuperInner::String(DuperString::from(Cow::Owned(string))),
    })
}

pub(crate) fn serialize_bytes(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let bytes = JsCast::dyn_ref::<Uint8Array>(value)
        .ok_or_else(|| format!("expected bytes, found {value:?}"))?;
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(bytes.to_vec()))),
    })
}

pub(crate) fn serialize_temporal(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    if let Some(temporal) = value.dyn_ref::<crate::temporal::Instant>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("Instant").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_instant_from(Cow::Owned(temporal.to_string()))
                    .map_err(|err| JsValue::from_str(&format!("failed to parse Instant: {err}")))?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::ZonedDateTime>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("ZonedDateTime").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_zoned_date_time_from(Cow::Owned(temporal.to_string())).map_err(
                    |err| JsValue::from_str(&format!("failed to parse ZonedDateTime: {err}")),
                )?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::PlainDate>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("PlainDate").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_plain_date_from(Cow::Owned(temporal.to_string())).map_err(
                    |err| JsValue::from_str(&format!("failed to parse PlainDate: {err}")),
                )?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::PlainTime>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("PlainTime").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_plain_time_from(Cow::Owned(temporal.to_string())).map_err(
                    |err| JsValue::from_str(&format!("failed to parse PlainTime: {err}")),
                )?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::PlainDateTime>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("PlainDateTime").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_plain_date_time_from(Cow::Owned(temporal.to_string())).map_err(
                    |err| JsValue::from_str(&format!("failed to parse PlainDateTime: {err}")),
                )?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::PlainYearMonth>() {
        return Ok(DuperValue {
            identifier: Some(
                DuperIdentifier::try_from("PlainYearMonth").expect("valid identifier"),
            ),
            inner: DuperInner::Temporal(
                DuperTemporal::try_plain_year_month_from(Cow::Owned(temporal.to_string()))
                    .map_err(|err| {
                        JsValue::from_str(&format!("failed to parse PlainYearMonth: {err}"))
                    })?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::PlainMonthDay>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("PlainMonthDay").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_plain_month_day_from(Cow::Owned(temporal.to_string())).map_err(
                    |err| JsValue::from_str(&format!("failed to parse PlainMonthDay: {err}")),
                )?,
            ),
        });
    } else if let Some(temporal) = value.dyn_ref::<crate::temporal::Duration>() {
        return Ok(DuperValue {
            identifier: Some(DuperIdentifier::try_from("Duration").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_duration_from(Cow::Owned(temporal.to_string())).map_err(
                    |err| JsValue::from_str(&format!("failed to parse Duration: {err}")),
                )?,
            ),
        });
    }
    let string = value
        .as_string()
        .ok_or_else(|| format!("expected string, found {value:?}"))?;
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Temporal(
            DuperTemporal::try_unspecified_from(Cow::Owned(string)).map_err(|err| {
                JsValue::from_str(&format!("failed to parse Temporal value: {err}"))
            })?,
        ),
    })
}

pub(crate) fn serialize_integer(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let bigint = JsCast::dyn_ref::<BigInt>(value)
        .ok_or_else(|| format!("expected BigInt, found {value:?}"))?;
    match i64::try_from(bigint.clone()) {
        Ok(integer) => Ok(DuperValue {
            identifier,
            inner: DuperInner::Integer(integer),
        }),
        Err(_) => Ok(DuperValue {
            identifier: Some(identifier.unwrap_or_else(|| {
                DuperIdentifier::try_from(Cow::Borrowed("I64")).expect("valid identifier")
            })),
            inner: DuperInner::String(DuperString::from(Cow::Owned(
                bigint
                    .to_string(10)?
                    .as_string()
                    .ok_or("failed to convert BigInt to string")?,
            ))),
        }),
    }
}

pub(crate) fn serialize_float(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Float(
            value
                .as_f64()
                .ok_or_else(|| format!("expected float, found {value:?}"))?,
        ),
    })
}

pub(crate) fn serialize_boolean(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Boolean(value.is_truthy()),
    })
}

pub(crate) fn serialize_null(
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Null,
    })
}
