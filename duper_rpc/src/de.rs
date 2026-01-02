use duper::DuperValue;
use serde_core::{Deserialize, de::Visitor};

use crate::{DuperRpcVersion, Error, Request, RequestCall, RequestId, Response, ResponseResult};

struct ErrorVisitor {}

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = Error;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a duper_rpc error")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::MapAccess<'de>,
    {
        let mut typ: Option<String> = None;
        let mut value: Option<DuperValue<'de>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "type" => {
                    if typ.is_some() {
                        return Err(serde_core::de::Error::duplicate_field("type"));
                    } else {
                        typ = Some(map.next_value()?);
                    }
                }
                "value" => {
                    if value.is_some() {
                        return Err(serde_core::de::Error::duplicate_field("value"));
                    } else {
                        value = Some(map.next_value()?);
                    }
                }
                field => {
                    return Err(serde_core::de::Error::unknown_field(
                        field,
                        &["type", "value"],
                    ));
                }
            }
        }

        let Some(typ) = typ else {
            return Err(serde_core::de::Error::missing_field("type"));
        };
        match typ.as_ref() {
            "ParseError" => {
                if value.is_some() {
                    return Err(serde_core::de::Error::custom(
                        "ParseError cannot have value",
                    ));
                }
                Ok(Error::ParseError)
            }
            "InvalidRequest" => {
                if value.is_some() {
                    return Err(serde_core::de::Error::custom(
                        "InvalidRequest cannot have value",
                    ));
                }
                Ok(Error::InvalidRequest)
            }
            "MethodNotFound" => {
                if value.is_some() {
                    return Err(serde_core::de::Error::custom(
                        "MethodNotFound cannot have value",
                    ));
                }
                Ok(Error::MethodNotFound)
            }
            "InvalidParams" => {
                if value.is_some() {
                    return Err(serde_core::de::Error::custom(
                        "InvalidParams cannot have value",
                    ));
                }
                Ok(Error::InvalidParams)
            }
            "InternalError" => {
                if value.is_some() {
                    return Err(serde_core::de::Error::custom(
                        "InternalError cannot have value",
                    ));
                }
                Ok(Error::InternalError)
            }
            "Custom" => {
                let Some(value) = value else {
                    return Err(serde_core::de::Error::missing_field("value"));
                };
                Ok(Error::Custom(value.static_clone()))
            }
            typ => Err(serde_core::de::Error::unknown_field(
                typ,
                &[
                    "ParseError",
                    "InvalidRequest",
                    "MethodNotFound",
                    "InvalidParams",
                    "InternalError",
                    "Custom",
                ],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        deserializer.deserialize_map(ErrorVisitor {})
    }
}

struct RequestIdVisitor {}

impl<'de> Visitor<'de> for RequestIdVisitor {
    type Value = RequestId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a duper_rpc request ID")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v.into())
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v.into())
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v.into())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(RequestId::I64(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v.into())
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v.into())
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_i64(v.into())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        if v > i64::MAX as u64 {
            Err(serde_core::de::Error::custom("Invalid ID"))
        } else {
            self.visit_i64(v as i64)
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        Ok(RequestId::String(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde_core::de::Error,
    {
        self.visit_string(v.to_string())
    }
}

impl<'de> Deserialize<'de> for RequestId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        deserializer.deserialize_any(RequestIdVisitor {})
    }
}

struct RequestCallVisitor {}

impl<'de> Visitor<'de> for RequestCallVisitor {
    type Value = RequestCall;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a single duper_rpc request call")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::MapAccess<'de>,
    {
        let mut duper_rpc: Option<DuperRpcVersion> = None;
        let mut id: Option<RequestId> = None;
        let mut method: Option<String> = None;
        let mut params: Option<DuperValue<'de>> = None;
        let mut invalid_request = false;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "duper_rpc" => {
                    if duper_rpc.is_some() {
                        invalid_request = true;
                    } else {
                        match map.next_value::<String>()?.as_ref() {
                            "0.1" => duper_rpc = Some(DuperRpcVersion::DuperRpc01),
                            _ => {
                                invalid_request = true;
                            }
                        }
                    }
                }
                "id" => {
                    if id.is_some() {
                        invalid_request = true;
                    } else {
                        id = Some(map.next_value()?);
                    }
                }
                "method" => {
                    if method.is_some() {
                        invalid_request = true;
                    } else {
                        method = Some(map.next_value()?);
                    }
                }
                "params" => {
                    if params.is_some() {
                        invalid_request = true;
                    } else {
                        params = Some(map.next_value()?);
                    }
                }
                _ => {
                    invalid_request = true;
                }
            }
        }

        if invalid_request {
            Ok(RequestCall::Invalid {
                id,
                error: Error::InvalidRequest,
            })
        } else {
            match duper_rpc {
                Some(DuperRpcVersion::DuperRpc01) => {
                    let Some(method) = method else {
                        return Ok(RequestCall::Invalid {
                            id,
                            error: Error::InvalidRequest,
                        });
                    };
                    Ok(RequestCall::Valid {
                        id,
                        method,
                        params: params.map(|value| value.static_clone()).unwrap_or(
                            DuperValue::Tuple {
                                identifier: None,
                                inner: vec![],
                            },
                        ),
                    })
                }
                None => Ok(RequestCall::Invalid {
                    id,
                    error: Error::InvalidRequest,
                }),
            }
        }
    }
}

impl<'de> Deserialize<'de> for RequestCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        deserializer.deserialize_map(RequestCallVisitor {})
    }
}

struct RequestVisitor {}

impl<'de> Visitor<'de> for RequestVisitor {
    type Value = Request;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a duper_rpc request")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::MapAccess<'de>,
    {
        Ok(Request::Single(
            RequestCallVisitor {}
                .visit_map(map)
                .unwrap_or_else(|_| RequestCall::Invalid {
                    id: None,
                    error: Error::InvalidRequest,
                }),
        ))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::SeqAccess<'de>,
    {
        let mut vec = seq.size_hint().map(Vec::with_capacity).unwrap_or_default();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(Request::Batch(vec))
    }
}

impl<'de> Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        Ok(deserializer
            .deserialize_any(RequestVisitor {})
            .unwrap_or_else(|_| {
                Request::Single(RequestCall::Invalid {
                    id: None,
                    error: Error::InvalidRequest,
                })
            }))
    }
}

struct ResponseResultVisitor {}

impl<'de> Visitor<'de> for ResponseResultVisitor {
    type Value = ResponseResult;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a single duper_rpc response result")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::MapAccess<'de>,
    {
        let mut duper_rpc: Option<DuperRpcVersion> = None;
        let mut id: Option<RequestId> = None;
        let mut result: Option<DuperValue<'de>> = None;
        let mut error: Option<Error> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "duper_rpc" => {
                    if duper_rpc.is_some() {
                        return Err(serde_core::de::Error::duplicate_field("duper_rpc"));
                    } else {
                        match map.next_value::<String>()?.as_ref() {
                            "0.1" => duper_rpc = Some(DuperRpcVersion::DuperRpc01),
                            version => {
                                return Err(serde_core::de::Error::invalid_value(
                                    serde_core::de::Unexpected::Str(version),
                                    &"one of: \"0.1\"",
                                ));
                            }
                        }
                    }
                }
                "id" => {
                    if id.is_some() {
                        return Err(serde_core::de::Error::duplicate_field("id"));
                    } else {
                        id = Some(map.next_value()?);
                    }
                }
                "result" => {
                    if result.is_some() {
                        return Err(serde_core::de::Error::duplicate_field("result"));
                    } else {
                        result = Some(map.next_value()?);
                    }
                }
                "error" => {
                    if error.is_some() {
                        return Err(serde_core::de::Error::duplicate_field("error"));
                    } else {
                        error = Some(map.next_value()?);
                    }
                }
                field => {
                    return Err(serde_core::de::Error::unknown_field(
                        field,
                        &["duper_rpc", "id", "result", "error"],
                    ));
                }
            }
        }

        match duper_rpc {
            Some(DuperRpcVersion::DuperRpc01) => match (result, error) {
                (Some(result), None) => {
                    let Some(id) = id else {
                        return Err(serde_core::de::Error::missing_field("id"));
                    };
                    Ok(ResponseResult::Valid {
                        id,
                        result: result.static_clone(),
                    })
                }
                (None, Some(error)) => Ok(ResponseResult::Invalid { id, error }),
                (None, None) => Err(serde_core::de::Error::missing_field("result")),
                (Some(_), Some(_)) => Err(serde_core::de::Error::custom(
                    "cannot have both result and error in response",
                )),
            },
            None => Err(serde_core::de::Error::missing_field("duper_rpc")),
        }
    }
}

impl<'de> Deserialize<'de> for ResponseResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        deserializer.deserialize_map(ResponseResultVisitor {})
    }
}

struct ResponseVisitor {}

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = Response;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a duper_rpc response")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::MapAccess<'de>,
    {
        Ok(Response::Single(ResponseResultVisitor {}.visit_map(map)?))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde_core::de::SeqAccess<'de>,
    {
        let mut vec = seq.size_hint().map(Vec::with_capacity).unwrap_or_default();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(Response::Batch(vec))
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        Ok(deserializer.deserialize_any(ResponseVisitor {})?)
    }
}
