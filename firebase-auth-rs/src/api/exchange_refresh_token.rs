/// Implements the exchange refresh token API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token)
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the exchange refresh token API.
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
    /// Creates a new request body payload for the exchange refresh token API.
    pub fn new(refresh_token: String) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            refresh_token,
        }
    }
}

/// Response payload for the exchange refresh token API.
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

/// Exchanges a refresh token for an access token and an ID token.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn exchange_refresh_token(
    client: &reqwest::Client,
    api_key: &String,
    request: ExchangeRefreshTokenRequestBodyPayload,
) -> Result<ExchangeRefreshTokenResponsePayload> {
    client::send_post::<
        ExchangeRefreshTokenRequestBodyPayload,
        ExchangeRefreshTokenResponsePayload,
    >(client, "token", api_key, request)
    .await
}
