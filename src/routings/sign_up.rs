use dioxus::{
    hooks::{to_owned, UseState},
    prelude::{
        dioxus_elements, fc_to_builder, inline_props, render, use_future,
        use_state, Element, Props, Scope,
    },
};
use material_dioxus::{MatButton, MatTextField};

use crate::auth::sign_up::{sign_up, SignUpInfo};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignUp(cx: Scope) -> Element {
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);
    let log_in = use_future(cx, (), |_| {
        let email = email.get().clone();
        let password = password.get().clone();

        async move {
            let info = SignUpInfo {
                mail_address: email,
                password,
            };

            log::info!("Sign up: {:?}", info);
            sign_up(&info)
                .await
                .unwrap_or_default();
        }
    });

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

            br {}

            MatTextField {
                label: "Password",
                value: password_field(password.get().clone()),
                _oninput: {
                    to_owned![password];
                    move |event: String| {
                        // NOTE: Hide password
                        // log::info!("Input password: {}", event);
                        password.set(event)
                    }
                }
            }

            br {}

            MatTextField {
                label: "Confirm password",
                value: password_field(confirm_password.get().clone()),
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
                onclick: move |_| {
                    if can_sign_up(email, password, confirm_password) {
                        log::info!("Sign up");
                        log_in.restart();
                    }
                },
                MatButton{
                    label: "Sign Up",
                    outlined: true,
                    disabled: !can_sign_up(email, password, confirm_password),
                }
            }
        }
    }
}

fn password_field(password: String) -> String {
    let count = password.chars().count();

    password
        .chars()
        .enumerate()
        .map(|(index, character)| {
            if index != count - 1 {
                '*'
            } else {
                character
            }
        })
        .collect::<String>()
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
}
