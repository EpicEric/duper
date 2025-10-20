use std::fmt::{self, Display};

use duper::{DuperIdentifierTryFromError, DuperObjectTryFromError};

/// The kinds of errors that can happen during serialization and deserialization.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// Parsing failed at the given [`pest`] rule.
    ///
    /// This error implements `.to_miette()`, in order to allow generation of a
    /// `miette` `Report`.
    ParseError(Box<pest::error::Error<duper::DuperRule>>),
    /// Serialization failed with an unspecified error.
    SerializationError,
    /// Deserialization failed with the given reason.
    DeserializationError(serde_core::de::value::Error),
    /// An invalid value was provided.
    InvalidValue,
    /// Unspecified conditions.
    Custom,
}

/// This type includes the error kind and message associated with the failure.
#[derive(Debug, Clone)]
pub struct ErrorImpl {
    pub kind: ErrorKind,
    pub message: String,
}

/// This type represents all possible errors that can occur when serializing or
/// deserializing Duper data.
#[derive(Debug, Clone)]
pub struct Error {
    pub inner: Box<ErrorImpl>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(ErrorImpl {
                kind,
                message: message.into(),
            }),
        }
    }

    pub(crate) fn custom(msg: impl Into<String> + Clone) -> Self {
        Self::new(ErrorKind::Custom, msg)
    }

    pub(crate) fn serialization(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::SerializationError, msg)
    }

    pub(crate) fn invalid_value(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::InvalidValue, msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            match self.inner.kind {
                ErrorKind::ParseError(_) => "ParseError",
                ErrorKind::SerializationError => "SerializationError",
                ErrorKind::DeserializationError(_) => "DeserializationError",
                ErrorKind::InvalidValue => "InvalidValue",
                ErrorKind::Custom => "Custom",
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

impl From<serde_core::de::value::Error> for Error {
    fn from(value: serde_core::de::value::Error) -> Self {
        let message = value.to_string();
        Self::new(ErrorKind::DeserializationError(value), message)
    }
}

impl From<Box<pest::error::Error<duper::DuperRule>>> for Error {
    fn from(value: Box<pest::error::Error<duper::DuperRule>>) -> Self {
        let message = value.variant.message().into_owned();
        Self::new(ErrorKind::ParseError(value), message)
    }
}

impl From<DuperIdentifierTryFromError<'_>> for Error {
    fn from(value: DuperIdentifierTryFromError) -> Self {
        let message = value.to_string();
        Self::new(ErrorKind::SerializationError, message)
    }
}

impl From<DuperObjectTryFromError<'_>> for Error {
    fn from(value: DuperObjectTryFromError) -> Self {
        let message = value.to_string();
        Self::new(ErrorKind::SerializationError, message)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
