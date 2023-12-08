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
    sign_in::{sign_in, SignInInfo},
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignIn(cx: Scope) -> Element {
    let auth_context = use_shared_state::<Option<AuthContext>>(cx).unwrap();
    let navigation = use_navigator(cx);
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);

    let sign_in = use_future(cx, (), |_| {
        let email = email.get().clone();
        let password = password.get().clone();
        let navigation = navigation.clone();
        let auth_context = auth_context.clone();

        async move {
            if email.is_empty() || password.is_empty() {
                return;
            }

            let info = SignInInfo {
                email,
                password,
            };

            log::info!("Sign in: {:?}", info);
            let result = sign_in(&info).await;
            match result {
                | Ok(context) => {
                    log::info!("Sign in success");

                    // NOTE: Update auth context
                    let mut auth_context_ref = auth_context.write();
                    *auth_context_ref = Some(context);

                    // NOTE: Navigate to dashboard
                    navigation.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::error!("Sign in failed: {:?}", error);
                },
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

fn can_sign_in(
    email: &UseState<String>,
    password: &UseState<String>,
) -> bool {
    !email.get().is_empty() && !password.get().is_empty()
}
