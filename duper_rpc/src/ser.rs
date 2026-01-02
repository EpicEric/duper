use serde_core::{
    Serialize,
    ser::{SerializeMap, SerializeSeq},
};

use crate::{Error, RequestId, Response, ResponseResult};

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            Error::ParseError => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "ParseError")?;
                map.end()
            }
            Error::InvalidRequest => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "InvalidRequest")?;
                map.end()
            }
            Error::MethodNotFound => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "MethodNotFound")?;
                map.end()
            }
            Error::InvalidParams => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "InvalidParams")?;
                map.end()
            }
            Error::InternalError => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "InternalError")?;
                map.end()
            }
            Error::Custom(value) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Custom")?;
                map.serialize_entry("value", value)?;
                map.end()
            }
        }
    }
}

impl Serialize for ResponseResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            ResponseResult::Valid { id, result } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("duper_rpc", "0.1")?;
                match id {
                    RequestId::String(id) => map.serialize_entry("id", id)?,
                    RequestId::I64(id) => map.serialize_entry("id", id)?,
                }
                map.serialize_entry("result", result)?;
                map.end()
            }
            ResponseResult::Invalid { id, error } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("duper_rpc", "0.1")?;
                match id {
                    Some(RequestId::String(id)) => map.serialize_entry("id", id)?,
                    Some(RequestId::I64(id)) => map.serialize_entry("id", id)?,
                    None => map.serialize_entry("id", &Option::<&str>::None)?,
                }
                map.serialize_entry("error", error)?;
                map.end()
            }
        }
    }
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            Response::Single(result) => result.serialize(serializer),
            Response::Batch(result_vec) => {
                let mut seq = serializer.serialize_seq(Some(result_vec.len()))?;
                for result in result_vec {
                    seq.serialize_element(result)?;
                }
                seq.end()
            }
        }
    }
}
