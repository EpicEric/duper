use duper::DuperValue;

use crate::{Error, RequestId};

#[derive(Debug, Clone)]
pub enum Response {
    Single(ResponseResult),
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

#[derive(Debug, Clone)]
pub enum ResponseResult {
    Valid {
        id: RequestId,
        result: DuperValue<'static>,
    },
    Invalid {
        id: Option<RequestId>,
        error: Error,
    },
}
