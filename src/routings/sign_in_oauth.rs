use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, Element, Scope,
};
use dioxus_router::prelude::use_navigator;
use material_dioxus::MatButton;

use crate::routings::route::Route;

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn SignInWithOAuth(cx: Scope) -> Element {
    // Setup hooks
    let navigator = use_navigator(cx);

    render! {
        h1 { "Sign in with OAuth" }

        div {
            span {
                onclick: |_| {
                    log::info!("Sign in with Google");
                    let _ = authorize_with_google();
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
                onclick: move |_| {
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

fn authorize_with_google() -> anyhow::Result<()> {
    if let Some(window) = web_sys::window() {
        let url = google_oauth_rs::api::request_authorization::AuthorizationRequestParameters {
            client_id: crate::generated::dotenv::GOOGLE_CLIENT_ID.to_string(),
            redirect_uri: "http://localhost:8080/auth/google-callback".to_string(),
            scope: vec![
                google_oauth_rs::api::request_authorization::Scope::OpenID,
                google_oauth_rs::api::request_authorization::Scope::Email,
                google_oauth_rs::api::request_authorization::Scope::Profile
            ],
            response_type: google_oauth_rs::api::request_authorization::ResponseType::Code,
            access_type: Some(google_oauth_rs::api::request_authorization::AccessType::Offline),
            state: Some("state".to_string()), // TODO: Generate a random string
            include_granted_scopes: Some(true),
            enable_granular_consent: None,
            login_hint: None,
            prompt: None,
        }.build_redirect_uri()?;

        let location = window.location();
        match location.set_href(url.as_str()) {
            | Ok(_) => Ok(()),
            | Err(e) => Err(anyhow::anyhow!(
                "Failed to set href: {:?}",
                e
            )),
        }
    } else {
        Err(anyhow::anyhow!("Failed to get window"))
    }
}
