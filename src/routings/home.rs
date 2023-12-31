use std::sync::Arc;

use async_std::sync::Mutex;
use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, use_shared_state,
    Element, Scope, Scoped, UseSharedState,
};
use dioxus_router::hooks::use_navigator;
use material_dioxus::MatButton;

use crate::{application_context::ApplicationContext, routings::route::Route};

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn Home(cx: Scope) -> Element {
    // Setup hooks
    let context =
        use_shared_state::<Arc<Mutex<ApplicationContext>>>(cx).unwrap();
    let navigator = use_navigator(cx);

    redirect_to_dashboard_if_logged_in(cx, context);

    render! {
        h1 { "Home" }

        div {
            span {
                onclick: |_| {
                    navigator.push(Route::SignUp { });
                },
                MatButton {
                    label: "Sign up with email",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    navigator.push(Route::SignIn { });
                },
                MatButton {
                    label: "Sign in with email",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    navigator.push(Route::SignInWithOAuth { });
                },
                MatButton {
                    label: "Sign in with OAuth",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    navigator.push(Route::SignInAnonymously { });
                },
                MatButton {
                    label: "Sign in anonymously",
                    outlined: true,
                }
            }
        }
    }
}

fn redirect_to_dashboard_if_logged_in(
    cx: &Scoped<'_>,
    context: &UseSharedState<Arc<Mutex<ApplicationContext>>>,
) {
    // Setup hooks
    let context = context.clone();
    let navigation = use_navigator(cx).clone();

    cx.spawn(async move {
        let context = context.clone();
        let context = context.read();
        let context = context.lock().await;
        if context.auth_session.is_some() {
            log::info!("Redirect to dashboard");
            navigation.push(Route::Dashboard {});
        }
    });
}
