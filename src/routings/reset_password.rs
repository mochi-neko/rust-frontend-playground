use async_std::sync::Mutex;
use std::sync::Arc;

use dioxus::{
    hooks::UseSharedState,
    prelude::{
        component, dioxus_elements, fc_to_builder, render, to_owned,
        use_shared_state, use_state, Element, GlobalAttributes, IntoDynNode,
        Scope, Scoped, UseState,
    },
};
use dioxus_router::hooks::use_navigator;
use material_dioxus::{button::MatButton, text_inputs::MatTextField};

use crate::application_context::ApplicationContext;

use super::route::Route;

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn ResetPassword(cx: Scope) -> Element {
    // Setup hooks
    let context =
        use_shared_state::<Arc<Mutex<ApplicationContext>>>(cx).unwrap();
    let email = use_state(cx, String::new);
    let error_message = use_state::<Option<String>>(cx, || None);
    let navigator = use_navigator(cx);

    render! {
        h1 { "Reset password" }

        div {
            MatTextField {
                label: "E-mail",
                value: email.get(),
                _oninput: {
                    to_owned![email];
                    move |event :String| {
                        email.set(event)
                    }
                }
            }
        }

        div {
            span {
                onclick: |_| {
                    if can_send(email)
                    {
                        send_send_password_reset_email(cx, context, email.get().clone(), error_message)
                    }
                },
                MatButton {
                    label: "Send password reset email",
                    outlined: true,
                    disabled: !can_send(email),
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

                    br {}
                }
            }
        }

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

fn can_send(email: &UseState<String>) -> bool {
    !email.get().is_empty()
}

fn send_send_password_reset_email(
    cx: &Scoped<'_>,
    context: &UseSharedState<Arc<Mutex<ApplicationContext>>>,
    email: String,
    error_message: &UseState<Option<String>>,
) {
    // Setup hooks
    let context = context.clone();
    let error_message = error_message.clone();
    let navigation = use_navigator(cx).clone();

    cx.spawn({
        async move {
            log::info!("Send password reset email: {:?}", email);
            error_message.set(None);
            let context = context.clone();
            let context = context.read();
            let context = context.lock().await;
            match context
                .auth_config
                .send_reset_password_email(email, None)
                .await
            {
                | Ok(_) => {
                    log::info!("Send password reset email success");
                    error_message.set(None);
                    navigation.push(Route::SignIn {});
                },
                | Err(error) => {
                    log::error!("Sign up failed: {:?}", error);
                    match error {
                        | fars::error::Error::ApiError {
                            status_code: _,
                            error_code,
                            response: _,
                        } => match error_code {
                            | fars::error::CommonErrorCode::EmailNotFound => {
                                error_message.set(Some(
                                    "Error: E-mail address not found."
                                        .to_string(),
                                ));
                            },
                            | _ => {
                                error_message.set(Some(
                                    "Error: Internal error.".to_string(),
                                ));
                            },
                        },
                        | _ => {
                            error_message.set(Some(
                                "Error: Internal error.".to_string(),
                            ));
                        },
                    }
                },
            }
        }
    })
}
