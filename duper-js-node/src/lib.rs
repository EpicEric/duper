use duper::{
    DuperIdentifierTryFromError, DuperObjectTryFromError, DuperParser, DuperTemporalTryFromError,
    DuperValue, PrettyPrinter, Serializer,
};
use napi::{Env, bindgen_prelude::Object};
use napi_derive::napi;

use crate::{
    de::DuperMetaDeserializer,
    ser::{DuperMetaSerializer, SerdeError},
};

mod de;
mod ser;

#[derive(Debug, thiserror::Error)]
pub enum DuperError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),
    #[error("Invalid serialization options: {0}")]
    SerializeOptions(&'static str),
    #[error("Identifier error: {0}")]
    InvalidIdentifier(#[from] DuperIdentifierTryFromError<'static>),
    #[error("Object error: {0}")]
    InvalidObject(#[from] DuperObjectTryFromError<'static>),
    #[error("Temporal error: {0}")]
    InvalidTemporal(#[from] DuperTemporalTryFromError<'static>),
}

#[napi]
pub fn parse(env: &Env, input: String, parse_any: bool) -> anyhow::Result<Object<'_>> {
    let value = match parse_any {
        true => DuperParser::parse_duper_value(&input),
        false => DuperParser::parse_duper_trunk(&input),
    }
    .map_err(|err| {
        DuperError::Parse(
            DuperParser::prettify_error(&input, &err, None).unwrap_or_else(|_| format!("{err:?}")),
        )
    })?;
    Ok(value.serialize_meta(&DuperMetaSerializer::new(env))?)
}

#[napi(object)]
pub struct SerializeOptions {
    pub indent: Option<String>,
    pub strip_identifiers: bool,
    pub minify: bool,
}

#[napi]
pub fn serialize(
    env: &Env,
    value: Object,
    options: Option<SerializeOptions>,
) -> anyhow::Result<String> {
    let SerializeOptions {
        indent,
        strip_identifiers,
        minify,
    } = options.unwrap_or(SerializeOptions {
        indent: None,
        strip_identifiers: false,
        minify: false,
    });
    if let Some(indent) = indent {
        if minify {
            Err(DuperError::SerializeOptions(
                "Cannot serialize Duper value with both indent and minify options",
            )
            .into())
        } else {
            Ok(PrettyPrinter::new(strip_identifiers, indent.as_ref())
                .map_err(DuperError::SerializeOptions)?
                .pretty_print(&DuperValue::deserialize_meta(DuperMetaDeserializer {
                    env,
                    object: value,
                })?))
        }
    } else {
        Ok(
            Serializer::new(strip_identifiers, minify).serialize(&DuperValue::deserialize_meta(
                DuperMetaDeserializer { env, object: value },
            )?),
        )
    }
}
