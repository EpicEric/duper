use duper::{DuperParser, DuperValue, PrettyPrinter, Serializer};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::{de::Visitor, ser::serialize_jsvalue};

mod de;
mod repr;
mod ser;

struct StringifyOptions {
    indent: Option<String>,
    strip_identifiers: bool,
    minify: bool,
}

#[wasm_bindgen(typescript_custom_section)]
const I_STRINGIFY_OPTIONS: &'static str = r#"
/**
 * @property {string | number} [indent] - Whitespace string to use as indentation, or the number of
 * spaces to use as indentation.
 * @property {boolean} [stripIdentifiers] - Whether Duper identifiers should be removed from the stringified value.
 * @property {boolean} [minify] - Whether stringify should minify the value. Not compatible with `indent`.
 */
interface IStringifyOptions {
    indent?: string | number;
    stripIdentifiers?: boolean;
    minify?: boolean;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IStringifyOptions")]
    pub type IStringifyOptions;
}

/// Stringifies the provided value into Duper, with the specified options.
#[wasm_bindgen]
pub fn stringify(value: &JsValue, options: Option<IStringifyOptions>) -> Result<String, JsError> {
    let value: DuperValue = serialize_jsvalue(value)
        .map_err(|e| JsError::new(&format!("Failed to serialize into Duper value: {:?}", e)))?;
    let options: JsValue = options.into();
    let StringifyOptions {
        indent,
        strip_identifiers,
        minify,
    } = if options.is_truthy() {
        let indent = Reflect::get(&options, &JsValue::from_str("indent"))
            .map_err(|e| JsError::new(&format!("Failed to get property indent: {:?}", e)))?;
        let strip_identifiers = Reflect::get(&options, &JsValue::from_str("stripIdentifiers"))
            .map_err(|e| {
                JsError::new(&format!("Failed to get property stripIdentifiers: {:?}", e))
            })?;
        let minify = Reflect::get(&options, &JsValue::from_str("minify"))
            .map_err(|e| JsError::new(&format!("Failed to get property minify: {:?}", e)))?;
        StringifyOptions {
            indent: if let Some(indent) = indent.as_f64() {
                Some((0..indent as u32).map(|_| ' ').collect())
            } else {
                indent.as_string()
            },
            strip_identifiers: strip_identifiers.is_truthy(),
            minify: minify.is_truthy(),
        }
    } else {
        StringifyOptions {
            indent: None,
            strip_identifiers: false,
            minify: false,
        }
    };
    if let Some(indent) = indent {
        if minify {
            Err(JsError::new(
                "Cannot stringify with both indent and minify options",
            ))
        } else {
            Ok(PrettyPrinter::new(strip_identifiers, &indent)
                .map_err(|e| JsError::new(&format!("Failed to pretty print Duper: {:?}", e)))?
                .pretty_print(value))
        }
    } else {
        Ok(Serializer::new(strip_identifiers, minify).serialize(value))
    }
}

/// Parses the provided Duper string into a DuperValue, or a JSON-safe alternative if specified.
#[wasm_bindgen]
pub fn parse(
    value: &str,
    #[wasm_bindgen(js_name = "jsonSafe")] json_safe: Option<bool>,
) -> Result<JsValue, JsError> {
    let value = DuperParser::parse_duper_value(value).map_err(|err| {
        JsError::new(
            &DuperParser::prettify_error(value, &err, None).unwrap_or_else(|_| format!("{err:?}")),
        )
    })?;
    let deserialized = value.accept(&mut Visitor)?;
    if json_safe.is_some_and(|json_safe| json_safe) {
        deserialized
            .to_json()
            .map_err(|err| JsError::new(&format!("failed to make JSON-safe: {err:?}")))
    } else {
        Ok(deserialized.into())
    }
}
