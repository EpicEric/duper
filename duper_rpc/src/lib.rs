pub mod error;
mod handler;
pub mod request;
pub mod response;
pub mod server;

pub use crate::{
    error::Error,
    request::{Request, RequestCall},
    response::{Response, ResponseResult},
    server::server,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestId {
    String(String),
    I64(i64),
}

pub type Result<T> = std::result::Result<T, Error>;
