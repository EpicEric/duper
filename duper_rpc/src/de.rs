use duper::DuperValue;
use serde_core::{Deserialize, de::Visitor};

use crate::{DuperRpcVersion, Error, Request, RequestCall, RequestId};

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
