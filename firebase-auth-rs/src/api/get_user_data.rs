/// Implements the get user data API of the Firebase Auth.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info)
use serde::{Deserialize, Serialize};

use crate::{client, result::Result};

/// Request body payload for the get user data API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Serialize)]
pub struct GetUserDataRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl GetUserDataRequestBodyPayload {
    /// Creates a new request body payload for the get user data API.
    pub fn new(id_token: String) -> Self {
        Self {
            id_token,
        }
    }
}

/// Response payload for the get user data API.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Deserialize)]
pub struct GetUserDataResponsePayload {
    /// The account associated with the given Firebase ID token.
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

/// User information.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Deserialize, PartialEq)]
pub struct User {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
    /// Whether or not the account's email has been verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Vec<ProviderUserInfo>,
    /// The photo url of the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// Hash version of password.
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    /// The timestamp, in milliseconds, that the account password was last changed.
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: f64,
    /// The timestamp, in seconds, which marks a boundary, before which Firebase ID token are considered revoked.
    #[serde(rename = "validSince")]
    pub valid_since: String,
    /// Whether the account is disabled or not.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The timestamp, in milliseconds, that the account last logged in at.
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: String,
    /// The timestamp, in milliseconds, that the account was created at.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Whether the account is authenticated by the developer.
    #[serde(rename = "customAuth")]
    pub custom_auth: Option<bool>,
}

/// Provider user information.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Deserialize, PartialEq)]
pub struct ProviderUserInfo {
    /// The provider identifier.
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo url of the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// The federated identifier.
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
    /// The raw identifier of the account.
    #[serde(rename = "rawId")]
    pub raw_id: Option<String>,
    /// The screen name of the account.
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
}

/// Gets the user data.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
///
/// ## Arguments
/// * `client` - HTTP client.
/// * `api_key` - Your Firebase project's API key.
/// * `request_payload` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
pub async fn get_user_data(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: GetUserDataRequestBodyPayload,
) -> Result<GetUserDataResponsePayload> {
    client::send_post::<
        GetUserDataRequestBodyPayload,
        GetUserDataResponsePayload,
    >(client, "accounts:lookup", api_key, request_payload)
    .await
}
