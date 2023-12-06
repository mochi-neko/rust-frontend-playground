use dioxus::{
    hooks::{to_owned, use_shared_state, UseState},
    prelude::{
        dioxus_elements, fc_to_builder, inline_props, render, use_future,
        use_state, Element, Props, Scope,
    },
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use super::route::Route;
use crate::auth::{
    auth_context::AuthContext,
    sign_up::{sign_up, SignUpInfo},
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignUp(cx: Scope) -> Element {
    let auth_context = use_shared_state::<Option<AuthContext>>(cx).unwrap();
    let navigation = use_navigator(cx);

    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);

    let sign_up = use_future(cx, (), |_| {
        let email = email.get().clone();
        let password = password.get().clone();
        let navigation = navigation.clone();
        let auth_context = auth_context.clone();

        async move {
            if email.is_empty() || password.is_empty() {
                return;
            }

            let info = SignUpInfo {
                email,
                password,
            };

            log::info!("Sign up: {:?}", info);
            let result = sign_up(&info).await;
            match result {
                | Ok(context) => {
                    log::info!("Sign up success");

                    // NOTE: Update auth context
                    let mut auth_context_ref = auth_context.write();
                    *auth_context_ref = Some(context);

                    // NOTE: Navigate to dashboard
                    navigation.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::info!("Sign up failed: {:?}", error);
                },
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
