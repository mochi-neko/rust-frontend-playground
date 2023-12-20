#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Auth error: {inner:?}")]
    FirebaseAuthError {
        inner: firebase_auth_rs::error::Error,
    },
}
