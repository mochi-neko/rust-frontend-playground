/// Implements the sign in API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
use serde::{Deserialize, Serialize};

use super::result::{ApiErrorResponse, FirebaseError, Result};

/// Request body payload for the `signInWithEmailAndPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Serialize)]
pub struct SignInWithEmailPasswordRequestBodyPayload {
    /// The email the user is sign in with.
    #[serde(rename = "email")]
    email: String,
    /// The password for the account.
    #[serde(rename = "password")]
    password: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl SignInWithEmailPasswordRequestBodyPayload {
    /// Creates a new request body payload for the `signInWithEmailAndPassword` endpoint.
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

/// Response payload for the `signInWithEmailAndPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Deserialize)]
pub struct SignInWithEmailPasswordResponsePayload {
    /// A Firebase Auth ID token for the authenticated user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// The email for the authenticated user.
    #[serde(rename = "email")]
    pub email: String,
    /// A Firebase Auth refresh token for the authenticated user.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// The uid of the authenticated user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// Whether the email is for an existing account.
    #[serde(rename = "registered")]
    pub registered: bool,
}

/// Common error codes for sign in API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
pub enum CommonErrorCode {
    /// The email address is already in use by another account.
    EmailExists,
    /// Password sign-in is disabled for this project.
    OperationNotAllowed,
    /// We have blocked all requests from this device due to unusual activity. Try again later.
    TooManyAttemptsTryLater,
}

impl CommonErrorCode {
    /// Error code as string.
    pub fn code(&self) -> &str {
        match self {
            | CommonErrorCode::EmailExists => "EMAIL_EXISTS",
            | CommonErrorCode::OperationNotAllowed => "OPERATION_NOT_ALLOWED",
            | CommonErrorCode::TooManyAttemptsTryLater => {
                "TOO_MANY_ATTEMPTS_TRY_LATER"
            },
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        match self {
            CommonErrorCode::EmailExists => "The email address is already in use by another account.",
            CommonErrorCode::OperationNotAllowed => "Password sign-in is disabled for this project.",
            CommonErrorCode::TooManyAttemptsTryLater => "We have blocked all requests from this device due to unusual activity. Try again later.",
        }
    }
}

impl TryFrom<String> for CommonErrorCode {
    type Error = ();

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            | "EMAIL_EXISTS" => Ok(CommonErrorCode::EmailExists),
            | "OPERATION_NOT_ALLOWED" => {
                Ok(CommonErrorCode::OperationNotAllowed)
            },
            | "TOO_MANY_ATTEMPTS_TRY_LATER" => {
                Ok(CommonErrorCode::TooManyAttemptsTryLater)
            },
            | _ => Err(()),
        }
    }
}

/// Signs in a user with the given email address and password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
pub async fn sign_in_with_email_password(
    api_key: &String,
    request: SignInWithEmailPasswordRequestBodyPayload,
) -> Result<SignInWithEmailPasswordResponsePayload> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(|error| {
            log::error!(
                "[Firebase] Failed to send request to sign in: {:?}",
                error
            );
            FirebaseError::HttpError(error)
        })?;

    if response.status().is_success() {
        let response_payload = response
            .json::<SignInWithEmailPasswordResponsePayload>()
            .await
            .map_err(|error| {
                log::error!(
                    "[Firebase] Failed to deserialize response to sign in: {:?}",
                    error
                );
                FirebaseError::JsonError(error)
            })?;

        Ok(response_payload)
    } else {
        let status_code = response.status();
        let error_response = response
            .json::<ApiErrorResponse>()
            .await
            .map_err(|error| {
                log::error!(
                    "[Firebase] Failed to deserialize error response to sign in: {:?}",
                    error
                );
                FirebaseError::JsonError(error)
            })?;

        log::error!(
            "[Firebase] Failed to sign in with bad status code ({}): {:?}",
            status_code,
            error_response
        );
        Err(FirebaseError::ApiError(error_response))
    }
}
