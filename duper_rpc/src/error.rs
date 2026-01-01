#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    Custom { code: i64, message: String },
}
