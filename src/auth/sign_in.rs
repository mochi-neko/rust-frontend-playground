use crate::{firebase::auth, generated::dotenv};

use super::auth_context::AuthContext;

#[derive(Debug)]
pub(crate) struct SignInInfo {
    pub(crate) email: String,
    pub(crate) password: String,
}

pub(crate) async fn sign_in(info: &SignInInfo) -> anyhow::Result<AuthContext> {
    let response = auth::sign_in_with_email_and_password(
        &dotenv::FIREBASE_API_KEY.to_string(),
        info.email.clone(),
        info.password.clone(),
    )
    .await
    .map_err(|error| {
        log::error!("Sign in failed: {:?}", error);
        error
    })?;

    Ok(AuthContext {
        id_token: response.id_token,
        refresh_token: response.refresh_token,
    })
}
