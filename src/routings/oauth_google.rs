use async_std::sync::Mutex;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Display};

use dioxus::prelude::{
    component, dioxus_elements, render, use_shared_state, Element, Props, Scope,
};
use dioxus_router::prelude::{use_navigator, FromQuery};
use fars::{config::AuthConfig, data::IdpPostBody, session::AuthSession};
use google_oauth_rs::api::exchange_access_token::{
    ExchangeAccessTokenRequestParameters, GrandType,
};

use crate::{
    application_context::ApplicationContext, generated::dotenv,
    routings::route::Route,
};

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn OAuthGoogle(
    cx: Scope,
    query: RedirectToAuthServerResponseQuery,
) -> Element {
    // Setup hooks
    let context =
        use_shared_state::<Arc<Mutex<ApplicationContext>>>(cx).unwrap();
    let navigator = use_navigator(cx);

    let sign_in = move |cx: &Scope<'_, OAuthGoogleProps>| {
        log::info!("Sign in with Google");

        let context = context.clone();
        let navigator = navigator.clone();
        let code = query.code.clone();

        cx.spawn(async move {
            let context = context.clone();
            let context = context.read();
            let mut context = context.lock().await;
            match sign_in_with_google(context.auth_config.clone(), code).await {
                | Ok(session) => {
                    log::info!("Sign in with Google success");
                    context.auth_session = Some(session);
                    navigator.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::error!(
                        "Error to exchange access token: {:?}",
                        error,
                    );
                },
            }
        });
    };

    sign_in(&cx);

    render! {
        h1 { "Signing in with Google..." }
    }
}

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn OAuthGoogleError(
    cx: Scope,
    error: RedirectToAuthServerResponseErrorQuery,
) -> Element {
    log::info!(
        "Redirect error OAuth with Google: error: {:?}",
        error,
    );

    render! {
        h1 { "Error to sign in with Google" }
    }
}

fn parse_query_str(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|part| {
            let mut parts = part.split('=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                Some((key.to_string(), value.to_string()))
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RedirectToAuthServerResponseQuery {
    pub code: String,
    pub scope: String,
    pub authuser: usize,
    pub prompt: String,
    pub state: Option<String>,
}

impl Display for RedirectToAuthServerResponseQuery {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut query = String::new();

        query.push_str("code=");
        query.push_str(&self.code);

        query.push_str("&scope=");
        query.push_str(&self.scope);

        query.push_str("&authuser=");
        query.push_str(&self.authuser.to_string());

        query.push_str("&prompt=");
        query.push_str(&self.prompt);

        if let Some(state) = &self.state {
            query.push_str("&state=");
            query.push_str(state);
        }

        write!(f, "{}", query)
    }
}

impl FromQuery for RedirectToAuthServerResponseQuery {
    fn from_query(query: &str) -> Self {
        log::info!(
            "RedirectToAuthServerResponseQuery: {}",
            query
        );
        let params = parse_query_str(query);
        RedirectToAuthServerResponseQuery {
            code: params
                .get("code")
                .cloned()
                .unwrap_or_default(),
            scope: params
                .get("scope")
                .cloned()
                .unwrap_or_default(),
            authuser: params
                .get("authuser")
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
            prompt: params
                .get("prompt")
                .cloned()
                .unwrap_or_default(),
            state: params.get("state").cloned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RedirectToAuthServerResponseErrorQuery {
    pub error: String,
}

impl Display for RedirectToAuthServerResponseErrorQuery {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut query = String::new();

        query.push_str("error=");
        query.push_str(&self.error);

        write!(f, "{}", query)
    }
}

impl FromQuery for RedirectToAuthServerResponseErrorQuery {
    fn from_query(query: &str) -> Self {
        let params = parse_query_str(query);
        RedirectToAuthServerResponseErrorQuery {
            error: params
                .get("error")
                .cloned()
                .unwrap_or_default(),
        }
    }
}

async fn sign_in_with_google(
    auth_config: AuthConfig,
    auth_code: String,
) -> anyhow::Result<AuthSession> {
    let client = reqwest::ClientBuilder::new().build()?;

    let request_parameter = ExchangeAccessTokenRequestParameters {
        client_id: dotenv::GOOGLE_CLIENT_ID.to_string(),
        client_secret: dotenv::GOOGLE_CLIENT_SECRET.to_string(),
        code: auth_code,
        grant_type: GrandType::AuthorizationCode,
        redirect_uri: "http://localhost:8080/auth/google-callback".to_string(),
    };

    let token_response =
        google_oauth_rs::api::exchange_access_token::exchange_access_token(
            &client,
            request_parameter,
        )
        .await?;

    log::info!("Exchange access token success");

    let session = auth_config
        .sign_in_oauth_credencial(
            "http://localhost:8080/auth/google-callback".to_string(),
            IdpPostBody::Google {
                id_token: token_response.id_token,
            },
        )
        .await?;

    log::info!("Sign in with OAuth credential success");

    Ok(session)
}
