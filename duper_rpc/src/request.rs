use duper::DuperValue;

use crate::{Error, RequestId};

#[derive(Debug)]
pub enum Request {
    Single(RequestCall),
    Batch(Vec<RequestCall>),
}

#[derive(Debug)]
pub enum RequestCall {
    Valid {
        id: Option<RequestId>,
        method: String,
        params: DuperValue<'static>,
    },
    Invalid {
        id: Option<RequestId>,
        error: Error,
    },
}
