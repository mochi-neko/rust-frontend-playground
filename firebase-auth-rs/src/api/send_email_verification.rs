/// Implements the send email verification API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification)
use serde::{Deserialize, Serialize};

use crate::{client, error::Error, result::Result};

/// Request body payload for the send email verification API.
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
    /// Creates a new request body payload for the send email verification API.
    pub fn new(id_token: String) -> Self {
        Self {
            request_type: "VERIFY_EMAIL".to_string(),
            id_token,
        }
    }
}

/// Response payload for the the send email verification API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
#[derive(Deserialize)]
pub struct SendEmailVerificationResponsePayload {
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
}

/// Sends an email verification to the specified user.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `api_key` - Your Firebase project's API key.
/// - `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
///
/// ## Common error codes
/// - INVALID_ID_TOKEN: The user's credential is no longer valid. The user must sign in again.
/// - USER_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
pub async fn send_email_verification(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: SendEmailVerificationRequestBodyPayload,
    locale: Option<String>,
) -> Result<SendEmailVerificationResponsePayload> {
    let optional_headers = match locale {
        | Some(locale) => {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "X-Firebase-Locale",
                reqwest::header::HeaderValue::from_str(&locale).map_err(
                    |error| Error::HeaderError {
                        key: "X-Firebase-Locale",
                        error: error,
                    },
                )?,
            );
            Some(headers)
        },
        | None => None,
    };

    client::send_post::<
        SendEmailVerificationRequestBodyPayload,
        SendEmailVerificationResponsePayload,
    >(
        client,
        "accounts:sendOobCode",
        api_key,
        request_payload,
        optional_headers,
    )
    .await
}
