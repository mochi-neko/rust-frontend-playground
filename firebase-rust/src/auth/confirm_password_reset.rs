/// Implements confirm password reset API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

/// Request body payload for the `resetPassword` endpoint.
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
    /// Creates a new request body payload for the `resetPassword` endpoint.
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

/// Response payload for the `resetPassword` endpoint.
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

/// Common error codes for confirm password reset API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
pub enum CommonErrorCode {
    /// Password sign-in is disabled for this project.
    OperationNotAllowed,
    /// The action code has expired.
    ExpiredOobCode,
    /// The action code is invalid. This can happen if the code is malformed, expired, or has already been used.
    InvalidOobCode,
    /// The user account has been disabled by an administrator.
    UserDisabled,
}

impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::OperationNotAllowed => "OPERATION_NOT_ALLOWED",
            | CommonErrorCode::ExpiredOobCode => "EXPIRED_OOB_CODE",
            | CommonErrorCode::InvalidOobCode => "INVALID_OOB_CODE",
            | CommonErrorCode::UserDisabled => "USER_DISABLED",
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
            | CommonErrorCode::UserDisabled => {
                "The user account has been disabled by an administrator."
            },
        }
    }
}

impl TryFrom<&str> for CommonErrorCode {
    type Error = ();

    fn try_from(code: &str) -> std::result::Result<Self, Self::Error> {
        match code {
            | "OPERATION_NOT_ALLOWED" => {
                Ok(CommonErrorCode::OperationNotAllowed)
            },
            | "EXPIRED_OOB_CODE" => Ok(CommonErrorCode::ExpiredOobCode),
            | "INVALID_OOB_CODE" => Ok(CommonErrorCode::InvalidOobCode),
            | "USER_DISABLED" => Ok(CommonErrorCode::UserDisabled),
            | _ => Err(()),
        }
    }
}

/// Confirms the password reset with the given code.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
///
/// ## Arguments
/// * `api_key` - Your Firebase project's API key.
/// * `request` - Request body payload for the `resetPassword` endpoint.
///
/// ## Returns
/// The result with the response payload for the `resetPassword` endpoint.
pub async fn confirm_password_reset(
    api_key: &String,
    request: ConfirmPasswordResetRequestBodyPayload,
) -> Result<ConfirmPasswordResetResponsePayload> {
    client::send_post::<
        ConfirmPasswordResetRequestBodyPayload,
        ConfirmPasswordResetResponsePayload,
    >(
        "accounts:resetPassword",
        api_key,
        request,
    )
    .await
}
