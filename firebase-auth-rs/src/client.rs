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
    client: &reqwest::Client,
    endpoint: &str,
    api_key: &String,
    request_payload: T,
    optional_headers: Option<reqwest::header::HeaderMap>,
) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/{}?key={}",
        endpoint, api_key
    );

    let mut builder = client
        .post(url)
        .json(&request_payload);

    if let Some(optional_headers) = optional_headers {
        builder = builder.headers(optional_headers);
    }

    let response = builder
        .send()
        .await
        .map_err(|error| FirebaseError::HttpError(error))?;

    let status_code = response.status();

    let response_text = response
        .text()
        .await
        .map_err(
            |error| FirebaseError::ReadResponseFailed {
                error,
            },
        )?;

    if status_code.is_success() {
        serde_json::from_str::<U>(&response_text).map_err(|error| {
            FirebaseError::ResponseJsonError {
                error,
                json: response_text,
            }
        })
    } else {
        let error_response =
            serde_json::from_str::<ApiErrorResponse>(&response_text).map_err(
                |error| FirebaseError::ResponseJsonError {
                    error,
                    json: response_text,
                },
            )?;

        let error_code = error_response
            .error
            .message
            .clone()
            .into();

        Err(FirebaseError::ApiError {
            status_code,
            error_code,
            response: error_response,
        })
    }
}
