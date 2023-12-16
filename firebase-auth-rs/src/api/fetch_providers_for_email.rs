/// Implements the fetch providers for email API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the fetch providers for email API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
#[derive(Serialize)]
pub struct FetchProvidersForEmailRequestBodyPayload {
    /// User's email address
    #[serde(rename = "identifier")]
    identifier: String,
    /// The URI to which the IDP redirects the user back. For this use case, this is just the current URL.
    #[serde(rename = "continueUri")]
    continue_uri: String,
}

impl FetchProvidersForEmailRequestBodyPayload {
    /// Creates a new request body payload for the fetch providers for email API.
    pub fn new(
        identifier: String,
        continue_uri: String,
    ) -> Self {
        Self {
            identifier,
            continue_uri,
        }
    }
}

/// Response payload for the fetch providers for email API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
#[derive(Deserialize)]
pub struct FetchProvidersForEmailResponsePayload {
    /// The list of providers that the user has previously signed in with.
    #[serde(rename = "allProviders")]
    pub all_providers: Vec<String>,
    /// Whether the email address is for an existing account.
    #[serde(rename = "registered")]
    pub registered: bool,
}

/// Fetches the list of sign-in methods available for the specified email address.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `api_key` - Your Firebase project's API key.
/// - `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
///
/// ## Common error codes
/// - INVALID_EMAIL: The email address is badly formatted.
pub async fn fetch_providers_for_email(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: FetchProvidersForEmailRequestBodyPayload,
) -> Result<FetchProvidersForEmailResponsePayload> {
    client::send_post::<
        FetchProvidersForEmailRequestBodyPayload,
        FetchProvidersForEmailResponsePayload,
    >(
        client,
        "accounts:createAuthUri",
        api_key,
        request_payload,
        None,
    )
    .await
}
