use duper::{DuperIdentifier, DuperObject, DuperTemporal, visitor::DuperVisitor};

use crate::{DuperObjectEntry, DuperValue};

pub(crate) struct UniffiVisitor;

impl DuperVisitor for UniffiVisitor {
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
                value: val.accept(self),
            });
        }
        DuperValue::Object {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value,
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &[duper::DuperValue<'a>],
    ) -> Self::Value {
        let mut value = Vec::with_capacity(array.len());
        for val in array.iter() {
            value.push(val.accept(self));
        }
        DuperValue::Array {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value,
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &[duper::DuperValue<'a>],
    ) -> Self::Value {
        let mut value = Vec::with_capacity(tuple.len());
        for val in tuple.iter() {
            value.push(val.accept(self));
        }
        DuperValue::Tuple {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value,
        }
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &'a str,
    ) -> Self::Value {
        DuperValue::String {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: string.to_string(),
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &'a [u8],
    ) -> Self::Value {
        DuperValue::Bytes {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: bytes.as_ref().to_vec(),
        }
    }

    fn visit_temporal<'a>(&mut self, temporal: &DuperTemporal<'a>) -> Self::Value {
        DuperValue::Temporal {
            identifier: temporal
                .identifier()
                .map(|identifier| identifier.as_ref().to_string()),
            value: temporal.as_ref().to_string(),
        }
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        DuperValue::Integer {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: integer,
        }
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        DuperValue::Float {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: float,
        }
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        DuperValue::Boolean {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: boolean,
        }
    }

    fn visit_null<'a>(&mut self, identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        DuperValue::Null {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
        }
    }
}
