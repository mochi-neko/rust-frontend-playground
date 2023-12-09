/// Implements the delete account API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account)
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

/// Request body payload for the `deleteAccount` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
#[derive(Serialize)]
pub struct DeleteAccountRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl DeleteAccountRequestBodyPayload {
    /// Creates a new request body payload for the `deleteAccount` endpoint.
    pub fn new(id_token: String) -> Self {
        Self {
            id_token,
        }
    }
}

/// Response payload for the `deleteAccount` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
#[derive(Deserialize)]
pub struct DeleteAccountResponsePayload {}

/// Deletes the account of the user specified by the given ID token.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
pub async fn delete_account(
    api_key: &String,
    request: DeleteAccountRequestBodyPayload,
) -> Result<DeleteAccountResponsePayload> {
    client::send_post::<
        DeleteAccountRequestBodyPayload,
        DeleteAccountResponsePayload,
    >("accounts:delete", api_key, request)
    .await
}
