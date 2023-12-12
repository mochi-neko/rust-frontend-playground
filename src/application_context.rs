use crate::auth_context::AuthContext;

#[derive(Clone)]
pub(crate) struct ApplicationContext {
    pub(crate) client: reqwest::Client,
    pub(crate) auth: Option<AuthContext>,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            auth: None,
        }
    }
}

impl ApplicationContext {
    pub(crate) fn set_auth(
        &mut self,
        auth: AuthContext,
    ) {
        self.auth = Some(auth);
    }
}
