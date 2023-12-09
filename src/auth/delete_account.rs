use firebase_rust::auth::delete_account::DeleteAccountRequestBodyPayload;

use crate::generated::dotenv;

pub(crate) async fn delete_account(id_token: String) -> anyhow::Result<()> {
    let _ = firebase_rust::auth::delete_account::delete_account(
        &dotenv::FIREBASE_API_KEY.to_string(),
        DeleteAccountRequestBodyPayload::new(id_token.clone()),
    )
    .await
    .map_err(|error| {
        log::error!("Sign in failed: {:?}", error);
        error
    })?;

    Ok(())
}
