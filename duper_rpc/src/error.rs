use duper::DuperValue;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    Custom(DuperValue<'static>),
}
