use duper::DuperValue;

/// Possible errors when calling a Duper RPC endpoint.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// RPC request couldn't be parsed.
    ParseError,
    /// RPC request was invalid.
    InvalidRequest,
    /// RPC method not found.
    MethodNotFound,
    /// Parameters are invalid.
    InvalidParams,
    /// Unspecified server error.
    InternalError,
    /// A custom error containing arbitrary data.
    Custom(DuperValue<'static>),
}
