/// Implements the exchange refresh token API of the Firebase Auth.
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

/// Request body payload for the `token` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Serialize)]
pub struct ExchangeRefreshTokenRequestBodyPayload {
    /// The refresh token's grant type, always "refresh_token".
    #[serde(rename = "grant_type")]
    grant_type: String,
    /// A Firebase Auth refresh token.
    #[serde(rename = "refresh_token")]
    refresh_token: String,
}

impl ExchangeRefreshTokenRequestBodyPayload {
    /// Creates a new request body payload for the `token` endpoint.
    pub fn new(refresh_token: String) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            refresh_token,
        }
    }
}

/// Response payload for the `token` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Deserialize)]
pub struct ExchangeRefreshTokenResponsePayload {
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expires_in")]
    pub expires_in: String,
    /// The type of the refresh token, always "Bearer".
    #[serde(rename = "token_type")]
    pub token_type: String,
    /// The Firebase Auth refresh token provided in the request or a new refresh token.
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    /// A Firebase Auth ID token.
    #[serde(rename = "id_token")]
    pub id_token: String,
    /// The uid corresponding to the provided ID token.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// Your Firebase project ID.
    #[serde(rename = "project_id")]
    pub project_id: String,
}

/// Common error codes for exchange refresh token API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
pub enum CommonErrorCode {
    /// The user's credential is no longer valid. The user must sign in again.
    TokenExpired,
    /// The user account has been disabled by an administrator.
    UserDisabled,
    /// The user corresponding to the refresh token was not found. It is likely the user was deleted.
    UserNotFound,
    /// Invalid API key provided.
    InvalidApiKey,
    /// An invalid refresh token is provided.
    InvalidRefreshToken,
    /// Invalid JSON payload received, unknown field "refresh_tokens".
    InvalidJsonPayload,
    /// The grant type specified is invalid.
    InvalidGrantType,
    /// No refresh token provided.
    MissingRefreshToken,
}

impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::TokenExpired => "TOKEN_EXPIRED",
            | CommonErrorCode::UserDisabled => "USER_DISABLED",
            | CommonErrorCode::UserNotFound => "USER_NOT_FOUND",
            | CommonErrorCode::InvalidApiKey => "INVALID_API_KEY",
            | CommonErrorCode::InvalidRefreshToken => "INVALID_REFRESH_TOKEN",
            | CommonErrorCode::InvalidJsonPayload => "INVALID_JSON_PAYLOAD",
            | CommonErrorCode::InvalidGrantType => "INVALID_GRANT_TYPE",
            | CommonErrorCode::MissingRefreshToken => "MISSING_REFRESH_TOKEN",
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        match self {
            CommonErrorCode::TokenExpired => "The user's credential is no longer valid. The user must sign in again.",
            CommonErrorCode::UserDisabled => "The user account has been disabled by an administrator.",
            CommonErrorCode::UserNotFound => "The user corresponding to the refresh token was not found. It is likely the user was deleted.",
            CommonErrorCode::InvalidApiKey => "Invalid API key provided.",
            CommonErrorCode::InvalidRefreshToken => "An invalid refresh token is provided.",
            CommonErrorCode::InvalidJsonPayload => "Invalid JSON payload received, unknown field \"refresh_tokens\".",
            CommonErrorCode::InvalidGrantType => "The grant type specified is invalid.",
            CommonErrorCode::MissingRefreshToken => "No refresh token provided.",
        }
    }
}

impl TryFrom<String> for CommonErrorCode {
    type Error = ();

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            | "TOKEN_EXPIRED" => Ok(CommonErrorCode::TokenExpired),
            | "USER_DISABLED" => Ok(CommonErrorCode::UserDisabled),
            | "USER_NOT_FOUND" => Ok(CommonErrorCode::UserNotFound),
            | "INVALID_API_KEY" => Ok(CommonErrorCode::InvalidApiKey),
            | "INVALID_REFRESH_TOKEN" => {
                Ok(CommonErrorCode::InvalidRefreshToken)
            },
            | "INVALID_JSON_PAYLOAD" => Ok(CommonErrorCode::InvalidJsonPayload),
            | "INVALID_GRANT_TYPE" => Ok(CommonErrorCode::InvalidGrantType),
            | "MISSING_REFRESH_TOKEN" => {
                Ok(CommonErrorCode::MissingRefreshToken)
            },
            | _ => Err(()),
        }
    }
}

/// Exchanges a refresh token for an access token and an ID token.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
///
/// ## Arguments
/// * `api_key` - Your Firebase project's API key.
/// * `request` - Request body payload for the `token` endpoint.
///
/// ## Returns
/// The result with the response payload for the `token` endpoint.
pub async fn exchange_refresh_token(
    api_key: &String,
    request: ExchangeRefreshTokenRequestBodyPayload,
) -> Result<ExchangeRefreshTokenResponsePayload> {
    client::send_post::<
        ExchangeRefreshTokenRequestBodyPayload,
        ExchangeRefreshTokenResponsePayload,
    >("token", api_key, request)
    .await
}