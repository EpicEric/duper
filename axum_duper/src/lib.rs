#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//! # Axum Duper
//!
//! Duper extractor / response for [`axum`].
//!
//! This crate provides the [`Duper`] struct, which can be used to extract typed
//! information from request's body, or to serialize a structured response.
//!
//! Under the hood, it wraps [`serde_duper`].

use std::ops::Deref;

use axum::{
    extract::{FromRequest, OptionalFromRequest, Request},
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use duper::serde::error::DuperSerdeErrorKind;
use serde_core::{Serialize, de::DeserializeOwned};

pub static DUPER_CONTENT_TYPE: &str = "application/duper";
pub static DUPER_ALT_CONTENT_TYPE: &str = "application/x-duper";

/// Rejection used for [`Duper`].
///
/// Contains one variant for each way the [`Duper`] extractor can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DuperRejection {
    DuperDataError,
    DuperSyntaxError,
    MissingDuperContentType,
    InternalDuperError,
}

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

/// Duper extractor / response.
///
/// When used as an extractor, it can deserialize request bodies into some type
/// that implements [`DeserializeOwned`]. The request will be
/// rejected (and a [`DuperRejection`] will be returned) if:
///
/// - The request doesn’t have a `Content-Type: application/duper` or
///   `Content-Type: application/x-duper` header.
/// - The body doesn’t contain a syntactically valid Duper value.
/// - The body contains a syntactically valid Duper value, but it couldn’t be
///   deserialized into the target type.
/// - Buffering the request body fails.
///
/// Since parsing Duper values requires consuming the request body, the `Duper`
/// extractor must be *last* if there are multiple extractors in a handler.
///
/// # Extractor example
///
/// ```rust, no_run
/// use axum::{Router, routing::post};
/// use axum_duper::Duper;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// async fn create_user(Duper(payload): Duper<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
///
/// When used as a response, it can serialize any type that implements
/// [`Serialize`] to `Duper`, and will automatically set the
/// `Content-Type: application/duper` header.
///
/// If the [`Serialize`] implementation decides to fail, or if a map with
/// non-string keys is used, a 500 response will be issued, whose body is
/// the error message in UTF-8.
///
/// # Response example
///
/// ```
/// use axum::{Router, routing::get, extract::Path};
/// use axum_duper::Duper;
/// use serde::Serialize;
/// use uuid::Uuid;
///
/// #[derive(Serialize)]
/// struct User {
///     id: Uuid,
///     username: String,
/// }
///
/// async fn get_user(Path(user_id) : Path<Uuid>) -> Duper<User> {
///     let user = find_user(user_id).await;
///     Duper(user)
/// }
///
/// async fn find_user(user_id: Uuid) -> User {
///     // ...
///     # unimplemented!()
/// }
///
/// let app = Router::new().route("/users/{id}", get(get_user));
/// # let _: Router = app;
/// ```
pub struct Duper<T>(pub T);

impl<T> Duper<T>
where
    T: DeserializeOwned,
{
    /// Construct a `Duper<T>` from a byte slice. Most users should prefer to
    /// use the `FromRequest` impl, but special cases may require first
    /// extracting a `Request` into `Bytes`, then optionally constructing a
    /// `Duper<T>`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DuperRejection> {
        let string = str::from_utf8(bytes).map_err(|_| DuperRejection::DuperDataError)?;
        Self::from_string(string)
    }

    /// Construct a `Duper<T>` from a str slice. Most users should prefer to
    /// use the `FromRequest` impl, but special cases may require first
    /// extracting a `Request` into `String`, then optionally constructing a
    /// `Duper<T>`.
    pub fn from_string(string: &str) -> Result<Self, DuperRejection> {
        match duper::serde::de::from_string(string) {
            Ok(value) => Ok(Self(value)),
            Err(err) => match err.inner.kind {
                DuperSerdeErrorKind::ParseError(_) => Err(DuperRejection::DuperSyntaxError),
                DuperSerdeErrorKind::SerializationError
                | DuperSerdeErrorKind::DeserializationError(_)
                | DuperSerdeErrorKind::InvalidValue
                | DuperSerdeErrorKind::Custom => Err(DuperRejection::InternalDuperError),
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
        duper::serde::ser::to_string(&self.0)
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
