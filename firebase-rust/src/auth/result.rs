/// A result type for the Firebase Auth API.
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Result type for the Firebase Auth API.
pub type Result<T> = std::result::Result<T, FirebaseError>;

/// Error type for the Firebase API.
#[derive(Debug, Error)]
pub enum FirebaseError {
    /// API error.
    #[error("Firebase API error: {0}")]
    ApiError(ApiErrorResponse),
    /// HTTP error.
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    /// JSON error.
    #[error("JSON error: {0}")]
    JsonError(reqwest::Error),
    /// Validation error.
    #[error("Validation error: {0}")]
    ValidationError(String),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

/// Error response payload for the auth endpoints.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    #[serde(rename = "error")]
    pub error: Error,
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
pub struct Error {
    #[serde(rename = "errors")]
    pub errors: Vec<ErrorElement>,
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "message")]
    pub message: String,
}

/// Error response payload for the auth endpoints.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
pub struct ErrorElement {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "message")]
    pub message: String,
}
