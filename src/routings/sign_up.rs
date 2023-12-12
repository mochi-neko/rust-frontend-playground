use dioxus::{
    hooks::{to_owned, use_shared_state, UseState},
    prelude::{
        dioxus_elements, fc_to_builder, inline_props, render, use_state,
        Element, Props, Scope,
    },
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use crate::application_context::ApplicationContext;
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignUp(cx: Scope) -> Element {
    // Setup hooks
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);

    render! {
        h1 { "Sign up" }

        div {
            MatTextField {
                label: "E-mail",
                value: email.get(),
                _oninput: {
                    to_owned![email];
                    move |event :String| {
                        log::info!("Input e-mail address: {}", event);
                        email.set(event)
                    }
                }
            }
        }

        div {
            MatTextField {
                label: "Password",
                value: password.get().clone().replace(|_| true, "*"),
                _oninput: {
                    to_owned![password];
                    move |event: String| {
                        // NOTE: Hide password
                        // log::info!("Input password: {}", event);
                        password.set(event)
                    }
                }
            }
        }

        div {
            MatTextField {
                label: "Confirm password",
                value: confirm_password.get().clone().replace(|_| true, "*"),
                _oninput: {
                    to_owned![confirm_password];
                    move |event: String| {
                        // NOTE: Hide password
                        // log::info!("Input confirm_password: {}", event);
                        confirm_password.set(event)
                    }
                }
            }
        }

        div {
            span {
                onclick: |_| sign_up(cx, email.get().clone(), password.get().clone()),
                MatButton {
                    label: "Sign Up",
                    outlined: true,
                    disabled: !can_sign_up(email, password, confirm_password),
                }
            }
        }

        br {}

        div {
            label {
                "If you already have an account, please "
            }

            Link {
                to: Route::SignIn {},
                "sign in",
            }

            label {
                "."
            }
        }
    }
}

fn can_sign_up(
    email: &UseState<String>,
    password: &UseState<String>,
    confirm_password: &UseState<String>,
) -> bool {
    !email.get().is_empty()
        && !password.get().is_empty()
        && !confirm_password
            .get()
            .is_empty()
        && password.get() == confirm_password.get()
}

fn sign_up(
    cx: &dioxus::prelude::Scoped<'_, SignUpProps>,
    email: String,
    password: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigator = use_navigator(cx).clone();

    cx.spawn(async move {
        log::info!("Sign up: {:?}", email);
        let mut context = context.write();
        match crate::auth::sign_up(&context.client, email, password).await {
            | Ok(auth_context) => {
                log::info!("Sign up success");
                // NOTE: Update auth context
                context.set_auth(auth_context);
                // NOTE: Navigate to dashboard
                navigator.push(Route::Dashboard {});
            },
            | Err(error) => {
                log::error!("Sign up failed: {:?}", error);
            },
        }
    });
}
