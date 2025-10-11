use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    SerializationError,
    InvalidValue,
    UnsupportedType,
    Custom(String),
}

#[derive(Debug, Clone)]
struct ErrorImpl {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug, Clone)]
pub struct Error {
    inner: Box<ErrorImpl>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(ErrorImpl {
                kind,
                message: message.into(),
            }),
        }
    }

    pub fn custom(msg: impl Into<String> + Clone) -> Self {
        Self::new(ErrorKind::Custom(msg.clone().into()), msg)
    }

    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::SerializationError, msg)
    }

    pub fn invalid_value(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::InvalidValue, msg)
    }

    pub fn unsupported_type(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::UnsupportedType, msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            match self.inner.kind {
                ErrorKind::SerializationError => "SerializationError",
                ErrorKind::InvalidValue => "InvalidValue",
                ErrorKind::UnsupportedType => "UnsupportedType",
                ErrorKind::Custom(_) => "Custom",
            },
            self.inner.message
        )
    }
}

impl std::error::Error for Error {}

impl serde_core::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}
