#[derive(Clone)]
pub(crate) struct AuthContext {
    pub(crate) id_token: String,
    pub(crate) refresh_token: String,
}
