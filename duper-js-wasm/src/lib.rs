extern crate alloc;

use duper::{DuperParser, PrettyPrinter, Serializer};
use wasm_bindgen::prelude::*;
use wasm_bindgen_derive::TryFromJsValue;

use crate::parse::WasmVisitor;

mod parse;
mod serialize;

#[derive(TryFromJsValue)]
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct DuperObjectEntry {
    pub key: String,
    pub value: DuperValue,
}

#[wasm_bindgen]
impl DuperObjectEntry {
    #[wasm_bindgen(constructor)]
    pub fn new(key: String, value: DuperValue) -> Self {
        Self { key, value }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum DuperValueType {
    Object,
    Array,
    Tuple,
    String,
    Bytes,
    Temporal,
    Integer,
    Float,
    Boolean,
    Null,
}

#[derive(TryFromJsValue)]
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct DuperValue {
    #[wasm_bindgen(js_name = type)]
    pub typ: DuperValueType,
    pub identifier: Option<String>,
    pub value: JsValue,
}

#[wasm_bindgen]
impl DuperValue {
    #[wasm_bindgen(constructor)]
    pub fn new(r#type: DuperValueType, identifier: Option<String>, value: JsValue) -> Self {
        Self {
            typ: r#type,
            identifier,
            value,
        }
    }
}

#[wasm_bindgen]
pub fn parse(input: &str, parse_any: bool) -> Result<DuperValue, JsError> {
    let value = match parse_any {
        true => DuperParser::parse_duper_value(input),
        false => DuperParser::parse_duper_trunk(input),
    }
    .map_err(|err| {
        JsError::new(
            &DuperParser::prettify_error(input, &err, None).unwrap_or_else(|_| format!("{err:?}")),
        )
    })?;
    Ok(value.accept(&mut WasmVisitor))
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct SerializeOptions {
    pub indent: Option<String>,
    #[wasm_bindgen(js_name = stripIdentifiers)]
    pub strip_identifiers: bool,
    pub minify: bool,
}

#[wasm_bindgen]
impl SerializeOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        indent: Option<String>,
        #[wasm_bindgen(js_name = stripIdentifiers)] strip_identifiers: bool,
        minify: bool,
    ) -> Self {
        Self {
            indent,
            strip_identifiers,
            minify,
        }
    }
}

#[wasm_bindgen]
pub fn serialize(value: DuperValue, options: Option<SerializeOptions>) -> Result<String, JsError> {
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
            Err(JsError::new(
                "Cannot serialize Duper value with both indent and minify options",
            ))
        } else {
            Ok(PrettyPrinter::new(strip_identifiers, indent.as_ref())
                .map_err(JsError::new)?
                .pretty_print(value.serialize()?))
        }
    } else {
        Ok(Serializer::new(strip_identifiers, minify).serialize(value.serialize()?))
    }
}
