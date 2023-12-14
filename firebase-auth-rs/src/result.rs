/// A result type for the Firebase Auth API.
use crate::error::Error;

/// Result type for the Firebase Auth API.
pub type Result<T> = std::result::Result<T, Error>;
