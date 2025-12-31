use duper::{DuperParser, ToJson};
use serde_core::Serialize;
use wasm_bindgen::prelude::*;

mod temporal;
mod visitor;

#[wasm_bindgen(typescript_custom_section)]
const I_STRINGIFY_OPTIONS: &'static str = r#"
type ConvertDuperTo = "json" | "yaml" | "toml";"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ConvertDuperTo")]
    pub type ConvertDuperTo;
}

enum ConvertTo {
    Json,
    Yaml,
    Toml,
}

impl TryFrom<&'_ str> for ConvertTo {
    type Error = JsError;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        match value {
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            "toml" => Ok(Self::Toml),
            _ => Err(JsError::new(&format!("unknown encoding {value}"))),
        }
    }
}

#[wasm_bindgen(js_name = convertDuper)]
pub fn convert_duper(value: &str, to: Option<ConvertDuperTo>) -> Result<String, JsError> {
    let target = to
        .map(|target| {
            let target: JsValue = target.into();
            let Some(target) = target.as_string() else {
                return Err(JsError::new("`to` argument must be a string"));
            };
            ConvertTo::try_from(target.as_str())
        })
        .unwrap_or(Ok(ConvertTo::Json))?;

    let duper = DuperParser::parse_duper_value(value).map_err(|err| {
        let err = err.first().expect("at least one error");
        let mut line = 1;
        let mut column = 1;
        for char in value.chars().take(err.span().start) {
            if char == '\n' || char == '\r' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        JsError::new(&format!(
            "Line {} column {} | {}",
            line,
            column,
            err.reason()
        ))
    })?;

    match target {
        ConvertTo::Json => serde_json::to_string_pretty(&duper.accept(&mut ToJson {}))
            .map_err(|err| JsError::new(&err.to_string())),
        ConvertTo::Yaml => match duper.accept(&mut visitor::SaphyrVisitor {}) {
            Ok(yaml) => {
                let yaml: saphyr::Yaml = (&yaml).into();
                let mut out = String::new();
                let mut emitter = saphyr::YamlEmitter::new(&mut out);
                emitter.multiline_strings(true);
                emitter
                    .dump(&yaml)
                    .map_err(|err| JsError::new(&err.to_string()))?;
                Ok(out)
            }
            Err(err) => Err(JsError::new(&err)),
        },
        ConvertTo::Toml => match duper.accept(&mut visitor::TomlVisitor {}) {
            Ok(Some(toml::Value::Table(table))) => {
                // let mut out = String::new();
                let mut buf = toml::ser::Buffer::new();
                let serializer = toml::Serializer::pretty(&mut buf);
                table
                    .serialize(serializer)
                    .map_err(|err| JsError::new(&err.to_string()))?;
                Ok(buf.to_string())
            }
            Ok(Some(value)) => Err(JsError::new(&format!(
                "TOML only supports tables as the root value, not {}",
                value.type_str()
            ))),
            Ok(None) => Err(JsError::new("Cannot serialize null in TOML")),
            Err(err) => Err(JsError::new(&err)),
        },
    }
}
