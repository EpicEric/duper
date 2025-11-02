use duper::DuperParser;
use wasm_bindgen::prelude::*;

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

    let duper = duper.accept(&mut visitor::EncodeBytesVisitor {});

    match target {
        ConvertTo::Json => {
            serde_json::to_string_pretty(&duper).map_err(|err| JsError::new(&err.to_string()))
        }
        ConvertTo::Yaml => {
            serde_yaml_ng::to_string(&duper).map_err(|err| JsError::new(&err.to_string()))
        }
        ConvertTo::Toml => {
            toml::to_string_pretty(&duper).map_err(|err| JsError::new(&err.to_string()))
        }
    }
}
