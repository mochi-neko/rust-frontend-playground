/// Implements the send password reset email API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email)
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the send password reset email API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
#[derive(Serialize)]
pub struct SendPasswordResetEmailRequestBodyPayload {
    /// The kind of OOB code to return. Should be "PASSWORD_RESET" for password reset.
    #[serde(rename = "requestType")]
    request_type: String,
    /// User's email address.
    #[serde(rename = "email")]
    email: String,
}

impl SendPasswordResetEmailRequestBodyPayload {
    /// Creates a new request body payload for the send password reset email API.
    pub fn new(email: String) -> Self {
        Self {
            request_type: "PASSWORD_RESET".to_string(),
            email,
        }
    }
}

/// Response payload for the send password reset email API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
#[derive(Deserialize)]
pub struct SendPasswordResetEmailResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
}

/// Sends a password reset email to the given email address.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn send_password_reset_email(
    client: &reqwest::Client,
    api_key: &String,
    request: SendPasswordResetEmailRequestBodyPayload,
) -> Result<SendPasswordResetEmailResponsePayload> {
    client::send_post::<
        SendPasswordResetEmailRequestBodyPayload,
        SendPasswordResetEmailResponsePayload,
    >(
        client,
        "accounts:sendOobCode",
        api_key,
        request,
    )
    .await
}
