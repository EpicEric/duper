use duper::DuperValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    Custom(DuperValue<'static>),
}
