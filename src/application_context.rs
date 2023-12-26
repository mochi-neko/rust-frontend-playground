use firebase_auth_rs::auth::Auth;

pub(crate) struct ApplicationContext {
    pub(crate) auth: Option<Auth>,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        Self {
            auth: None,
        }
    }
}
