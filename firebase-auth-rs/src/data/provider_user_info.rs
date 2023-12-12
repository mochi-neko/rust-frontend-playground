use serde::Deserialize;

/// Provider user info.
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
#[derive(Deserialize)]
pub struct ProviderUserInfo {
    /// Provider ID.
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// Federated ID.
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    /// Display name.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Photo URL.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
}
