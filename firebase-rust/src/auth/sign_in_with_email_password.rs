use serde::{Deserialize, Serialize};

use super::auth_result::{ApiErrorResponse, FirebaseError, FirebaseResult};

/// Request body payload for the `signInWithEmailAndPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Serialize)]
struct SignInWithEmailPasswordRequestBodyPayload {
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

/// Response payload for the `signInWithEmailAndPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Deserialize)]
pub struct SignInWithEmailPasswordResponsePayload {
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    #[serde(rename = "localId")]
    pub local_id: String,
    #[serde(rename = "registered")]
    pub registered: bool,
}

/// Signs in a user with the given email address and password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
pub async fn sign_in_with_email_password(
    api_key: &String,
    email: String,
    password: String,
) -> FirebaseResult<SignInWithEmailPasswordResponsePayload> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        api_key
    );

    let request_payload = SignInWithEmailPasswordRequestBodyPayload {
        email,
        password,
        return_secure_token: true,
    };

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&request_payload)
        .send()
        .await
        .map_err(|error| {
            log::error!(
                "Failed to send request to sign in: {:?}",
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
                    "Failed to deserialize response to sign in: {:?}",
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
                    "Failed to deserialize error response to sign in: {:?}",
                    error
                );
                FirebaseError::JsonError(error)
            })?;

        log::error!(
            "Failed to sign in with bad status code ({}): {:?}",
            status_code,
            error_response
        );
        Err(FirebaseError::ApiError(error_response))
    }
}
