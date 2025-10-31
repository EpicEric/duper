use std::fmt::Display;

use duper::{DuperIdentifier, DuperValue};
use js_sys::{Array, BigInt, Boolean, Function, Object, Reflect, Uint8Array};
use wasm_bindgen::{convert::TryFromJsValue, prelude::*};

use crate::ser::{
    serialize_array, serialize_boolean, serialize_bytes, serialize_float, serialize_integer,
    serialize_null, serialize_object, serialize_string, serialize_tuple,
};

#[wasm_bindgen(typescript_custom_section)]
const DUPER_VALUE_TYPE: &'static str = r#"
/**
 * Valid Duper types.
 */
type DuperValueType = "object" | "array" | "tuple" | "string" | "bytes" | "integer" | "float" | "boolean" | "null";"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "DuperValueType")]
    pub type DuperValueType;
}

pub(crate) enum JsDuperValueInner {
    Object(JsValue),
    Array(JsValue),
    Tuple(JsValue),
    String(JsValue),
    Bytes(JsValue),
    Integer(JsValue),
    Float(JsValue),
    Boolean(JsValue),
    Null,
}

impl JsDuperValueInner {
    fn new(typ: Option<String>, inner: JsValue) -> Result<JsDuperValueInner, JsError> {
        if let Some(typ) = typ {
            match typ.as_str() {
                "object" => {
                    let entries = Object::entries(&inner.dyn_into().map_err(|inner| {
                        JsError::new(&format!("expected Object, found {inner:?}"))
                    })?);
                    let new_object = Object::new();
                    for tup in entries.into_iter() {
                        let tup: Array = tup.dyn_into().expect("expected key-value tuple");
                        let key = tup.get(0);
                        if !key.is_string() {
                            return Err(JsError::new(&format!(
                                "invalid key for Object: expected string, found {key:?}"
                            )));
                        }
                        let value = tup.get(1);
                        match JsDuperValue::try_from_js_value(value) {
                            Ok(value) => {
                                Reflect::set(&new_object, &key, &value.into()).map_err(|err| {
                                    JsError::new(&format!(
                                        "failed to set key {key:?} of Object: {err:?}"
                                    ))
                                })?
                            }
                            Err(value) => {
                                return Err(JsError::new(&format!(
                                    "invalid value with key {key:?} for Object: expected DuperValue, found {value:?}"
                                )));
                            }
                        };
                    }
                    Ok(JsDuperValueInner::Object(new_object.into()))
                }
                "array" => {
                    let array = inner.dyn_into::<Array>().map_err(|inner| {
                        JsError::new(&format!("expected Array, found {inner:?}"))
                    })?;
                    let new_array = Array::new_with_length(array.length());
                    for (index, element) in array.into_iter().enumerate() {
                        match JsDuperValue::try_from_js_value(element) {
                            Ok(value) => new_array.set(index as u32, value.into()),
                            Err(element) => {
                                return Err(JsError::new(&format!(
                                    "invalid element #{index} for Array: expected DuperValue, found {element:?}"
                                )));
                            }
                        }
                    }
                    Ok(JsDuperValueInner::Array(new_array.into()))
                }
                "tuple" => {
                    let array = inner.dyn_into::<Array>().map_err(|inner| {
                        JsError::new(&format!("expected Array, found {inner:?}"))
                    })?;
                    let new_array = Array::new_with_length(array.length());
                    for (index, element) in array.into_iter().enumerate() {
                        match JsDuperValue::try_from_js_value(element) {
                            Ok(value) => new_array.set(index as u32, value.into()),
                            Err(element) => {
                                return Err(JsError::new(&format!(
                                    "invalid element #{index} for Array: expected DuperValue, found {element:?}"
                                )));
                            }
                        }
                    }
                    Ok(JsDuperValueInner::Tuple(new_array.into()))
                }
                "string" => {
                    if inner.is_string() {
                        Ok(JsDuperValueInner::String(inner))
                    } else {
                        Err(JsError::new(&format!("expected string, found {inner:?}")))
                    }
                }
                "bytes" => {
                    if Uint8Array::is_type_of(&inner) {
                        Ok(JsDuperValueInner::Bytes(inner))
                    } else if Array::is_array(&inner) {
                        let array: Array = inner.dyn_into().unwrap();
                        let bytes = Uint8Array::new_with_length(array.length());
                        for (index, element) in array.into_iter().enumerate() {
                            bytes.set(&element, index as u32);
                        }
                        Ok(JsDuperValueInner::Bytes(bytes.into()))
                    } else {
                        Err(JsError::new(&format!(
                            "expected Uint8Array or Array, found {inner:?}"
                        )))
                    }
                }
                "integer" => {
                    if inner.is_bigint() {
                        Ok(JsDuperValueInner::Integer(inner))
                    } else if inner.as_f64().is_some() {
                        Ok(JsDuperValueInner::Integer(
                            BigInt::new(&inner)
                                .map_err(|err| JsError::new(&err.to_string().as_string().unwrap()))?
                                .into(),
                        ))
                    } else {
                        Err(JsError::new(&format!(
                            "expected number or BigInt, found {inner:?}"
                        )))
                    }
                }
                "float" => {
                    if inner.as_f64().is_some() {
                        Ok(JsDuperValueInner::Integer(inner))
                    } else {
                        Err(JsError::new(&format!("expected number, found {inner:?}")))
                    }
                }
                "boolean" => {
                    if Boolean::is_type_of(&inner) {
                        Ok(JsDuperValueInner::Boolean(inner))
                    } else {
                        Err(JsError::new(&format!("expected boolean, found {inner:?}")))
                    }
                }
                "null" => {
                    if inner.is_null() || inner.is_undefined() {
                        Ok(JsDuperValueInner::Null)
                    } else {
                        Err(JsError::new(&format!(
                            "expected null or undefined, found {inner:?}"
                        )))
                    }
                }
                _ => Err(JsError::new(&format!("unknown type {typ}"))),
            }
        } else if inner.is_null() || inner.is_undefined() {
            Ok(JsDuperValueInner::Null)
        } else if Boolean::is_type_of(&inner) {
            Ok(JsDuperValueInner::Boolean(inner))
        } else if inner.is_bigint() {
            Ok(JsDuperValueInner::Integer(inner))
        } else if inner.as_f64().is_some() {
            Ok(JsDuperValueInner::Float(inner))
        } else if Uint8Array::is_type_of(&inner) {
            Ok(JsDuperValueInner::Bytes(inner))
        } else if inner.as_string().is_some() {
            Ok(JsDuperValueInner::String(inner))
        } else if Array::is_array(&inner) {
            Ok(JsDuperValueInner::Array(inner))
        } else if Object::instanceof(&inner) && !inner.is_function() && !inner.is_symbol() {
            Ok(JsDuperValueInner::Object(inner))
        } else {
            Err(JsError::new(&format!("unknown inner {inner:?}")))
        }
    }
}

impl Display for JsDuperValueInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            JsDuperValueInner::Object(_) => "object",
            JsDuperValueInner::Array(_) => "array",
            JsDuperValueInner::Tuple(_) => "tuple",
            JsDuperValueInner::String(_) => "string",
            JsDuperValueInner::Bytes(_) => "bytes",
            JsDuperValueInner::Integer(_) => "integer",
            JsDuperValueInner::Float(_) => "float",
            JsDuperValueInner::Boolean(_) => "boolean",
            JsDuperValueInner::Null => "null",
        })
    }
}

#[wasm_bindgen(js_name = DuperValue)]
pub struct JsDuperValue {
    pub(crate) identifier: Option<DuperIdentifier<'static>>,
    pub(crate) inner: JsDuperValueInner,
}

impl JsDuperValue {
    pub(crate) fn serialize(&self) -> Result<DuperValue<'static>, JsValue> {
        let identifier = self.identifier.clone();
        match &self.inner {
            JsDuperValueInner::Object(inner) => serialize_object(inner, identifier),
            JsDuperValueInner::Array(inner) => serialize_array(inner, identifier),
            JsDuperValueInner::Tuple(inner) => serialize_tuple(inner, identifier),
            JsDuperValueInner::String(inner) => serialize_string(inner, identifier),
            JsDuperValueInner::Bytes(inner) => serialize_bytes(inner, identifier),
            JsDuperValueInner::Integer(inner) => serialize_integer(inner, identifier),
            JsDuperValueInner::Float(inner) => serialize_float(inner, identifier),
            JsDuperValueInner::Boolean(inner) => serialize_boolean(inner, identifier),
            JsDuperValueInner::Null => serialize_null(identifier),
        }
    }
}

#[wasm_bindgen(js_class = DuperValue)]
impl JsDuperValue {
    /// Creates a new Duper value.
    #[wasm_bindgen(constructor)]
    pub fn new(
        inner: JsValue,
        identifier: Option<String>,
        r#type: Option<DuperValueType>,
    ) -> Result<Self, JsError> {
        let typ = r#type.and_then(|typ| JsValue::from(typ).as_string());
        let inner = JsDuperValueInner::new(typ, inner)?;
        Ok(Self {
            identifier: identifier.map(DuperIdentifier::try_from).transpose()?,
            inner,
        })
    }

    /// Returns the identifier of this Duper value.
    #[wasm_bindgen(getter)]
    pub fn identifier(&self) -> Option<String> {
        self.identifier
            .as_ref()
            .map(|identifier| identifier.as_ref().to_owned())
    }

    /// Sets the identifier of this Duper value.
    #[wasm_bindgen(setter)]
    pub fn set_identifier(&mut self, identifier: Option<String>) -> Result<(), JsError> {
        self.identifier = identifier.map(DuperIdentifier::try_from).transpose()?;
        Ok(())
    }

    /// Returns the contents of this Duper value.
    #[wasm_bindgen(getter)]
    pub fn inner(&self) -> JsValue {
        match &self.inner {
            JsDuperValueInner::Object(inner)
            | JsDuperValueInner::Array(inner)
            | JsDuperValueInner::Tuple(inner)
            | JsDuperValueInner::String(inner)
            | JsDuperValueInner::Bytes(inner)
            | JsDuperValueInner::Integer(inner)
            | JsDuperValueInner::Float(inner)
            | JsDuperValueInner::Boolean(inner) => inner.clone(),
            JsDuperValueInner::Null => JsValue::NULL,
        }
    }

    /// Returns the type of this Duper value.
    #[wasm_bindgen(getter)]
    pub fn r#type(&self) -> DuperValueType {
        JsValue::from(self.inner.to_string()).into()
    }

    /// Sets the contents for this Duper value, optionally specifying the type.
    #[wasm_bindgen(js_name = setValue)]
    pub fn set_value(
        &mut self,
        inner: JsValue,
        r#type: Option<DuperValueType>,
    ) -> Result<(), JsError> {
        let typ = r#type.and_then(|typ| JsValue::from(typ).as_string());
        self.inner = JsDuperValueInner::new(typ, inner)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        match &self.inner {
            JsDuperValueInner::Object(inner) => {
                let entries =
                    Object::entries(inner.dyn_ref().ok_or_else(|| {
                        JsError::new(&format!("expected Object, found {inner:?}"))
                    })?);
                let new_object = Object::new();
                let to_json = JsValue::from_str("toJSON");

                for (index, tup) in entries.into_iter().enumerate() {
                    let tup: Array = tup.dyn_into().expect("expected key-value tuple");
                    let key = tup.get(0);
                    if !key.is_string() {
                        return Err(JsValue::from_str(&format!(
                            "invalid key #{index} for Object: expected string, found {key:?}"
                        )));
                    }
                    let value = tup.get(1);
                    Reflect::set(
                        &new_object,
                        &key,
                        &Reflect::get(&value, &to_json)?
                            .dyn_into::<Function>()?
                            .call0(&value)?,
                    )?;
                }

                Ok(new_object.into())
            }
            JsDuperValueInner::Array(inner) | JsDuperValueInner::Tuple(inner) => {
                let array = inner
                    .dyn_ref::<Array>()
                    .ok_or_else(|| JsError::new(&format!("expected Object, found {inner:?}")))?;
                let new_array = Array::new_with_length(array.length());
                let to_json = JsValue::from_str("toJSON");

                for (index, element) in array.iter().enumerate() {
                    new_array.set(
                        index as u32,
                        Reflect::get(&element, &to_json)?
                            .dyn_into::<Function>()?
                            .call0(&element)?,
                    );
                }

                Ok(new_array.into())
            }
            JsDuperValueInner::String(inner)
            | JsDuperValueInner::Float(inner)
            | JsDuperValueInner::Boolean(inner) => Ok(inner.clone()),
            JsDuperValueInner::Null => Ok(JsValue::NULL),
            JsDuperValueInner::Bytes(inner) => {
                let bytes = JsCast::dyn_ref::<Uint8Array>(inner)
                    .ok_or_else(|| JsValue::from_str("inner value was not an Uint8Array"))?;
                let js_array = Array::new_with_length(bytes.length());
                for (index, value) in bytes.to_vec().into_iter().enumerate() {
                    js_array.set(index as u32, value.into());
                }
                Ok(js_array.into())
            }
            JsDuperValueInner::Integer(inner) => {
                let bigint = JsCast::dyn_ref::<BigInt>(inner)
                    .ok_or_else(|| JsValue::from_str("inner value was not a BigInt"))?;
                match i64::try_from(bigint.clone()) {
                    Ok(integer) => Ok((integer as f64).into()),
                    Err(_) => Ok(bigint.to_string(10)?.into()),
                }
            }
        }
    }
}
