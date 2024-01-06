use fars::Config;
use fars::Session;

pub(crate) struct ApplicationContext {
    pub(crate) auth_config: Config,
    pub(crate) auth_session: Option<Session>,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        Self {
            auth_config: Config::new(
                crate::generated::dotenv::FIREBASE_API_KEY.to_string(),
            ),
            auth_session: None,
        }
    }
}
