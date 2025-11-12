use std::collections::HashMap;

use duper::{
    DuperIdentifierTryFromError, DuperObjectTryFromError, DuperParser, DuperTemporalTryFromError,
    PrettyPrinter, Serializer,
};

use crate::parse::UniffiVisitor;

mod parse;
mod serialize;

pub enum DuperValue {
    Object {
        identifier: Option<String>,
        value: HashMap<String, DuperValue>,
    },
    Array {
        identifier: Option<String>,
        value: Vec<DuperValue>,
    },
    Tuple {
        identifier: Option<String>,
        value: Vec<DuperValue>,
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

#[derive(Debug, thiserror::Error)]
pub enum DuperError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Invalid options: {0}")]
    Options(&'static str),
    #[error("Identifier error: {0}")]
    InvalidIdentifier(#[from] DuperIdentifierTryFromError<'static>),
    #[error("Object error: {0}")]
    InvalidObject(#[from] DuperObjectTryFromError<'static>),
    #[error("Temporal error: {0}")]
    InvalidTemporal(#[from] DuperTemporalTryFromError<'static>),
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

pub fn serialize(
    value: DuperValue,
    strip_identifiers: bool,
    minify: bool,
    indent: Option<String>,
) -> Result<String, DuperError> {
    let value = value.serialize()?;
    if let Some(indent) = indent {
        if minify {
            Err(DuperError::Options(
                "Cannot serialize Duper value with both indent and minify options",
            ))
        } else {
            Ok(PrettyPrinter::new(strip_identifiers, indent.as_ref())
                .map_err(DuperError::Options)?
                .pretty_print(value))
        }
    } else {
        Ok(Serializer::new(strip_identifiers, minify).serialize(value))
    }
}

uniffi::include_scaffolding!("duper");
