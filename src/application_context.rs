use firebase_auth_rs::auth::AuthSession;

#[derive(Default)]
pub(crate) struct ApplicationContext {
    pub(crate) auth: Option<AuthSession>,
}
