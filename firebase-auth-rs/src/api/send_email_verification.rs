/// Implements the send email verification API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification)
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the send email verification API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
#[derive(Serialize)]
pub struct SendEmailVerificationRequestBodyPayload {
    /// The type of confirmation code to send. Should always be "VERIFY_EMAIL".
    #[serde(rename = "requestType")]
    request_type: String,
    /// The Firebase ID token of the user to verify.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl SendEmailVerificationRequestBodyPayload {
    /// Creates a new request body payload for the send email verification API.
    pub fn new(id_token: String) -> Self {
        Self {
            request_type: "VERIFY_EMAIL".to_string(),
            id_token,
        }
    }
}

/// Response payload for the the send email verification API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
#[derive(Deserialize)]
pub struct SendEmailVerificationResponsePayload {
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
}

/// Sends an email verification to the specified user.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn send_email_verification(
    client: &reqwest::Client,
    api_key: &String,
    request: SendEmailVerificationRequestBodyPayload,
) -> Result<SendEmailVerificationResponsePayload> {
    client::send_post::<
        SendEmailVerificationRequestBodyPayload,
        SendEmailVerificationResponsePayload,
    >(
        client,
        "accounts:sendOobCode",
        api_key,
        request,
    )
    .await
}
