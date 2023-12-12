/// Implements the delete account API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account)
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the delete account API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
#[derive(Serialize)]
pub struct DeleteAccountRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl DeleteAccountRequestBodyPayload {
    /// Creates a new request body payload for the delete account API.
    pub fn new(id_token: String) -> Self {
        Self {
            id_token,
        }
    }
}

/// Response payload for the delete account API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
#[derive(Deserialize)]
pub struct DeleteAccountResponsePayload {}

/// Deletes the account of the user specified by the given ID token.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn delete_account(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: DeleteAccountRequestBodyPayload,
) -> Result<DeleteAccountResponsePayload> {
    client::send_post::<
        DeleteAccountRequestBodyPayload,
        DeleteAccountResponsePayload,
    >(
        client,
        "accounts:delete",
        api_key,
        request_payload,
    )
    .await
}