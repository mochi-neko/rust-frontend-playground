use serde::Deserialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Result type for the Firebase API.
pub(crate) type FirebaseResult<T> = Result<T, FirebaseError>;

/// Error type for the Firebase API.
#[derive(Debug, Error)]
pub(crate) enum FirebaseError {
    /// API error.
    #[error("Firebase API error: {0}")]
    ApiError(ApiErrorResponse),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

/// Error response payload for the auth endpoints.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ApiErrorResponse {
    #[serde(rename = "error")]
    pub(crate) error: Error,
}

impl Display for ApiErrorResponse {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

/// Error response payload for the auth endpoints.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Error {
    #[serde(rename = "errors")]
    pub(crate) errors: Vec<ErrorElement>,
    #[serde(rename = "code")]
    pub(crate) code: i64,
    #[serde(rename = "message")]
    pub(crate) message: String,
}

/// Error response payload for the auth endpoints.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ErrorElement {
    #[serde(rename = "domain")]
    pub(crate) domain: String,
    #[serde(rename = "reason")]
    pub(crate) reason: String,
    #[serde(rename = "message")]
    pub(crate) message: String,
}
