use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTuple, DuperValue,
};
use js_sys::{Array, BigInt, Boolean, Object, Reflect, Symbol, Uint8Array, try_iter};
use wasm_bindgen::prelude::*;

use crate::{SYMBOL_DUPER_IDENTIFIER, SYMBOL_DUPER_TYPE, SYMBOL_DUPER_VALUE};

pub(crate) fn serialize_jsvalue(value: &JsValue) -> Result<DuperValue<'static>, JsValue> {
    let identifier = if value.is_instance_of::<Object>() {
        let identifier = Reflect::get(value, &Symbol::for_(SYMBOL_DUPER_IDENTIFIER))?;
        let identifier = if identifier.is_truthy() {
            if identifier.is_string() {
                identifier
                    .as_string()
                    .map(DuperIdentifier::try_from)
                    .transpose()
                    .map_err(|e| JsValue::from_str(&e.to_string()))?
            } else {
                return Err(JsValue::from_str(&format!(
                    "invalid Duper identifier {identifier:?}"
                )));
            }
        } else {
            None
        };
        let typ = Reflect::get(value, &Symbol::for_(SYMBOL_DUPER_TYPE))?;
        if typ.is_truthy() {
            if typ.is_string() {
                match typ.as_string().expect("checked Duper type").as_str() {
                    "object" => return serialize_object(value, identifier),
                    "array" => return serialize_array(value, identifier),
                    "tuple" => return serialize_tuple(value, identifier),
                    "string" => {
                        return serialize_string(
                            &Reflect::get(value, &Symbol::for_(SYMBOL_DUPER_VALUE))?,
                            identifier,
                        );
                    }
                    "bytes" => (),
                    "integer" => {
                        return serialize_integer(
                            &Reflect::get(value, &Symbol::for_(SYMBOL_DUPER_VALUE))?,
                            identifier,
                        );
                    }
                    "float" => {
                        return serialize_float(
                            &Reflect::get(value, &Symbol::for_(SYMBOL_DUPER_VALUE))?,
                            identifier,
                        );
                    }
                    "boolean" => {
                        return serialize_boolean(
                            &Reflect::get(value, &Symbol::for_(SYMBOL_DUPER_VALUE))?,
                            identifier,
                        );
                    }
                    "null" => return serialize_null(identifier),
                    _ => return Err(JsValue::from_str(&format!("invalid Duper type {typ:?}"))),
                }
            } else {
                return Err(JsValue::from_str(&format!(
                    "
                    invalid Duper type {typ:?}"
                )));
            }
        }
        identifier
    } else {
        None
    };

    // Not a Duper deserialized value; detect which type it actually is
    if value.is_null() || value.is_undefined() {
        serialize_null(identifier)
    } else if Boolean::is_type_of(value) {
        serialize_boolean(value, identifier)
    } else if value.is_bigint() {
        serialize_integer(value, identifier)
    } else if value.as_f64().is_some() {
        serialize_float(value, identifier)
    } else if Uint8Array::is_type_of(value) {
        serialize_bytes(value, identifier)
    } else if value.as_string().is_some() {
        serialize_string(value, identifier)
    } else if Array::is_array(value) {
        serialize_array(value, identifier)
    } else if !value.is_function() && !value.is_symbol() {
        serialize_object(value, identifier)
    } else {
        Err(JsValue::from_str(&format!("unknown value {value:?}")))
    }
}

fn serialize_object(
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

fn serialize_array(
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

fn serialize_tuple(
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

fn serialize_string(
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

fn serialize_bytes(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    let bytes: &Uint8Array =
        JsCast::dyn_ref(value).ok_or_else(|| format!("expected bytes, found {value:?}"))?;
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Bytes(DuperBytes::from(Cow::Owned(bytes.to_vec()))),
    })
}

fn serialize_integer(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    if let Some(bigint) = JsCast::dyn_ref::<BigInt>(value) {
        match i64::try_from(bigint.clone()) {
            Ok(integer) => Ok(DuperValue {
                identifier,
                inner: DuperInner::Integer(integer),
            }),
            Err(_) => Ok(DuperValue {
                identifier: Some(identifier.unwrap_or_else(|| {
                    DuperIdentifier::try_from(Cow::Borrowed("Integer")).expect("valid identifier")
                })),
                inner: DuperInner::String(DuperString::from(Cow::Owned(
                    value
                        .as_string()
                        .ok_or_else(|| format!("expected integer, found {value:?}"))?,
                ))),
            }),
        }
    } else if let Some(float) = value.as_f64() {
        if float.fract() == 0.0 {
            Ok(DuperValue {
                identifier,
                inner: DuperInner::Integer(float as i64),
            })
        } else {
            Ok(DuperValue {
                identifier,
                inner: DuperInner::Float(float),
            })
        }
    } else {
        Err(JsValue::from_str(&format!(
            "expected integer, found {value:?}"
        )))
    }
}

fn serialize_float(
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

fn serialize_boolean(
    value: &JsValue,
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Boolean(value.is_truthy()),
    })
}

fn serialize_null(
    identifier: Option<DuperIdentifier<'static>>,
) -> Result<DuperValue<'static>, JsValue> {
    Ok(DuperValue {
        identifier,
        inner: DuperInner::Null,
    })
}
