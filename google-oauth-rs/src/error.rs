//! An error type for the Google OAuth API.

/// An error type for the Google OAuth API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request build failed: {0:?}")]
    RequestBuildError(reqwest::Error),
    #[error("HTTP error: {0:?}")]
    HttpError(reqwest::Error),
    #[error("Read response failed: {0:?}")]
    ReadResponseFailed(reqwest::Error),
    #[error("API error: ({status_code:?}) {response:?}")]
    ApiError {
        status_code: reqwest::StatusCode,
        response: String,
    },
    #[error("Response JSON error: {error:?} - {json:?}")]
    ResponseJsonError {
        error: serde_json::Error,
        json: String,
    },
}
