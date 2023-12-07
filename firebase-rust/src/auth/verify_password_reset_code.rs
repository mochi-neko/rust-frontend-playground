/// Implements the verify password reset code API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code)
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

/// Request body payload for the `resetPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
#[derive(Serialize)]
pub struct VerifyPasswordResetCodeRequestBodyPayload {
    /// The email action code sent to the user's email for resetting the password.
    #[serde(rename = "oobCode")]
    oob_code: String,
}

impl VerifyPasswordResetCodeRequestBodyPayload {
    /// Creates a new request body payload for the `resetPassword` endpoint.
    pub fn new(oob_code: String) -> Self {
        Self {
            oob_code,
        }
    }
}

/// Response payload for the `resetPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
#[derive(Deserialize)]
pub struct VerifyPasswordResetCodeResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Type of the email action code. Should be "PASSWORD_RESET".
    #[serde(rename = "requestType")]
    pub request_type: String,
}

/// Common error codes for verify password reset code API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
pub enum CommonErrorCode {
    /// Password sign-in is disabled for this project.
    OperationNotAllowed,
    /// The action code has expired.
    ExpiredOobCode,
    /// The action code is invalid. This can happen if the code is malformed, expired, or has already been used.
    InvalidOobCode,
}

impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::OperationNotAllowed => "OPERATION_NOT_ALLOWED",
            | CommonErrorCode::ExpiredOobCode => "EXPIRED_OOB_CODE",
            | CommonErrorCode::InvalidOobCode => "INVALID_OOB_CODE",
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        match self {
            | CommonErrorCode::OperationNotAllowed => {
                "Password sign-in is disabled for this project."
            },
            | CommonErrorCode::ExpiredOobCode => "The action code has expired.",
            | CommonErrorCode::InvalidOobCode => {
                "The action code is invalid. This can happen if the code is malformed, expired, or has already been used."
            },
        }
    }
}

impl TryFrom<&str> for CommonErrorCode {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            | "OPERATION_NOT_ALLOWED" => {
                Ok(CommonErrorCode::OperationNotAllowed)
            },
            | "EXPIRED_OOB_CODE" => Ok(CommonErrorCode::ExpiredOobCode),
            | "INVALID_OOB_CODE" => Ok(CommonErrorCode::InvalidOobCode),
            | _ => Err(()),
        }
    }
}

/// Verifies the password reset code sent to the user's email for resetting the password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
///
/// ## Arguments
/// * `api_key` - Your Firebase project's API key.
/// * `request` - Request body payload for the `resetPassword` endpoint.
///
/// ## Returns
/// The result of the response payload for the `resetPassword` endpoint.
pub async fn verify_password_reset_code(
    api_key: &String,
    request: VerifyPasswordResetCodeRequestBodyPayload,
) -> Result<VerifyPasswordResetCodeResponsePayload> {
    client::send_post::<
        VerifyPasswordResetCodeRequestBodyPayload,
        VerifyPasswordResetCodeResponsePayload,
    >(
        "accounts:resetPassword",
        api_key,
        request,
    )
    .await
}