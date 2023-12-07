/// Implements the sign up API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
use serde::{Deserialize, Serialize};

use super::result::{ApiErrorResponse, FirebaseError, Result};

/// Request body payload for the `signUp` endpoint.
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
    /// Creates a new request body payload for the `signUp` endpoint.
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

/// Response payload for the `signUp` endpoint.
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

/// Common error codes for sign up API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
pub enum CommonErrorCode {
    /// There is no user record corresponding to this identifier. The user may have been deleted.
    EmailNotFound,
    /// The password is invalid or the user does not have a password.
    InvalidPassword,
    /// The user account has been disabled by an administrator.
    UserDisabled,
}

impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::EmailNotFound => "EMAIL_NOT_FOUND",
            | CommonErrorCode::InvalidPassword => "INVALID_PASSWORD",
            | CommonErrorCode::UserDisabled => "USER_DISABLED",
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        match self {
            | CommonErrorCode::EmailNotFound => {
                "There is no user record corresponding to this identifier. The user may have been deleted."
            },
            | CommonErrorCode::InvalidPassword => {
                "The password is invalid or the user does not have a password."
            },
            | CommonErrorCode::UserDisabled => {
                "The user account has been disabled by an administrator."
            },
        }
    }
}

impl TryFrom<String> for CommonErrorCode {
    type Error = ();

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            | "EMAIL_NOT_FOUND" => Ok(CommonErrorCode::EmailNotFound),
            | "INVALID_PASSWORD" => Ok(CommonErrorCode::InvalidPassword),
            | "USER_DISABLED" => Ok(CommonErrorCode::UserDisabled),
            | _ => Err(()),
        }
    }
}

/// Signs up a user with the given email address and password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
pub async fn sign_up_with_email_password(
    api_key: &String,
    request: SignUpWithEmailPasswordRequestBodyPayload,
) -> Result<SignUpWithEmailPasswordResponsePayload> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(|error| FirebaseError::HttpError(error))?;

    if response.status().is_success() {
        let response_payload = response
            .json::<SignUpWithEmailPasswordResponsePayload>()
            .await
            .map_err(|error| FirebaseError::JsonError(error))?;

        Ok(response_payload)
    } else {
        let error_response = response
            .json::<ApiErrorResponse>()
            .await
            .map_err(|error| FirebaseError::JsonError(error))?;

        Err(FirebaseError::ApiError(error_response))
    }
}
