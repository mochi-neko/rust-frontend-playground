//! Implements the get user data API of the Firebase Auth.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info)

use serde::{Deserialize, Serialize};

use crate::{
    client, data::provider_user_info::ProviderUserInfo, result::Result,
};

/// Request body payload for the get user data API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Serialize)]
pub struct GetUserDataRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl GetUserDataRequestBodyPayload {
    /// Creates a new request body payload for the get user data API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
    ///
    /// ## Arguments
    /// - `id_token` - The Firebase ID token of the account.
    pub fn new(id_token: String) -> Self {
        Self {
            id_token,
        }
    }
}

/// Response payload for the get user data API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Deserialize)]
pub struct GetUserDataResponsePayload {
    /// The account associated with the given Firebase ID token.
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

/// User information.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Deserialize, PartialEq)]
pub struct User {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Whether or not the account's email has been verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: Option<bool>,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Option<Vec<ProviderUserInfo>>,
    /// The photo url of the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// Hash version of password.
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    /// The timestamp, in milliseconds, that the account password was last changed.
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: Option<f64>,
    /// The timestamp, in seconds, which marks a boundary, before which Firebase ID token are considered revoked.
    #[serde(rename = "validSince")]
    pub valid_since: Option<String>,
    /// Whether the account is disabled or not.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The timestamp, in milliseconds, that the account last logged in at.
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: String,
    /// The timestamp, in milliseconds, that the account was created at.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The timestamp, in milliseconds, that the account was last refreshed at.
    #[serde(rename = "lastResheshAt")]
    pub last_refresh_at: Option<String>,
    /// Whether the account is authenticated by the developer.
    #[serde(rename = "customAuth")]
    pub custom_auth: Option<bool>,
}

/// Gets the user data.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
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
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
/// - USER_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
///
/// ## Example
/// ```
/// use firebase_auth_rs::api::get_user_data::{
///     GetUserDataRequestBodyPayload,
///     get_user_data,
/// };
///
/// let request_payload = GetUserDataRequestBodyPayload::new(
///     "id-token".to_string(),
/// );
///
/// let response_payload = get_user_data(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn get_user_data(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: GetUserDataRequestBodyPayload,
) -> Result<GetUserDataResponsePayload> {
    client::send_post::<
        GetUserDataRequestBodyPayload,
        GetUserDataResponsePayload,
    >(client, "accounts:lookup", api_key, request_payload, None,)
    .await
}
