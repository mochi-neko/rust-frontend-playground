use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, to_owned,
    use_shared_state, use_state, Element, Props, Scope,
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use crate::application_context::ApplicationContext;
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignIn(cx: Scope) -> Element {
    // Setup hooks
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);

    render! {
        h1 { "Sign in" }

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
            span {
                onclick: |_| sign_in(cx, email.get().clone(), password.get().clone()),
                MatButton {
                    label: "Sign In",
                    outlined: true,
                    disabled: email.is_empty() || password.is_empty(),
                }
            }
        }

        br {}

        div {
            label {
                "If you don't have an account, please "
            }

            Link {
                to: Route::SignUp {},
                "sign up",
            }

            label {
                "."
            }
        }

        div {
            label {
                "If you forgot your password, please "
            }

            Link {
                to: Route::ResetPassword {},
                "reset password",
            }

            label {
                "."
            }
        }
    }
}

fn sign_in(
    cx: &dioxus::prelude::Scoped<'_, SignInProps>,
    email: String,
    password: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigator = use_navigator(cx).clone();

    cx.spawn({
        async move {
            log::info!("Sign in: {:?}", email);
            let mut context = context.write();
            match crate::auth::sign_in(&context.client, email, password).await {
                | Ok(auth_context) => {
                    log::info!("Sign in success");
                    // NOTE: Update auth context
                    context.set_auth(auth_context);
                    // NOTE: Navigate to dashboard
                    navigator.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::error!("Sign in failed: {:?}", error);
                },
            }
        }
    });
}
