use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, Element, Scope, Scoped,
};
use dioxus_router::prelude::use_navigator;
use material_dioxus::MatButton;

use crate::routings::route::Route;

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn SignInWithOAuth(cx: Scope) -> Element {
    render! {
        h1 { "Sign in with OAuth" }

        div {
            span {
                onclick: |_| {
                    log::info!("Sign in with Google");
                    let _ = sign_in_with_google(cx);
                },
                MatButton {
                    label: "Sign in with Google",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    let navigator = use_navigator(cx).clone();
                    navigator.push(Route::Home { });
                },
                MatButton {
                    label: "Back to home",
                    outlined: true,
                }
            }
        }
    }
}

fn sign_in_with_google(cx: &Scoped<'_>) -> anyhow::Result<()> {
    let window = web_sys::window().unwrap();
    let url = google_oauth_rs::api::redirect_auth_server::RedirectToAuthServerRequest {
        client_id: crate::generated::dotenv::GOOGLE_CLIENT_ID.to_string(),
        redirect_uri: "http://localhost:8080/auth/google/callback".to_string(),
        scope: vec![ google_oauth_rs::api::redirect_auth_server::Scope::Email, google_oauth_rs::api::redirect_auth_server::Scope::Profile],
        response_type: google_oauth_rs::api::redirect_auth_server::ResponseType::Code,
        access_type: Some(google_oauth_rs::api::redirect_auth_server::AccessType::Offline),
        state: Some("state".to_string()), // TODO: Generate a random string
        include_granted_scopes: Some(true),
        enable_granular_consent: None,
        login_hint: None,
        prompt: None,
    }.build_redirect_uri()?;

    let _ = window.open_with_url_and_target(url.as_str(), "_blank");

    Ok(())
}
