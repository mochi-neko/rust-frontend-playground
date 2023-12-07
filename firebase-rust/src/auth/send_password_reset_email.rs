/// Implements the send password reset email API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email)
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

/// Request body payload for the `sendOobCode` endpoint.
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
    /// Creates a new request body payload for the `sendOobCode` endpoint.
    pub fn new(email: String) -> Self {
        Self {
            request_type: "PASSWORD_RESET".to_string(),
            email,
        }
    }
}

/// Response payload for the `sendOobCode` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
#[derive(Deserialize)]
pub struct SendPasswordResetEmailResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
}

/// Common error codes for send password reset email API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
pub enum CommonErrorCode {
    /// There is no user record corresponding to this identifier. The user may have been deleted.
    EmailNotFound,
}

impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::EmailNotFound => "EMAIL_NOT_FOUND",
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        match self {
            | CommonErrorCode::EmailNotFound => {
                "There is no user record corresponding to this identifier. The user may have been deleted."
            },
        }
    }
}

impl TryFrom<&str> for CommonErrorCode {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            | "EMAIL_NOT_FOUND" => Ok(CommonErrorCode::EmailNotFound),
            | _ => Err(()),
        }
    }
}

/// Sends a password reset email to the given email address.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
///
/// ## Arguments
/// * `api_key` - Your Firebase project's API key.
/// * `request` - Request body payload for the `sendOobCode` endpoint.
///
/// ## Returns
/// The result with the response payload for the `sendOobCode` endpoint.
pub async fn send_password_reset_email(
    api_key: &String,
    request: SendPasswordResetEmailRequestBodyPayload,
) -> Result<SendPasswordResetEmailResponsePayload> {
    client::send_post::<
        SendPasswordResetEmailRequestBodyPayload,
        SendPasswordResetEmailResponsePayload,
    >("accounts:sendOobCode", api_key, request)
    .await
}
