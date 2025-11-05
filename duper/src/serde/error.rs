use std::fmt::{self, Display};

use crate::{
    DuperIdentifierTryFromError, DuperObjectTryFromError, DuperParser, DuperTemporalTryFromError,
};

/// The kinds of errors that can happen during serialization and deserialization.
#[derive(Debug, Clone)]
pub enum DuperSerdeErrorKind {
    /// Parsing failed at the given [`chumsky`] spans.
    ParseError(Vec<chumsky::error::Rich<'static, char>>),
    /// Serialization failed with an unspecified error.
    SerializationError,
    /// Deserialization failed with the given reason.
    DeserializationError(serde_core::de::value::Error),
    /// An invalid value was provided.
    InvalidValue,
    /// Unspecified conditions.
    Custom,
}

impl Display for DuperSerdeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            DuperSerdeErrorKind::ParseError(_) => "ParseError",
            DuperSerdeErrorKind::SerializationError => "SerializationError",
            DuperSerdeErrorKind::DeserializationError(_) => "DeserializationError",
            DuperSerdeErrorKind::InvalidValue => "InvalidValue",
            DuperSerdeErrorKind::Custom => "Custom",
        })
    }
}

/// This type includes the error kind and message associated with the failure.
#[derive(Debug, Clone)]
pub struct ErrorImpl {
    pub kind: DuperSerdeErrorKind,
    pub message: String,
}

/// This type represents all possible errors that can occur when serializing or
/// deserializing Duper data.
#[derive(Debug, Clone)]
pub struct DuperSerdeError {
    pub inner: Box<ErrorImpl>,
}

impl DuperSerdeError {
    pub(crate) fn new(kind: DuperSerdeErrorKind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(ErrorImpl {
                kind,
                message: message.into(),
            }),
        }
    }

    pub(crate) fn custom(msg: impl Into<String> + Clone) -> Self {
        Self::new(DuperSerdeErrorKind::Custom, msg)
    }

    pub(crate) fn parse<'a>(src: &'a str, err_vec: Vec<chumsky::error::Rich<'a, char>>) -> Self {
        let message = DuperParser::prettify_error(src, &err_vec, None);
        Self::new(
            DuperSerdeErrorKind::ParseError(
                err_vec.into_iter().map(|err| err.into_owned()).collect(),
            ),
            message.unwrap_or_else(|_| "failed to generate parse error".into()),
        )
    }

    pub(crate) fn serialization(msg: impl Into<String>) -> Self {
        Self::new(DuperSerdeErrorKind::SerializationError, msg)
    }

    pub(crate) fn invalid_value(msg: impl Into<String>) -> Self {
        Self::new(DuperSerdeErrorKind::InvalidValue, msg)
    }
}

impl Display for DuperSerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.inner.kind, self.inner.message)
    }
}

impl std::error::Error for DuperSerdeError {}

impl serde_core::ser::Error for DuperSerdeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}

impl From<serde_core::de::value::Error> for DuperSerdeError {
    fn from(value: serde_core::de::value::Error) -> Self {
        let message = value.to_string();
        Self::new(DuperSerdeErrorKind::DeserializationError(value), message)
    }
}

impl From<DuperIdentifierTryFromError<'_>> for DuperSerdeError {
    fn from(value: DuperIdentifierTryFromError) -> Self {
        let message = value.to_string();
        Self::new(DuperSerdeErrorKind::SerializationError, message)
    }
}

impl From<DuperObjectTryFromError<'_>> for DuperSerdeError {
    fn from(value: DuperObjectTryFromError) -> Self {
        let message = value.to_string();
        Self::new(DuperSerdeErrorKind::SerializationError, message)
    }
}

impl From<DuperTemporalTryFromError<'_>> for DuperSerdeError {
    fn from(value: DuperTemporalTryFromError) -> Self {
        let message = value.to_string();
        Self::new(DuperSerdeErrorKind::SerializationError, message)
    }
}

pub type Result<T> = std::result::Result<T, DuperSerdeError>;
