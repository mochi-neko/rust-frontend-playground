use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, to_owned, use_future,
    use_state, Element, Props, Scope,
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{button::MatButton, text_inputs::MatTextField};

use super::route::Route;
use crate::auth::send_reset_password_email::send_reset_password_email;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn ResetPassword(cx: Scope) -> Element {
    let navigation = use_navigator(cx);
    let email = use_state(cx, String::new);
    let error_message = use_state::<Option<String>>(cx, || None);

    let send_password_reset_email = use_future(cx, (), |_| {
        let email = email.get().clone();
        let navigation = navigation.clone();
        let error_message = error_message.clone();

        async move {
            if email.is_empty() {
                error_message.set(None);
                return;
            }

            log::info!("Send password reset email: {:?}", email);
            let result = send_reset_password_email(email).await;
            match result {
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
    });

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
                onclick: move |_| {
                    if !email.get().is_empty() {
                        log::info!("Send password reset email to {}", email.get());
                        send_password_reset_email.restart();
                    }
                },
                MatButton{
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
