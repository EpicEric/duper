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

pub use crate::{
    error::Error,
    request::{Request, RequestBuilder, RequestCall},
    response::{Response, ResponseError, ResponseResult, ResponseSuccess},
    server::{Server, State},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestId {
    String(String),
    I64(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DuperRpcVersion {
    DuperRpc01,
}

pub type Result<T> = std::result::Result<T, Error>;
