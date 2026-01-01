use duper::DuperValue;

use crate::{Error, RequestId};

#[derive(Debug)]
pub enum Response {
    Single(ResponseResult),
    Batch(Vec<ResponseResult>),
}

#[derive(Debug)]
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
