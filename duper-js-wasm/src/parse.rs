use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperString, DuperTemporal, DuperTuple,
    visitor::DuperVisitor,
};
use js_sys::{BigInt, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::{DuperObjectEntry, DuperValue, DuperValueType};

pub(crate) struct WasmVisitor;

impl DuperVisitor for WasmVisitor {
    type Value = DuperValue;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mut value = Vec::with_capacity(object.len());
        for (key, val) in object.iter() {
            value.push(DuperObjectEntry {
                key: key.as_ref().to_string(),
                value: val.accept(self).into(),
            });
        }
        DuperValue {
            typ: DuperValueType::Object,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: value.into(),
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value {
        let mut value = Vec::with_capacity(array.len());
        for val in array.iter() {
            value.push(val.accept(self));
        }
        DuperValue {
            typ: DuperValueType::Array,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: value.into(),
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value {
        let mut value = Vec::with_capacity(tuple.len());
        for val in tuple.iter() {
            value.push(val.accept(self));
        }
        DuperValue {
            typ: DuperValueType::Tuple,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: value.into(),
        }
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &DuperString<'a>,
    ) -> Self::Value {
        DuperValue {
            typ: DuperValueType::String,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: string.as_ref().into(),
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        DuperValue {
            typ: DuperValueType::Bytes,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: Uint8Array::new_from_slice(bytes.as_ref()).into(),
        }
    }

    fn visit_temporal<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        temporal: &DuperTemporal<'a>,
    ) -> Self::Value {
        DuperValue {
            typ: DuperValueType::Temporal,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: temporal.as_ref().into(),
        }
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        DuperValue {
            typ: DuperValueType::Integer,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: BigInt::from(integer).into(),
        }
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        DuperValue {
            typ: DuperValueType::Float,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: float.into(),
        }
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        DuperValue {
            typ: DuperValueType::Boolean,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: boolean.into(),
        }
    }

    fn visit_null<'a>(&mut self, identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        DuperValue {
            typ: DuperValueType::Null,
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: JsValue::NULL,
        }
    }
}
