use dioxus::{
    hooks::{to_owned, UseState},
    prelude::{
        dioxus_elements, fc_to_builder, inline_props, render, use_future,
        use_state, Element, Props, Scope,
    },
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use super::route::Route;
use crate::auth::sign_up::{sign_up, SignUpInfo};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignUp(cx: Scope) -> Element {
    let navigation = use_navigator(cx);

    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);

    let sign_up = use_future(cx, (), |_| {
        let email = email.get().clone();
        let password = password.get().clone();
        let navigation = navigation.clone();

        async move {
            let info = SignUpInfo {
                email,
                password,
            };

            log::info!("Sign up: {:?}", info);
            let result = sign_up(&info).await;
            if result.is_ok() {
                log::info!("Sign up success");
                navigation.push(Route::Dashboard {});
            } else {
                log::info!(
                    "Sign up failed: {:?}",
                    result.err().unwrap()
                );
            }
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
                onclick: move |_| {
                    if can_sign_up(email, password, confirm_password) {
                        log::info!("Sign up");
                        sign_up.restart();
                    }
                },
                MatButton{
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
}
