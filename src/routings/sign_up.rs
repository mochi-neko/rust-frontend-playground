use dioxus::{
    hooks::{to_owned, use_shared_state, UseState},
    html::GlobalAttributes,
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
    let error_message = use_state(cx, String::new);

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
                onclick: move |_| sign_up(cx, email.get().clone(), password.get().clone(), error_message),
                MatButton {
                    label: "Sign Up",
                    outlined: true,
                    disabled: !can_sign_up(email, password, confirm_password),
                }
            }
        }

        br {}

        div {
            color: "red",
            label {
                error_message.get().as_str(),
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
    error_message: &UseState<String>,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigator = use_navigator(cx).clone();
    let error_message = error_message.clone();

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
                match error {
                    | crate::error::Error::FirebaseAuthError {
                        inner,
                    } => match inner {
                        | firebase_auth_rs::error::Error::ApiError {
                            status_code: _,
                            error_code,
                            response: _,
                        } => match error_code {
                            | firebase_auth_rs::error::CommonErrorCode::EmailExists => {
                                error_message.set("E-mail address already exists.".to_string());
                            },
                            | firebase_auth_rs::error::CommonErrorCode::OperationNotAllowed => {
                                error_message.set("Operation not allowed.".to_string());
                            },
                            | firebase_auth_rs::error::CommonErrorCode::TooManyAttemptsTryLater => {
                                error_message.set("Too many attempts. Please try again later.".to_string());
                            },
                            | _ => {
                                error_message.set("Internal error code.".to_string());
                            },
                        },
                        | _ => {
                            error_message.set("Internal error".to_string());
                        },
                    },
                }
            },
        }
    });
}
