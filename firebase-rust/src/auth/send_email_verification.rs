/// Implements the send email verification API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification)
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

/// Request body payload for the `sendOobCode` endpoint.
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
    /// Creates a new request body payload for the `sendOobCode` endpoint.
    pub fn new(id_token: String) -> Self {
        Self {
            request_type: "VERIFY_EMAIL".to_string(),
            id_token,
        }
    }
}

/// Response payload for the `sendOobCode` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
#[derive(Deserialize)]
pub struct SendEmailVerificationResponsePayload {
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
}

/// Common error codes for send email verification API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
pub enum CommonErrorCode {
    /// The user's credential is no longer valid. The user must sign in again.
    InvalidIdToken,
    /// There is no user record corresponding to this identifier. The user may have been deleted.
    UserNotFount,
}

// implement error code conversion
impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::InvalidIdToken => "INVALID_ID_TOKEN",
            | CommonErrorCode::UserNotFount => "USER_NOT_FOUND",
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        match self {
            | CommonErrorCode::InvalidIdToken => {
                "The user's credential is no longer valid. The user must sign in again."
            },
            | CommonErrorCode::UserNotFount => {
                "There is no user record corresponding to this identifier. The user may have been deleted."
            },
        }
    }
}

impl TryFrom<&str> for CommonErrorCode {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            | "INVALID_ID_TOKEN" => Ok(CommonErrorCode::InvalidIdToken),
            | "USER_NOT_FOUND" => Ok(CommonErrorCode::UserNotFount),
            | _ => Err(()),
        }
    }
}

/// Sends an email verification to the specified user.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
///
/// ## Arguments
/// * `api_key` - Your Firebase project API key.
/// * `request` - Request body payload for the `sendOobCode` endpoint.
///
/// ## Returns
/// The result with the response payload for the `sendOobCode` endpoint.
pub async fn send_email_verification(
    api_key: &String,
    request: SendEmailVerificationRequestBodyPayload,
) -> Result<SendEmailVerificationResponsePayload> {
    client::send_post::<
        SendEmailVerificationRequestBodyPayload,
        SendEmailVerificationResponsePayload,
    >("accounts:sendOobCode", api_key, request)
    .await
}
