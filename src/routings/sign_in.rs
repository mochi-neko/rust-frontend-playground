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
use crate::auth::sign_in::{sign_in, SignInInfo};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignIn(cx: Scope) -> Element {
    let navigation = use_navigator(cx);

    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let sign_in = use_future(cx, (), |_| {
        let email = email.get().clone();
        let password = password.get().clone();
        let navigation = navigation.clone();

        async move {
            let info = SignInInfo {
                email,
                password,
            };

            log::info!("Sign in: {:?}", info);
            let result = sign_in(&info).await;
            if result.is_ok() {
                log::info!("Sign in success");
                navigation.push(Route::Dashboard {});
            } else {
                log::info!(
                    "Sign in failed: {:?}",
                    result.err().unwrap()
                );
            }
        }
    });

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
                onclick: move |_| {
                    if can_sign_in(email, password) {
                        log::info!("Sign in");
                        sign_in.restart();
                    }
                },
                MatButton{
                    label: "Sign In",
                    outlined: true,
                    disabled: !can_sign_in(email, password),
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

fn can_sign_in(
    email: &UseState<String>,
    password: &UseState<String>,
) -> bool {
    !email.get().is_empty() && !password.get().is_empty()
}
