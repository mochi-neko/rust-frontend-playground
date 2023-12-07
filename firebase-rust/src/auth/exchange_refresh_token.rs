/// Implements the exchange refresh token API of the Firebase Auth.
use serde::{Deserialize, Serialize};

use super::result::{ApiErrorResponse, FirebaseError, Result};

/// Request body payload for the `token` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Serialize)]
struct ExchangeRefreshTokenRequestBodyPayload {
    #[serde(rename = "grant_type")]
    grant_type: String,
    #[serde(rename = "refresh_token")]
    refresh_token: String,
}

/// Response payload for the `token` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Deserialize)]
pub struct ExchangeRefreshTokenResponsePayload {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "id_token")]
    pub id_token: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
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
pub async fn exchange_refresh_token(
    api_key: &String,
    refresh_token: String,
) -> Result<ExchangeRefreshTokenResponsePayload> {
    let url = format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        api_key
    );

    let request_payload = ExchangeRefreshTokenRequestBodyPayload {
        grant_type: "refresh_token".to_string(),
        refresh_token,
    };

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&request_payload)
        .send()
        .await
        .map_err(|error| {
            log::error!(
                "[Firebase] Failed to send request to exchange refresh token: {:?}",
                error
            );
            FirebaseError::HttpError(error)
        })?;

    if response.status().is_success() {
        let response_payload = response
            .json::<ExchangeRefreshTokenResponsePayload>()
            .await
            .map_err(|error| {
                log::error!(
                    "[Firebase] Failed to deserialize response to exchange refresh token: {:?}",
                    error
                );
                FirebaseError::JsonError(error)
            })?;

        Ok(response_payload)
    } else {
        let status_code = response.status();
        let error_response = response
            .json::<ApiErrorResponse>()
            .await
            .map_err(|error| {
                log::error!(
                    "[Firebase] Failed to deserialize error response to exchange refresh token: {:?}",
                    error
                );
                FirebaseError::JsonError(error)
            })?;

        log::error!(
            "[Firebase] Failed to exchange refresh token with bad status code ({}): {:?}",
            status_code,
            error_response
        );
        Err(FirebaseError::ApiError(error_response))
    }
}
