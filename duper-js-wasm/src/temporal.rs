use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "temporal-polyfill")]
unsafe extern "C" {
    // Instant
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type Instant;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &Instant) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "Instant"])]
    pub(crate) fn from(value: &str) -> Instant;

    // ZonedDateTime
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type ZonedDateTime;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &ZonedDateTime) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "ZonedDateTime"])]
    pub(crate) fn from(value: &str) -> ZonedDateTime;

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

    // PlainYearMonth
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type PlainYearMonth;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &PlainYearMonth) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "PlainYearMonth"])]
    pub(crate) fn from(value: &str) -> PlainYearMonth;

    // PlainMonthDay
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type PlainMonthDay;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &PlainMonthDay) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "PlainMonthDay"])]
    pub(crate) fn from(value: &str) -> PlainMonthDay;

    // Duration
    #[wasm_bindgen(js_namespace = ["Temporal"])]
    pub(crate) type Duration;

    #[wasm_bindgen(method, js_name = toString)]
    pub(crate) fn to_string(this: &Duration) -> String;

    #[wasm_bindgen(js_namespace = ["Temporal", "Duration"])]
    pub(crate) fn from(value: &str) -> Duration;
}
