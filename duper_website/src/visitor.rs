use base64::{Engine, prelude::BASE64_STANDARD};
use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTemporal, DuperTuple, DuperValue, visitor::DuperVisitor,
};

// A visitor that transforms bytes into an array of integers.
pub(crate) struct EncodeBytesVisitor;

impl DuperVisitor for EncodeBytesVisitor {
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
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Object(
                DuperObject::try_from(new_object).expect("object keys are unchanged"),
            ),
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value {
        let mut new_array = Vec::with_capacity(array.len());
        for value in array.iter() {
            new_array.push(value.accept(self));
        }
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Array(DuperArray::from(new_array)),
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value {
        let mut new_tuple = Vec::with_capacity(tuple.len());
        for value in tuple.iter() {
            new_tuple.push(value.accept(self));
        }
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Tuple(DuperTuple::from(new_tuple)),
        }
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &DuperString<'a>,
    ) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::String(DuperString::from(string.as_ref().to_owned())),
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::String(DuperString::from(BASE64_STANDARD.encode(bytes.as_ref()))),
        }
    }

    fn visit_temporal<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        temporal: &DuperTemporal<'a>,
    ) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Temporal(temporal.static_clone()),
        }
    }

    fn visit_integer<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        integer: i64,
    ) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Integer(integer),
        }
    }

    fn visit_float<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        float: f64,
    ) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Float(float),
        }
    }

    fn visit_boolean<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        boolean: bool,
    ) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Boolean(boolean),
        }
    }

    fn visit_null<'a>(&mut self, identifier: Option<&DuperIdentifier<'a>>) -> Self::Value {
        DuperValue {
            identifier: identifier.map(|identifier| identifier.static_clone()),
            inner: DuperInner::Null,
        }
    }
}
