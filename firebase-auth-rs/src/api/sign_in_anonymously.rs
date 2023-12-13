/// Implements the sign in anonymously API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the sign in anonymously API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
#[derive(Serialize)]
pub struct SignInAnonymouslyRequestBodyPayload {
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl SignInAnonymouslyRequestBodyPayload {
    /// Creates a new request body payload for the sign in anonymously API.
    pub fn new() -> Self {
        Self {
            return_secure_token: true,
        }
    }
}

/// Response payload for the sign in anonymously API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
#[derive(Deserialize)]
pub struct SignInAnonymouslyResponsePayload {
    /// A Firebase Auth ID token for the newly created user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// Since the user is anonymous, this should be empty.
    #[serde(rename = "email")]
    pub email: String,
    /// A Firebase Auth refresh token for the newly created user.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// The uid of the newly created user.
    #[serde(rename = "localId")]
    pub local_id: String,
}

/// Signs in a user anonymously.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn sign_in_anonymously(
    client: &reqwest::Client,
    api_key: &String,
    request: SignInAnonymouslyRequestBodyPayload,
) -> Result<SignInAnonymouslyResponsePayload> {
    client::send_post::<
        SignInAnonymouslyRequestBodyPayload,
        SignInAnonymouslyResponsePayload,
    >(
        client,
        "accounts:signUp",
        api_key,
        request,
    )
    .await
}
