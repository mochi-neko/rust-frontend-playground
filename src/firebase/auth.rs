use serde::{Deserialize, Serialize};

use super::auth_result::{ApiErrorResponse, FirebaseError, FirebaseResult};

/// Request body payload for the `token` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Serialize)]
struct ExchangeRefreshTokenRequestBodyPayload {
    #[serde(rename = "grant_type")]
    grant_type: String,
    #[serde(rename = "refresh_token")]
    refresh_token: String,
}

/// Response payload for the `token` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct ExchangeRefreshTokenResponsePayload {
    #[serde(rename = "access_token")]
    access_token: String,
    #[serde(rename = "expires_in")]
    expires_in: String,
    #[serde(rename = "token_type")]
    token_type: String,
    #[serde(rename = "refresh_token")]
    refresh_token: String,
    #[serde(rename = "id_token")]
    id_token: String,
    #[serde(rename = "user_id")]
    user_id: String,
    #[serde(rename = "project_id")]
    project_id: String,
}

/// Exchanges a refresh token for an access token and an ID token.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
pub(crate) async fn exchange_refresh_token(
    api_key: &String,
    refresh_token: String,
) -> FirebaseResult<ExchangeRefreshTokenResponsePayload> {
    let url = format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        api_key
    );

    let request_payload = ExchangeRefreshTokenRequestBodyPayload {
        grant_type: "refresh_token".to_string(),
        refresh_token,
    };

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&request_payload)
        .send()
        .await
        .map_err(|error| {
            log::error!(
                "Failed to send request to exchange refresh token: {:?}",
                error
            );
            FirebaseError::Other(format!(
                "Failed to send request to exchange refresh token: {:?}",
                error
            ))
        })?;

    if response.status().is_success() {
        let response_payload = response
            .json::<ExchangeRefreshTokenResponsePayload>()
            .await
            .map_err(|error| {
                log::error!(
                    "Failed to deserialize response to exchange refresh token: {:?}",
                    error
                );
                FirebaseError::Other(format!(
                    "Failed to deserialize response to exchange refresh token: {:?}",
                    error
                ))
            })?;

        Ok(response_payload)
    } else {
        let status_code = response.status();
        let error_response = response
            .json::<ApiErrorResponse>()
            .await
            .map_err(|error| {
                log::error!(
                    "Failed to deserialize error response to exchange refresh token: {:?}",
                    error
                );
                FirebaseError::Other(format!(
                    "Failed to deserialize error response to exchange refresh token: {:?}",
                    error
                ))
            })?;

        log::error!(
            "Failed to exchange refresh token with bad status code ({}): {:?}",
            status_code,
            error_response
        );
        Err(FirebaseError::ApiError(error_response))
    }
}

/// Request body payload for the `signUp` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
#[derive(Serialize)]
struct SignUpWithEmailAndPasswordRequestBodyPayload {
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

/// Response payload for the `signUp` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct SignUpWithEmailAndPasswordResponsePayload {
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: String,
    #[serde(rename = "localId")]
    local_id: String,
}

/// Signs up a user with the given email address and password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
pub(crate) async fn sign_up_with_email_and_password(
    api_key: &String,
    email: String,
    password: String,
) -> FirebaseResult<SignUpWithEmailAndPasswordResponsePayload> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        api_key
    );

    let request_payload = SignUpWithEmailAndPasswordRequestBodyPayload {
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
                "Failed to send request to sign up: {:?}",
                error
            );
            FirebaseError::Other(format!(
                "Failed to send request to sign up: {:?}",
                error
            ))
        })?;

    if response.status().is_success() {
        let response_payload = response
            .json::<SignUpWithEmailAndPasswordResponsePayload>()
            .await
            .map_err(|error| {
                log::error!(
                    "Failed to deserialize response to sign up: {:?}",
                    error
                );
                FirebaseError::Other(format!(
                    "Failed to deserialize response to sign up: {:?}",
                    error
                ))
            })?;

        Ok(response_payload)
    } else {
        let status_code = response.status();
        let error_response = response
            .json::<ApiErrorResponse>()
            .await
            .map_err(|error| {
                log::error!(
                    "Failed to deserialize error response to sign up: {:?}",
                    error
                );
                FirebaseError::Other(format!(
                    "Failed to deserialize error response to sign up: {:?}",
                    error
                ))
            })?;

        log::error!(
            "Failed to sign up with bad status code ({}): {:?}",
            status_code,
            error_response
        );
        Err(FirebaseError::ApiError(error_response))
    }
}

/// Request body payload for the `signInWithEmailAndPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Serialize)]
struct SignInWithEmailAndPasswordRequestBodyPayload {
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

/// Response payload for the `signInWithEmailAndPassword` endpoint.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct SignInWithEmailAndPasswordResponsePayload {
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: String,
    #[serde(rename = "localId")]
    local_id: String,
    #[serde(rename = "registered")]
    registered: bool,
}

/// Signs in a user with the given email address and password.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
pub(crate) async fn sign_in_with_email_and_password(
    api_key: &String,
    email: String,
    password: String,
) -> FirebaseResult<SignInWithEmailAndPasswordResponsePayload> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        api_key
    );

    let request_payload = SignInWithEmailAndPasswordRequestBodyPayload {
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
            FirebaseError::Other(format!(
                "Failed to send request to sign in: {:?}",
                error
            ))
        })?;

    if response.status().is_success() {
        let response_payload = response
            .json::<SignInWithEmailAndPasswordResponsePayload>()
            .await
            .map_err(|error| {
                log::error!(
                    "Failed to deserialize response to sign in: {:?}",
                    error
                );
                FirebaseError::Other(format!(
                    "Failed to deserialize response to sign in: {:?}",
                    error
                ))
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
                FirebaseError::Other(format!(
                    "Failed to deserialize error response to sign in: {:?}",
                    error
                ))
            })?;

        log::error!(
            "Failed to sign in with bad status code ({}): {:?}",
            status_code,
            error_response
        );
        Err(FirebaseError::ApiError(error_response))
    }
}
