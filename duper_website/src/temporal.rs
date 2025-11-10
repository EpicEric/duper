use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@js-temporal/polyfill")]
unsafe extern "C" {
    // Instant
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type Instant;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &Instant) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "Instant"])]
    pub(crate) fn from(value: &str) -> Instant;

    // PlainDate
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type PlainDate;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &PlainDate) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "PlainDate"])]
    pub(crate) fn from(value: &str) -> PlainDate;

    // PlainTime
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type PlainTime;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &PlainTime) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "PlainTime"])]
    pub(crate) fn from(value: &str) -> PlainTime;

    // PlainDateTime
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type PlainDateTime;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &PlainDateTime) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "PlainDateTime"])]
    pub(crate) fn from(value: &str) -> PlainDateTime;
}
