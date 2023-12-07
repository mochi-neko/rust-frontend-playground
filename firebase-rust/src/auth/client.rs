/// Implements the Firebase Auth API client.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
use serde::{de::DeserializeOwned, Serialize};

use super::result::{ApiErrorResponse, FirebaseError, Result};

/// Sends a POST request to the Firebase Auth API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
///
/// ## Arguments
/// * `endpoint` - The endpoint to send the request to.
/// * `api_key` - The Firebase project's API key.
/// * `request_payload` - The request body payload.
///
/// ## Returns
/// The result with the response payload of the API.
pub(crate) async fn send_post<T, U>(
    endpoint: &str,
    api_key: &String,
    request_payload: T,
) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/{}?key={}",
        endpoint, api_key
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .json(&request_payload)
        .send()
        .await
        .map_err(|error| FirebaseError::HttpError(error))?;

    if response.status().is_success() {
        let response_payload = response
            .json::<U>()
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
