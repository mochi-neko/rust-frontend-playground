use firebase_auth_rs::auth::Auth;

#[derive(Default)]
pub(crate) struct ApplicationContext {
    pub(crate) auth: Option<Auth>,
}
