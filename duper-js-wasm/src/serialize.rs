use std::borrow::Cow;

use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTemporal, DuperTuple, DuperValue,
};
use js_sys::{BigInt, Boolean, Number, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_derive::try_from_js_array;

use crate::{DuperObjectEntry, DuperValue as Value, DuperValueType};

impl Value {
    pub(crate) fn serialize(self) -> Result<DuperValue<'static>, JsError> {
        match self.typ {
            DuperValueType::Object => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Object(DuperObject::try_from(
                    try_from_js_array(self.value)
                        .map_err(|_| JsError::new("Cannot serialize Duper object"))?
                        .into_iter()
                        .map(|DuperObjectEntry { key, value }| {
                            value.serialize().map(|val| (DuperKey::from(key), val))
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )?),
            }),
            DuperValueType::Array => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Array(DuperArray::from(
                    try_from_js_array::<Value>(self.value)
                        .map_err(|_| JsError::new("Cannot serialize Duper array"))?
                        .into_iter()
                        .map(|val| val.serialize())
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            }),
            DuperValueType::Tuple => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Tuple(DuperTuple::from(
                    try_from_js_array::<Value>(self.value)
                        .map_err(|_| JsError::new("Cannot serialize Duper tuple"))?
                        .into_iter()
                        .map(|val| val.serialize())
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            }),
            DuperValueType::String => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::String(DuperString::from(
                    self.value
                        .as_string()
                        .ok_or_else(|| JsError::new("Cannot serialize Duper string"))?,
                )),
            }),
            DuperValueType::Bytes => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Bytes(DuperBytes::from(
                    self.value
                        .dyn_into::<Uint8Array>()
                        .map_err(|_| JsError::new("Cannot serialize Duper string"))?
                        .to_vec(),
                )),
            }),
            DuperValueType::Temporal => Ok(DuperValue {
                inner: DuperInner::Temporal(match self.identifier.as_ref().map(AsRef::as_ref) {
                    Some("Instant") => DuperTemporal::try_instant_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some("ZonedDateTime") => DuperTemporal::try_zoned_date_time_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some("PlainDate") => DuperTemporal::try_plain_date_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some("PlainTime") => DuperTemporal::try_plain_time_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some("PlainDateTime") => DuperTemporal::try_plain_date_time_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some("PlainYearMonth") => {
                        DuperTemporal::try_plain_year_month_from(Cow::Owned(
                            self.value
                                .as_string()
                                .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                        ))?
                    }
                    Some("PlainMonthDay") => DuperTemporal::try_plain_month_day_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some("Duration") => DuperTemporal::try_duration_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                    Some(_) | None => DuperTemporal::try_unspecified_from(Cow::Owned(
                        self.value
                            .as_string()
                            .ok_or_else(|| JsError::new("Cannot serialize Duper temporal"))?,
                    ))?,
                }),
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
            }),
            DuperValueType::Integer => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Integer(
                    self.value
                        .dyn_into::<BigInt>()
                        .map_err(|_| JsError::new("Cannot serialize Duper integer"))?
                        .try_into()
                        .map_err(|_| JsError::new("Cannot serialize Duper integer"))?,
                ),
            }),
            DuperValueType::Float => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Float(
                    self.value
                        .dyn_into::<Number>()
                        .map_err(|_| JsError::new("Cannot serialize Duper float"))?
                        .into(),
                ),
            }),
            DuperValueType::Boolean => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Boolean(
                    self.value
                        .dyn_into::<Boolean>()
                        .map_err(|_| JsError::new("Cannot serialize Duper boolean"))?
                        .into(),
                ),
            }),
            DuperValueType::Null => Ok(DuperValue {
                identifier: self
                    .identifier
                    .map(|identifier| DuperIdentifier::try_from(identifier))
                    .transpose()?,
                inner: DuperInner::Null,
            }),
        }
    }
}
