//! Definitions for Duper RPC responses.

use duper::DuperValue;

use crate::{Error, RequestId};

/// Representation of an RPC response.
#[derive(Debug, Clone)]
pub enum Response {
    /// An RPC response containing a single result.
    Single(ResponseResult),
    /// An RPC response containing batch results.
    Batch(Vec<ResponseResult>),
}

impl IntoIterator for Response {
    type Item = ResponseResult;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Response::Single(result) => vec![result].into_iter(),
            Response::Batch(result_vec) => result_vec.into_iter(),
        }
    }
}

/// A result from an RPC request.
pub type ResponseResult = Result<ResponseSuccess, ResponseError>;

/// A successful response.
#[derive(Debug, Clone)]
pub struct ResponseSuccess {
    /// The ID of the RPC request.
    pub id: RequestId,
    /// The result of the RPC call.
    pub result: DuperValue<'static>,
}

/// An error response.
#[derive(Debug, Clone)]
pub struct ResponseError {
    /// The ID of the RPC request, if known.
    pub id: Option<RequestId>,
    /// The error associated with the request.
    pub error: Error,
}
