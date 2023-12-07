/// Implements the sign in API of Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
use serde::{Deserialize, Serialize};

use super::{client, result::Result};

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
///
/// ## Arguments
/// * `api_key` - Your Firebase project's API key.
/// * `request` - Request body payload for the `signInWithEmailAndPassword` endpoint.
///
/// ## Returns
/// The result with the response payload for the `signInWithEmailAndPassword` endpoint.
pub async fn sign_in_with_email_password(
    api_key: &String,
    request: SignInWithEmailPasswordRequestBodyPayload,
) -> Result<SignInWithEmailPasswordResponsePayload> {
    client::send_post::<
        SignInWithEmailPasswordRequestBodyPayload,
        SignInWithEmailPasswordResponsePayload,
    >(
        "accounts:signInWithPassword",
        api_key,
        request,
    )
    .await
}
