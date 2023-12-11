use firebase_rust::auth::send_email_verification::SendEmailVerificationRequestBodyPayload;

use crate::generated::dotenv;

pub(crate) async fn send_email_verification(
    id_token: String
) -> anyhow::Result<()> {
    let _ =
        firebase_rust::auth::send_email_verification::send_email_verification(
            &dotenv::FIREBASE_API_KEY.to_string(),
            SendEmailVerificationRequestBodyPayload::new(id_token),
        )
        .await
        .map_err(|error| {
            log::error!(
                "Send email verification failed: {:?}",
                error
            );
            error
        })?;

    Ok(())
}
