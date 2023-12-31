use async_std::sync::Mutex;
use std::sync::Arc;

use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, use_shared_state,
    Element, Scope,
};
use dioxus_router::hooks::use_navigator;
use material_dioxus::MatButton;

use crate::{application_context::ApplicationContext, routings::route::Route};

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn SignInAnonymously(cx: Scope) -> Element {
    // Setup hooks
    let context =
        use_shared_state::<Arc<Mutex<ApplicationContext>>>(cx).unwrap();
    let navigator = use_navigator(cx);

    let sign_in = move |cx: &Scope<'_>| {
        log::info!("Sign in anonymously");

        let context = context.clone();
        let navigator = navigator.clone();

        cx.spawn(async move {
            let context = context.clone();
            let context = context.read();
            let mut context = context.lock().await;
            match context
                .auth_config
                .clone()
                .sign_in_anonymously()
                .await
            {
                | Ok(session) => {
                    log::info!("Sign in anonymously success");
                    context.auth_session = Some(session);
                    navigator.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::error!(
                        "Error to sign in anonymously: {:?}",
                        error,
                    );
                },
            }
        });
    };

    sign_in(&cx);

    render! {
        h1 { "Sign in anonymously" }

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
