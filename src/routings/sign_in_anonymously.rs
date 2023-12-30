use dioxus::{
    hooks::use_shared_state,
    prelude::{component, dioxus_elements, render, Element, Scope},
};
use dioxus_router::hooks::use_navigator;

use crate::{application_context::ApplicationContext, routings::route::Route};

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn SignInAnonymously(cx: Scope) -> Element {
    let sign_in = move |cx: &Scope<'_>| {
        log::info!("Sign in anonymously");

        let context = use_shared_state::<ApplicationContext>(cx)
            .unwrap()
            .clone();
        let navigator = use_navigator(cx).clone();

        cx.spawn(async move {
            let mut context = context.write_silent();
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
    }
}
