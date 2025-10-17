use std::ops::Deref;

use axum::{
    extract::{FromRequest, OptionalFromRequest, Request},
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use serde_core::{Serialize, de::DeserializeOwned};
use serde_duper::ErrorKind;

#[derive(Debug)]
#[non_exhaustive]
pub enum DuperRejection {
    DuperDataError,
    DuperSyntaxError,
    MissingDuperContentType,
    InternalDuperError,
}

pub static DUPER_CONTENT_TYPE: &str = "application/duper";
pub static DUPER_ALT_CONTENT_TYPE: &str = "application/x-duper";

impl IntoResponse for DuperRejection {
    fn into_response(self) -> Response {
        match self {
            DuperRejection::DuperDataError | DuperRejection::DuperSyntaxError => {
                (StatusCode::BAD_REQUEST, "Failed to parse duper").into_response()
            }
            DuperRejection::MissingDuperContentType => (
                StatusCode::BAD_REQUEST,
                format!("Content-Type header must be {DUPER_CONTENT_TYPE}"),
            )
                .into_response(),
            DuperRejection::InternalDuperError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

pub struct Duper<T>(pub T);

impl<T> Duper<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DuperRejection> {
        let string = str::from_utf8(bytes).map_err(|_| DuperRejection::DuperDataError)?;
        Self::from_string(string)
    }

    pub fn from_string(string: &str) -> Result<Self, DuperRejection> {
        match serde_duper::from_string(string) {
            Ok(value) => Ok(Self(value)),
            Err(err) => match err.inner.kind {
                ErrorKind::ParseError(_) => Err(DuperRejection::DuperSyntaxError),
                ErrorKind::SerializationError
                | ErrorKind::DeserializationError(_)
                | ErrorKind::InvalidValue
                | ErrorKind::Custom => Err(DuperRejection::InternalDuperError),
            },
        }
    }
}

impl<T, S> FromRequest<S> for Duper<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = DuperRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(content_type) = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|content_type| content_type.to_str().ok())
            && (content_type == DUPER_CONTENT_TYPE || content_type == DUPER_ALT_CONTENT_TYPE)
        {
            let string = String::from_request(req, state)
                .await
                .map_err(|_| DuperRejection::DuperDataError)?;
            Self::from_string(&string)
        } else {
            Err(DuperRejection::MissingDuperContentType)
        }
    }
}

impl<T, S> OptionalFromRequest<S> for Duper<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = DuperRejection;

    async fn from_request(req: Request, state: &S) -> Result<Option<Self>, Self::Rejection> {
        let Some(content_type) = req.headers().get(CONTENT_TYPE) else {
            return Ok(None);
        };
        if let Ok(content_type) = content_type.to_str()
            && (content_type == DUPER_CONTENT_TYPE || content_type == DUPER_ALT_CONTENT_TYPE)
        {
            let string = String::from_request(req, state)
                .await
                .map_err(|_| DuperRejection::DuperDataError)?;
            Self::from_string(&string).map(Some)
        } else {
            Err(DuperRejection::MissingDuperContentType)
        }
    }
}

impl<T> IntoResponse for Duper<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        serde_duper::to_string(&self.0)
            .map(|string| {
                (
                    [(CONTENT_TYPE, HeaderValue::from_static(DUPER_CONTENT_TYPE))],
                    string,
                )
            })
            .map_err(|_| DuperRejection::InternalDuperError)
            .into_response()
    }
}

impl<T> Deref for Duper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Duper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
