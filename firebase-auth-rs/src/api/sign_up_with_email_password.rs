/// Implements the sign up with email password API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the sign up with email password API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
#[derive(Serialize)]
pub struct SignUpWithEmailPasswordRequestBodyPayload {
    /// The email for the user to create.
    #[serde(rename = "email")]
    email: String,
    /// The password for the user to create.
    #[serde(rename = "password")]
    password: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl SignUpWithEmailPasswordRequestBodyPayload {
    /// Creates a new request body payload for the sign up with email password API.
    pub fn new(
        email: String,
        password: String,
    ) -> Self {
        Self {
            email,
            password,
            return_secure_token: true,
        }
    }
}

/// Response payload for the sign up with email password API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
#[derive(Deserialize)]
pub struct SignUpWithEmailPasswordResponsePayload {
    /// A Firebase Auth ID token for the newly created user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// The email for the newly created user.
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

/// Signs up a user with the given email address and password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn sign_up_with_email_password(
    client: &reqwest::Client,
    api_key: &String,
    request: SignUpWithEmailPasswordRequestBodyPayload,
) -> Result<SignUpWithEmailPasswordResponsePayload> {
    client::send_post::<
        SignUpWithEmailPasswordRequestBodyPayload,
        SignUpWithEmailPasswordResponsePayload,
    >(
        client,
        "accounts:signUp",
        api_key,
        request,
    )
    .await
}
