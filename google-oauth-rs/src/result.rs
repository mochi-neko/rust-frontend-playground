//! A result type for the Google OAuth API.
use crate::error::Error;

/// A result type for the Google OAuth API.
pub type Result<T> = std::result::Result<T, Error>;
