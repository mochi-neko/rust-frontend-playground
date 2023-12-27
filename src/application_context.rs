use firebase_auth_rs::auth::{AuthConfig, AuthSession};

pub(crate) struct ApplicationContext {
    pub(crate) auth_config: AuthConfig,
    pub(crate) auth_session: Option<AuthSession>,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        Self {
            auth_config: AuthConfig::new(
                crate::generated::dotenv::FIREBASE_API_KEY.to_string(),
                None,
            ),
            auth_session: None,
        }
    }
}
