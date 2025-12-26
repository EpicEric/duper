use base64::{Engine, prelude::BASE64_STANDARD};
use duper::{
    DuperFloat, DuperIdentifier, DuperObject, DuperTemporal, DuperValue, visitor::DuperVisitor,
};

// A visitor that simplifies Duper values for Serde serializers.
pub(crate) struct SerdeJsonVisitor;

impl DuperVisitor for SerdeJsonVisitor {
    type Value = serde_json::Value;

    fn visit_object<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mut new_object = serde_json::Map::with_capacity(object.len());
        for (key, value) in object.iter() {
            new_object.insert(key.as_ref().to_owned(), value.accept(self));
        }
        serde_json::Value::Object(new_object)
    }

    fn visit_array<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        array: &[DuperValue<'a>],
    ) -> Self::Value {
        let mut new_array = Vec::with_capacity(array.len());
        for value in array.iter() {
            new_array.push(value.accept(self));
        }
        serde_json::Value::Array(new_array)
    }

    fn visit_tuple<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[DuperValue<'a>],
    ) -> Self::Value {
        let mut new_tuple = Vec::with_capacity(tuple.len());
        for value in tuple.iter() {
            new_tuple.push(value.accept(self));
        }
        serde_json::Value::Array(new_tuple)
    }

    fn visit_string<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        string: &'a str,
    ) -> Self::Value {
        serde_json::Value::String(string.to_owned())
    }

    fn visit_bytes<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value {
        serde_json::Value::String(BASE64_STANDARD.encode(bytes))
    }

    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value {
        serde_json::Value::String(temporal.as_ref().to_owned())
    }

    fn visit_integer<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        serde_json::Value::Number(
            serde_json::Number::from_i128(integer.into()).expect("valid range for JSON integer"),
        )
    }

    fn visit_float<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        float: DuperFloat,
    ) -> Self::Value {
        serde_json::Value::Number(
            serde_json::Number::from_f64(*float.as_ref()).expect("valid range for JSON float"),
        )
    }

    fn visit_boolean<'a>(
        &mut self,
        _identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        serde_json::Value::Bool(boolean)
    }

    fn visit_null<'a>(&mut self, _identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        serde_json::Value::Null
    }
}
