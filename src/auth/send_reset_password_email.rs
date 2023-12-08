use firebase_rust::auth::send_password_reset_email::{
    send_password_reset_email, SendPasswordResetEmailRequestBodyPayload,
};

use crate::generated::dotenv;

pub(crate) async fn send_reset_password_email(
    email: String
) -> anyhow::Result<()> {
    let _ = send_password_reset_email(
        &dotenv::FIREBASE_API_KEY.to_string(),
        SendPasswordResetEmailRequestBodyPayload::new(email),
    )
    .await
    .map_err(|error| {
        log::error!(
            "Send reset password email failed: {:?}",
            error
        );
        error
    })?;

    Ok(())
}
