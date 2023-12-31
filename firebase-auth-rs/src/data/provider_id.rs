//! Defines provider IDs.

use std::fmt::Display;

/// ID provider identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum ProviderId {
    // Password,
    Password,
    // Google OAuth.
    Google,
    // Facebook OAuth.
    Facebook,
    // Twitter OAuth.
    Twitter,
    // Github OAuth.
    Github,
    // Apple OAuth.
    Apple,
}

impl Display for ProviderId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ProviderId::Password => write!(f, "Password"),
            | ProviderId::Google => write!(f, "Google"),
            | ProviderId::Facebook => write!(f, "Facebook"),
            | ProviderId::Twitter => write!(f, "Twitter"),
            | ProviderId::Github => write!(f, "Github"),
            | ProviderId::Apple => write!(f, "Apple"),
        }
    }
}

impl ProviderId {
    /// Converts the provider ID to a string representation.
    ///
    /// ## Returns
    /// String representation of the provider ID.
    pub fn to_string(&self) -> String {
        match self {
            | ProviderId::Password => "password".to_string(),
            | ProviderId::Google => "google.com".to_string(),
            | ProviderId::Facebook => "facebook.com".to_string(),
            | ProviderId::Twitter => "twitter.com".to_string(),
            | ProviderId::Github => "github.com".to_string(),
            | ProviderId::Apple => "apple.com".to_string(),
        }
    }

    /// Tries to convert a string to a provider ID.
    ///
    /// ## Arguments
    /// - `string` - String to convert to a provider ID.
    ///
    /// ## Returns
    /// Provider ID if the string is a valid provider ID, otherwise an error.
    pub fn from_string(string: String) -> Result<Self, String> {
        match string.as_str() {
            | "password" => Ok(ProviderId::Password),
            | "google.com" => Ok(ProviderId::Google),
            | "facebook.com" => Ok(ProviderId::Facebook),
            | "twitter.com" => Ok(ProviderId::Twitter),
            | "github.com" => Ok(ProviderId::Github),
            | "apple.com" => Ok(ProviderId::Apple),
            | _ => Err(format!(
                "'{}' is not a valid provider ID",
                string
            )),
        }
    }
}
