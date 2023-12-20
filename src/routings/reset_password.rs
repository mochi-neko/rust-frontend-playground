use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, to_owned,
    use_shared_state, use_state, Element, GlobalAttributes, Props, Scope,
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{button::MatButton, text_inputs::MatTextField};

use super::route::Route;
use crate::{
    application_context::ApplicationContext, auth::send_reset_password_email,
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn ResetPassword(cx: Scope) -> Element {
    // Setup hooks
    let email = use_state(cx, String::new);
    let error_message = use_state::<Option<String>>(cx, || None);

    render! {
        h1 { "Reset password" }

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
            span {
                onclick: |_| send_send_password_reset_email(cx, email.get().clone(), error_message),
                MatButton {
                    label: "Send password reset email",
                    outlined: true,
                    disabled: email.get().is_empty(),
                }
            }
        }

        br {}

        div {
            if let Some(error_message) = error_message.get() {
                render! {
                    div {
                        color: "red",
                        label {
                            error_message.as_str(),
                        }
                    }
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
    }
}

fn send_send_password_reset_email(
    cx: &dioxus::prelude::Scoped<'_, ResetPasswordProps>,
    email: String,
    error_message: &dioxus::prelude::UseState<Option<String>>,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigation = use_navigator(cx).clone();
    let error_message = error_message.clone();

    cx.spawn({
        async move {
            log::info!("Send password reset email: {:?}", email);
            error_message.set(None);
            let context = context.read();
            match send_reset_password_email(&context.client, email).await {
                | Ok(_) => {
                    log::info!("Send password reset email success");
                    error_message.set(None);
                    navigation.push(Route::SignIn {});
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
                                | firebase_auth_rs::error::CommonErrorCode::EmailNotFound => {
                                    error_message.set(Some("Error: E-mail address not found.".to_string()));
                                },
                                | _ => {
                                    error_message.set(Some("Error: Internal error.".to_string()));
                                },
                            },
                            | _ => {
                                error_message.set(Some("Error: Internal error.".to_string()));
                            },
                        },
                    }
                },
            }
        }
    })
}
