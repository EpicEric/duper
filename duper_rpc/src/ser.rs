use serde_core::{
    Serialize,
    ser::{SerializeMap, SerializeSeq},
};

use crate::{
    Error, Request, RequestCall, RequestId, Response, ResponseError, ResponseResult,
    ResponseSuccess,
};

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
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Custom")?;
                map.serialize_entry("value", value)?;
                map.end()
            }
        }
    }
}

impl Serialize for RequestId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            RequestId::String { inner, .. } => serializer.serialize_str(inner),
            RequestId::Integer { inner, .. } => serializer.serialize_i64(*inner),
        }
    }
}

struct SerializableResponseResult<'a>(&'a ResponseResult);

impl<'a> Serialize for SerializableResponseResult<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self.0 {
            ResponseResult::Ok(ResponseSuccess { id, result }) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("duper_rpc", "0.1")?;
                map.serialize_entry("id", id)?;
                map.serialize_entry("result", result)?;
                map.end()
            }
            ResponseResult::Err(ResponseError { id, error }) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("duper_rpc", "0.1")?;
                match id {
                    Some(id) => map.serialize_entry("id", id)?,
                    None => map.serialize_entry("id", &Option::<&str>::None)?,
                }
                map.serialize_entry("error", error)?;
                map.end()
            }
        }
    }
}

struct ResponseBatch<'a>(&'a Vec<ResponseResult>);

impl<'a> Serialize for ResponseBatch<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for result in self.0 {
            seq.serialize_element(&SerializableResponseResult(result))?;
        }
        seq.end()
    }
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            Response::Single(result) => serializer
                .serialize_newtype_struct("RpcResponse", &SerializableResponseResult(result)),
            Response::Batch(result_vec) => {
                serializer.serialize_newtype_struct("RpcResponse", &ResponseBatch(result_vec))
            }
        }
    }
}

impl Serialize for RequestCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            RequestCall::Valid { id, method, params } => match id {
                Some(id) => {
                    let mut map = serializer.serialize_map(Some(4))?;
                    map.serialize_entry("duper_rpc", "0.1")?;
                    map.serialize_entry("id", id)?;
                    map.serialize_entry("method", method)?;
                    map.serialize_entry("params", params)?;
                    map.end()
                }
                None => {
                    let mut map = serializer.serialize_map(Some(3))?;
                    map.serialize_entry("duper_rpc", "0.1")?;
                    map.serialize_entry("method", method)?;
                    map.serialize_entry("params", params)?;
                    map.end()
                }
            },
            RequestCall::Invalid { .. } => Err(serde_core::ser::Error::custom(
                "cannot serialize invalid RPC request",
            )),
        }
    }
}

struct RequestBatch<'a>(&'a Vec<RequestCall>);

impl<'a> Serialize for RequestBatch<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for result in self.0 {
            seq.serialize_element(result)?;
        }
        seq.end()
    }
}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            Request::Single(call) => serializer.serialize_newtype_struct("RpcRequest", call),
            Request::Batch(call_vec) => {
                serializer.serialize_newtype_struct("RpcRequest", &RequestBatch(call_vec))
            }
        }
    }
}
