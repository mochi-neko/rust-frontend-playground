use crate::{firebase::auth, generated::dotenv};

use super::auth_context::AuthContext;

#[derive(Debug)]
pub(crate) struct SignUpInfo {
    pub(crate) email: String,
    pub(crate) password: String,
}

pub(crate) async fn sign_up(info: &SignUpInfo) -> anyhow::Result<AuthContext> {
    let response = auth::sign_up_with_email_and_password(
        &dotenv::FIREBASE_API_KEY.to_string(),
        info.email.clone(),
        info.password.clone(),
    )
    .await
    .map_err(|error| {
        log::error!("Sign up failed: {:?}", error);
        error
    })?;

    Ok(AuthContext {
        id_token: response.id_token,
        refresh_token: response.refresh_token,
    })
}
