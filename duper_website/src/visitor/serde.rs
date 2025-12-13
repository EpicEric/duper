use std::borrow::Cow;

use base64::{Engine, prelude::BASE64_STANDARD};
use duper::{
    DuperFloat, DuperIdentifier, DuperKey, DuperObject, DuperTemporal, DuperValue,
    visitor::DuperVisitor,
};

// A visitor that simplifies Duper values for Serde serializers.
pub(crate) struct SerdeVisitor;

impl DuperVisitor for SerdeVisitor {
    type Value = DuperValue<'static>;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mut new_object = Vec::with_capacity(object.len());
        for (key, value) in object.iter() {
            new_object.push((DuperKey::from(key.as_ref().to_owned()), value.accept(self)));
        }
        DuperValue::Object {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperObject::try_from(new_object).expect("object keys are unchanged"),
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value {
        let mut new_array = Vec::with_capacity(array.len());
        for value in array.iter() {
            new_array.push(value.accept(self));
        }
        DuperValue::Array {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: new_array,
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value {
        let mut new_tuple = Vec::with_capacity(tuple.len());
        for value in tuple.iter() {
            new_tuple.push(value.accept(self));
        }
        DuperValue::Tuple {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: new_tuple,
        }
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &'a str,
    ) -> Self::Value {
        DuperValue::String {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: Cow::Owned(string.to_string()),
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value {
        DuperValue::String {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: Cow::Owned(BASE64_STANDARD.encode(bytes)),
        }
    }

    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value {
        DuperValue::String {
            identifier: temporal
                .identifier()
                .map(|identifier| identifier.static_clone()),
            inner: Cow::Owned(temporal.as_ref().to_string()),
        }
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        DuperValue::Integer {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: integer,
        }
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        float: DuperFloat,
    ) -> Self::Value {
        DuperValue::Float {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: float,
        }
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        DuperValue::Boolean {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: boolean,
        }
    }

    fn visit_null<'a>(&mut self, identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        DuperValue::Null {
            identifier: identifier.map(|identifier| identifier.static_clone()),
        }
    }
}
