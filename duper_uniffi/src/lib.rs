use std::collections::HashMap;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperObject, DuperParser, DuperString, DuperTemporal,
    DuperTuple, visitor::DuperVisitor,
};

pub struct BoxedDuperValue(pub Box<DuperValue>);
uniffi::custom_type!(BoxedDuperValue, DuperValue, {
    lower: |boxed| *boxed.0,
    try_lift: |val| Ok(BoxedDuperValue(Box::new(val))),
});

pub enum DuperValue {
    Object {
        identifier: Option<String>,
        value: HashMap<String, BoxedDuperValue>,
    },
    Array {
        identifier: Option<String>,
        value: Vec<BoxedDuperValue>,
    },
    Tuple {
        identifier: Option<String>,
        value: Vec<BoxedDuperValue>,
    },
    String {
        identifier: Option<String>,
        value: String,
    },
    Bytes {
        identifier: Option<String>,
        value: Vec<u8>,
    },
    Temporal {
        identifier: Option<String>,
        value: String,
    },
    Integer {
        identifier: Option<String>,
        value: i64,
    },
    Float {
        identifier: Option<String>,
        value: f64,
    },
    Boolean {
        identifier: Option<String>,
        value: bool,
    },
    Null {
        identifier: Option<String>,
    },
}

struct UniffiVisitor;

impl DuperVisitor for UniffiVisitor {
    type Value = DuperValue;

    fn visit_object<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        object: &DuperObject<'a>,
    ) -> Self::Value {
        let mut value = HashMap::with_capacity(object.len());
        for (key, val) in object.iter() {
            value.insert(
                key.as_ref().to_string(),
                BoxedDuperValue(Box::new(val.accept(self).into())),
            );
        }
        DuperValue::Object {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value,
        }
    }

    fn visit_array<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        array: &DuperArray<'a>,
    ) -> Self::Value {
        let mut value = Vec::with_capacity(array.len());
        for val in array.iter() {
            value.push(BoxedDuperValue(Box::new(val.accept(self).into())));
        }
        DuperValue::Array {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value,
        }
    }

    fn visit_tuple<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        tuple: &DuperTuple<'a>,
    ) -> Self::Value {
        let mut value = Vec::with_capacity(tuple.len());
        for val in tuple.iter() {
            value.push(BoxedDuperValue(Box::new(val.accept(self).into())));
        }
        DuperValue::Tuple {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value,
        }
    }

    fn visit_string<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        string: &DuperString<'a>,
    ) -> Self::Value {
        DuperValue::String {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: string.as_ref().to_string(),
        }
    }

    fn visit_bytes<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        bytes: &DuperBytes<'a>,
    ) -> Self::Value {
        DuperValue::Bytes {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
            value: bytes.as_ref().to_vec(),
        }
    }

    fn visit_temporal<'a>(
        &mut self,
        identifier: Option<&DuperIdentifier<'a>>,
        temporal: &DuperTemporal<'a>,
    ) -> Self::Value {
        DuperValue::Temporal {
            identifier: identifier.map(|identifier| identifier.as_ref().to_string()),
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

#[derive(Debug, thiserror::Error)]
pub enum DuperError {
    #[error("Parse error: {0}")]
    Parse(String),
}

pub fn parse(input: &str, parse_any: bool) -> Result<DuperValue, DuperError> {
    let value = match parse_any {
        true => DuperParser::parse_duper_value(input),
        false => DuperParser::parse_duper_trunk(input),
    }
    .map_err(|err| {
        DuperError::Parse(
            DuperParser::prettify_error(input, &err, None).unwrap_or_else(|_| format!("{err:?}")),
        )
    })?;
    Ok(value.accept(&mut UniffiVisitor))
}

uniffi::include_scaffolding!("duper");
