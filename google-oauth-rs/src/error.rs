/// Error type for the Firebase API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request build failed: {0:?}")]
    RequestBuildError(reqwest::Error),
    #[error("HTTP error: {0:?}")]
    HttpError(reqwest::Error),
    #[error("Read response failed: {0:?}")]
    ReadResponseFailed(reqwest::Error),
    #[error("Response JSON error: {error:?} - {json:?}")]
    ResponseJsonError {
        error: serde_json::Error,
        json: String,
    },
}
