#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//! # duper_rpc
//!
//! An RPC implementation for Duper.
//!
//! This crate contains definitions of the base parts of the Duper RPC
//! (including requests, responses, and errors), as well as an implementation
//! of a [`tower`] based server (see [`server`]) and a [`RequestBuilder`].

mod de;
mod error;
mod handler;
pub mod request;
pub mod response;
mod ser;
pub mod server;

use duper::{DuperIdentifier, DuperValue};

pub use crate::{
    error::Error,
    request::{Request, RequestBuilder, RequestCall},
    response::{Response, ResponseError, ResponseResult, ResponseSuccess},
    server::{Server, State},
};

#[derive(Debug, Clone, PartialEq)]
pub enum RequestId {
    String {
        identifier: Option<DuperIdentifier<'static>>,
        inner: String,
    },
    Integer {
        identifier: Option<DuperIdentifier<'static>>,
        inner: i64,
    },
}

impl TryFrom<DuperValue<'static>> for RequestId {
    type Error = &'static str;

    fn try_from(value: DuperValue<'static>) -> std::result::Result<Self, Self::Error> {
        match value {
            DuperValue::String { identifier, inner } => Ok(RequestId::String {
                identifier,
                inner: inner.into_owned(),
            }),
            DuperValue::Integer { identifier, inner } => {
                Ok(RequestId::Integer { identifier, inner })
            }
            DuperValue::Object { .. } => Err("expected string or integer, found object"),
            DuperValue::Array { .. } => Err("expected string or integer, found array"),
            DuperValue::Tuple { .. } => Err("expected string or integer, found tuple"),
            DuperValue::Bytes { .. } => Err("expected string or integer, found bytes"),
            DuperValue::Temporal(_) => Err("expected string or integer, found Temporal value"),
            DuperValue::Float { .. } => Err("expected string or integer, found float"),
            DuperValue::Boolean { .. } => Err("expected string or integer, found boolean"),
            DuperValue::Null { .. } => Err("expected string or integer, found null"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DuperRpcVersion {
    DuperRpc01,
}

pub type Result<T> = std::result::Result<T, Error>;
