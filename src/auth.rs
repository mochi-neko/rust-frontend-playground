use firebase_auth_rs::api::{
    change_email::ChangeEmailRequestBodyPayload,
    change_password::ChangePasswordRequestBodyPayload,
    delete_account::DeleteAccountRequestBodyPayload,
    get_user_data::{
        GetUserDataRequestBodyPayload, GetUserDataResponsePayload,
    },
    send_email_verification::SendEmailVerificationRequestBodyPayload,
    send_password_reset_email::SendPasswordResetEmailRequestBodyPayload,
    sign_in_with_email_password::SignInWithEmailPasswordRequestBodyPayload,
    sign_up_with_email_password::SignUpWithEmailPasswordRequestBodyPayload,
    update_profile::{DeleteAttribute, UpdateProfileRequestBodyPayload},
};

use crate::auth_context::AuthContext;
use crate::generated::dotenv;

pub(crate) async fn sign_up(
    client: &reqwest::Client,
    email: String,
    password: String,
) -> anyhow::Result<AuthContext> {
    let response = firebase_auth_rs::api::sign_up_with_email_password::sign_up_with_email_password(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        SignUpWithEmailPasswordRequestBodyPayload::new(
            email.clone(),
            password.clone(),
        ),
    )
    .await
    .map_err(|error| {
        log::error!("[Auth] Sign up failed: {:?}", error);
        error
    })?;

    Ok(AuthContext {
        id_token: response.id_token,
        refresh_token: response.refresh_token,
    })
}

pub(crate) async fn sign_in(
    client: &reqwest::Client,
    email: String,
    password: String,
) -> anyhow::Result<AuthContext> {
    let response = firebase_auth_rs::api::sign_in_with_email_password::sign_in_with_email_password(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        SignInWithEmailPasswordRequestBodyPayload::new(
            email.clone(),
            password.clone(),
        ),
    )
    .await
    .map_err(|error| {
        log::error!("[Auth] Sign in failed: {:?}", error);
        error
    })?;

    Ok(AuthContext {
        id_token: response.id_token,
        refresh_token: response.refresh_token,
    })
}

pub(crate) async fn delete_account(
    client: &reqwest::Client,
    context: &AuthContext,
) -> anyhow::Result<()> {
    let _ = firebase_auth_rs::api::delete_account::delete_account(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        DeleteAccountRequestBodyPayload::new(context.id_token.clone()),
    )
    .await
    .map_err(|error| {
        log::error!(
            "[Auth] Delete account failed: {:?}",
            error
        );
        error
    })?;

    Ok(())
}

pub(crate) async fn get_user_data(
    client: &reqwest::Client,
    context: &AuthContext,
) -> anyhow::Result<GetUserDataResponsePayload> {
    firebase_auth_rs::api::get_user_data::get_user_data(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        GetUserDataRequestBodyPayload::new(context.id_token.clone()),
    )
    .await
    .map_err(|error| {
        log::error!(
            "[Auth] Get user data failed: {:?}",
            error
        );
        anyhow::Error::new(error)
    })
}

pub(crate) async fn send_email_verification(
    client: &reqwest::Client,
    context: &AuthContext,
) -> anyhow::Result<()> {
    let _ =
        firebase_auth_rs::api::send_email_verification::send_email_verification(
            client,
            &dotenv::FIREBASE_API_KEY.to_string(),
            SendEmailVerificationRequestBodyPayload::new(context.id_token.clone()),
        )
        .await
        .map_err(|error| {
            log::error!(
                "[Auth] Send email verification failed: {:?}",
                error
            );
            error
        })?;

    Ok(())
}

pub(crate) async fn send_reset_password_email(
    client: &reqwest::Client,
    email: String,
) -> anyhow::Result<()> {
    let _ = firebase_auth_rs::api::send_password_reset_email::send_password_reset_email(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        SendPasswordResetEmailRequestBodyPayload::new(email),
    )
    .await
    .map_err(|error| {
        log::error!(
            "[Auth] Send reset password email failed: {:?}",
            error
        );
        error
    })?;

    Ok(())
}

pub(crate) async fn change_email(
    client: &reqwest::Client,
    context: &AuthContext,
    email: String,
) -> anyhow::Result<AuthContext> {
    let response = firebase_auth_rs::api::change_email::change_email(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        ChangeEmailRequestBodyPayload::new(
            context.id_token.clone(),
            email,
            true,
        ),
    )
    .await
    .map_err(|error| {
        log::error!(
            "[Auth] Change email failed: {:?}",
            error
        );
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
    client: &reqwest::Client,
    context: &AuthContext,
    password: String,
) -> anyhow::Result<AuthContext> {
    let response = firebase_auth_rs::api::change_password::change_password(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        ChangePasswordRequestBodyPayload::new(
            context.id_token.clone(),
            password,
            true,
        ),
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

pub(crate) async fn update_profile(
    client: &reqwest::Client,
    context: &AuthContext,
    display_name: String,
    photo_url: String,
    delete_attribute: Vec<DeleteAttribute>,
) -> anyhow::Result<()> {
    let _ = firebase_auth_rs::api::update_profile::update_profile(
        client,
        &dotenv::FIREBASE_API_KEY.to_string(),
        UpdateProfileRequestBodyPayload::new(
            context.id_token.clone(),
            display_name,
            photo_url,
            delete_attribute,
            false,
        ),
    )
    .await
    .map_err(|error| {
        log::error!("Sign up failed: {:?}", error);
        error
    })?;

    Ok(())
}
