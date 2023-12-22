use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::result::Result;

#[derive(Serialize)]
pub enum GrandType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
}

#[derive(Serialize)]
pub struct ExchangeAccessTokenRequestParameter {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "grant_type")]
    pub grant_type: GrandType,
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
}

#[derive(Deserialize)]
pub enum TokenType {
    #[serde(rename = "Bearer")]
    Bearer,
}

#[derive(Deserialize)]
pub struct ExchangeAccessTokenResponsePayload {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "token_type")]
    pub token_type: TokenType,
    #[serde(rename = "id_token")]
    pub id_token: String,
}

pub async fn exchange_access_token(
    client: &reqwest::Client,
    request_parameter: ExchangeAccessTokenRequestParameter,
) -> Result<ExchangeAccessTokenResponsePayload> {
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&request_parameter)
        .send()
        .await
        .map_err(|error| Error::HttpError(error))?;

    let response_text = response
        .text()
        .await
        .map_err(|error| Error::ReadResponseFailed(error))?;

    let response_payload = serde_json::from_str::<
        ExchangeAccessTokenResponsePayload,
    >(&response_text)
    .map_err(|error| Error::ResponseJsonError {
        error,
        json: response_text,
    })?;

    Ok(response_payload)
}
