use firebase_rust::auth::change_email::ChangeEmailRequestBodyPayload;
use firebase_rust::auth::change_password::ChangePasswordRequestBodyPayload;

use super::auth_context::AuthContext;
use crate::generated::dotenv;

pub(crate) async fn change_email(
    id_token: String,
    email: String,
) -> anyhow::Result<AuthContext> {
    let response = firebase_rust::auth::change_email::change_email(
        &dotenv::FIREBASE_API_KEY.to_string(),
        ChangeEmailRequestBodyPayload::new(id_token, email, true),
    )
    .await
    .map_err(|error| {
        log::error!("Sign up failed: {:?}", error);
        error
    })?;

    let id_token = response
        .id_token
        .ok_or(anyhow::anyhow!(
            "id_token is missing in the response"
        ))?;

    let refresh_token = response
        .refresh_token
        .ok_or(anyhow::anyhow!(
            "refresh_token is missing in the response"
        ))?;

    Ok(AuthContext {
        id_token,
        refresh_token,
    })
}

pub(crate) async fn change_password(
    id_token: String,
    password: String,
) -> anyhow::Result<AuthContext> {
    let response = firebase_rust::auth::change_password::change_password(
        &dotenv::FIREBASE_API_KEY.to_string(),
        ChangePasswordRequestBodyPayload::new(id_token, password, true),
    )
    .await
    .map_err(|error| {
        log::error!("Sign up failed: {:?}", error);
        error
    })?;

    let id_token = response
        .id_token
        .ok_or(anyhow::anyhow!(
            "id_token is missing in the response"
        ))?;

    let refresh_token = response
        .refresh_token
        .ok_or(anyhow::anyhow!(
            "refresh_token is missing in the response"
        ))?;

    Ok(AuthContext {
        id_token,
        refresh_token,
    })
}
