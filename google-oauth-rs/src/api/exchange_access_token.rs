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
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: GrandType,
    pub redirect_uri: String,
}

#[derive(Deserialize)]
pub struct ExchangeAccessTokenResponsePayload {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
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
