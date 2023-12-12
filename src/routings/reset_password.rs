use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, to_owned,
    use_shared_state, use_state, Element, Props, Scope,
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
                onclick: |_| send_send_password_reset_email(cx, error_message, email.get().clone()),
                MatButton {
                    label: "Send password reset email",
                    outlined: true,
                    disabled: email.get().is_empty(),
                }
            }
        }

        div {
            if let Some(message) = error_message.get() {
                render! {
                    label {
                    "Error: "
                    }
                    label {
                        message.as_str()
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
    error_message: &dioxus::prelude::UseState<Option<String>>,
    email: String,
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
            let context = context.read();
            match send_reset_password_email(&context.client, email).await {
                | Ok(_) => {
                    log::info!("Send password reset email success");
                    error_message.set(None);
                    navigation.push(Route::SignIn {});
                },
                | Err(error) => {
                    log::error!(
                        "Send password reset email failed: {:?}",
                        error
                    );
                    error_message.set(Some(error.to_string()));
                },
            }
        }
    })
}
