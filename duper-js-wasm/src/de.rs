use duper::visitor::DuperVisitor;
use js_sys::{Array, BigInt, Object, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::repr::{JsDuperValue, JsDuperValueInner};

#[derive(Clone)]
pub(crate) struct Visitor;

impl DuperVisitor for Visitor {
    type Value = Result<JsDuperValue, JsError>;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        object: &duper::DuperObject<'a>,
    ) -> Self::Value {
        let js_object = Object::new();

        for (key, value) in object.iter() {
            let value_result = value.accept(self)?;
            Reflect::set(
                &js_object,
                &JsValue::from(key.as_ref()),
                &value_result.into(),
            )
            .map_err(|e| {
                JsError::new(&format!("Failed to set property {}: {:?}", key.as_ref(), e))
            })?;
        }

        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Object(js_object.into()),
        })
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        array: &duper::DuperArray<'a>,
    ) -> Self::Value {
        let js_array = Array::new_with_length(array.len() as u32);

        for (index, value) in array.iter().enumerate() {
            let value_result = value.accept(self)?;
            js_array.set(index as u32, value_result.into());
        }

        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Array(js_array.into()),
        })
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        tuple: &duper::DuperTuple<'a>,
    ) -> Self::Value {
        let js_array = Array::new_with_length(tuple.len() as u32);

        for (index, value) in tuple.iter().enumerate() {
            let value_result = value.accept(self)?;
            js_array.set(index as u32, value_result.into());
        }

        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Tuple(js_array.into()),
        })
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        string: &duper::DuperString<'a>,
    ) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::String(string.as_ref().into()),
        })
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        bytes: &duper::DuperBytes<'a>,
    ) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Bytes(Uint8Array::from(bytes.as_ref()).into()),
        })
    }

    fn visit_temporal<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        temporal: &duper::DuperTemporal<'a>,
    ) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Temporal(temporal.as_ref().into()),
        })
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Integer(BigInt::from(integer).into()),
        })
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Float(JsValue::from_f64(float)),
        })
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&duper::DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Boolean(JsValue::from_bool(boolean)),
        })
    }

    fn visit_null<'a>(&mut self, identifier: Option<&duper::DuperIdentifier<'a>>) -> Self::Value {
        Ok(JsDuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: JsDuperValueInner::Null,
        })
    }
}
