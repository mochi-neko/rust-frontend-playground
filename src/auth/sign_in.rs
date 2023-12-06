use crate::{firebase::auth, generated::dotenv};

#[derive(Debug)]
pub(crate) struct SignInInfo {
    pub(crate) email: String,
    pub(crate) password: String,
}

pub(crate) async fn sign_in(info: &SignInInfo) -> anyhow::Result<()> {
    let _ = auth::sign_in_with_email_and_password(
        &dotenv::FIREBASE_API_KEY.to_string(),
        info.email.clone(),
        info.password.clone(),
    )
    .await
    .map_err(|error| {
        log::error!("Sign in failed: {:?}", error);
        error
    })?;

    Ok(())
}
