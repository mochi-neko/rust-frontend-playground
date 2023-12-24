//! Exchange authorization code for access token.
//! See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#exchange-authorization-code).
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::result::Result;

/// Grant type defined by OAuth 2.0 specification.
#[derive(Serialize)]
pub enum GrandType {
    /// Authorization code.
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
}

/// Request parameters for the exchange authorization code for access token API.
/// See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#exchange-authorization-code).
#[derive(Serialize)]
pub struct ExchangeAccessTokenRequestParameters {
    /// The client ID obtained from the API Console Credentials page.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The client secret obtained from the API Console Credentials page.
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    /// The authorization code returned from the initial request.
    #[serde(rename = "code")]
    pub code: String,
    /// As defined in the OAuth 2.0 specification, this field's value must be set to authorization_code.
    #[serde(rename = "grant_type")]
    pub grant_type: GrandType,
    /// One of the redirect URIs listed for your project in the API Console Credentials page for the given client_id.
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
}

/// Returned token type.
#[derive(Deserialize)]
pub enum TokenType {
    /// Bearer token.
    #[serde(rename = "Bearer")]
    Bearer,
}

/// Response payload for the exchange authorization code for access token API.
/// See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#exchange-authorization-code).
#[derive(Deserialize)]
pub struct ExchangeAccessTokenResponsePayload {
    /// The token that your application sends to authorize a Google API request.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// The remaining lifetime of the access token in seconds.
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
    /// A token that you can use to obtain a new access token.
    /// Refresh tokens are valid until the user revokes access.
    /// Again, this field is only present in this response if you set the access_type parameter to offline in the initial request to Google's authorization server.
    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,
    /// The scopes of access granted by the access_token expressed as a list of space-delimited, case-sensitive strings.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The type of token returned. At this time, this field's value is always set to Bearer.
    #[serde(rename = "token_type")]
    pub token_type: TokenType,
    /// The ID token as OpenID.
    #[serde(rename = "id_token")]
    pub id_token: String,
}

/// Exchanges a refresh token for an access token and an ID token.
/// See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#exchange-authorization-code).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `request_parameter` - Request parameters.
///
/// ## Returns
/// Result with a response payload.
pub async fn exchange_access_token(
    client: &reqwest::Client,
    request_parameter: ExchangeAccessTokenRequestParameters,
) -> Result<ExchangeAccessTokenResponsePayload> {
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&request_parameter)
        .send()
        .await
        .map_err(|error| Error::HttpError(error))?;

    let status_code = response.status();

    let response_text = response
        .text()
        .await
        .map_err(|error| Error::ReadResponseFailed(error))?;

    if status_code.is_success() {
        let response_payload = serde_json::from_str::<
            ExchangeAccessTokenResponsePayload,
        >(&response_text)
        .map_err(|error| Error::ResponseJsonError {
            error,
            json: response_text,
        })?;

        Ok(response_payload)
    } else {
        Err(Error::ApiError {
            status_code: status_code,
            response: response_text,
        })
    }
}
