/// Error type for the Firebase API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request build failed: {0:?}")]
    RequestBuildError(reqwest::Error),
}
