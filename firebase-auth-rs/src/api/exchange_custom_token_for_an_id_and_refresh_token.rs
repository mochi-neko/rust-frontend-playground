/// Implements the exchange custom token for an ID and refresh token API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the exchange custom token for an ID and refresh token API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
#[derive(Serialize)]
pub struct ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload {
    /// A Firebase Auth custom token from which to create an ID and refresh token pair.
    #[serde(rename = "token")]
    token: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload {
    /// Creates a new request body payload for the exchange custom token for an ID and refresh token API.
    pub fn new(token: String) -> Self {
        Self {
            token,
            return_secure_token: true,
        }
    }
}

/// Response payload for the exchange custom token for an ID and refresh token API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
#[derive(Deserialize)]
pub struct ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload {
    /// A Firebase Auth ID token generated from the provided custom token.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// A Firebase Auth refresh token generated from the provided custom token.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
}

/// Exchanges a custom token for an ID and refresh token.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn exchange_custom_token_for_an_id_and_refresh_token(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload,
) -> Result<ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload> {
    client::send_post::<
        ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload,
        ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload,
    >(
        client,
        "accounts:signInWithCustomToken",
        api_key,
        request_payload,
        None,
    )
    .await
}
