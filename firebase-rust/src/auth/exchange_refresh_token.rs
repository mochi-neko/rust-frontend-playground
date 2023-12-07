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
