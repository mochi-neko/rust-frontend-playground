/// Implements the confirm password reset API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the confirm password reset API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
#[derive(Serialize)]
pub struct ConfirmPasswordResetRequestBodyPayload {
    /// The email action code sent to the user's email for resetting the password.
    #[serde(rename = "oobCode")]
    oob_code: String,
    /// The new password.
    #[serde(rename = "newPassword")]
    new_password: String,
}

impl ConfirmPasswordResetRequestBodyPayload {
    /// Creates a new request body payload for the confirm password reset API.
    pub fn new(
        oob_code: String,
        new_password: String,
    ) -> Self {
        Self {
            oob_code,
            new_password,
        }
    }
}

/// Response payload for the confirm password reset API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
#[derive(Deserialize)]
pub struct ConfirmPasswordResetResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Type of the email action code. Should be "PASSWORD_RESET".
    #[serde(rename = "requestType")]
    pub request_type: String,
}

/// Confirms the password reset with the given code.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn confirm_password_reset(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: ConfirmPasswordResetRequestBodyPayload,
) -> Result<ConfirmPasswordResetResponsePayload> {
    client::send_post::<
        ConfirmPasswordResetRequestBodyPayload,
        ConfirmPasswordResetResponsePayload,
    >(
        client,
        "accounts:resetPassword",
        api_key,
        request_payload,
        None,
    )
    .await
}
