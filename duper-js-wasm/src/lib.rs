use duper::DuperParser;
use wasm_bindgen::prelude::*;

use crate::de::Visitor;

mod de;

#[wasm_bindgen]
pub fn stringify(value: JsValue) -> Result<String, JsError> {
    todo!()
}

#[wasm_bindgen]
pub fn parse(value: &str) -> Result<JsValue, JsError> {
    let value = DuperParser::parse_duper_value(value)
        .map_err(|err| JsError::new(&format!("{:?}", miette::Error::new(err.into_miette()))))?;
    value.accept(&mut Visitor)
}
