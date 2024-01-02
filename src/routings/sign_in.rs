use async_std::sync::Mutex;
use dioxus::hooks::UseSharedState;
use std::sync::Arc;

use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, to_owned,
    use_shared_state, use_state, Element, GlobalAttributes, IntoDynNode, Scope,
    Scoped, UseState,
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use crate::application_context::ApplicationContext;
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn SignIn(cx: Scope) -> Element {
    // Setup hooks
    let context =
        use_shared_state::<Arc<Mutex<ApplicationContext>>>(cx).unwrap();
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let error_message = use_state::<Option<String>>(cx, || None);
    let navigator = use_navigator(cx);

    render! {
        h1 { "Sign in" }

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
            MatTextField {
                label: "Password",
                value: password.get().clone().replace(|_| true, "*"),
                _oninput: {
                    to_owned![password];
                    move |event: String| {
                        password.set(event)
                    }
                }
            }
        }

        div {
            span {
                onclick: |_| {
                    if can_sign_in(email, password)
                    {
                        sign_in(cx, context, email.get().clone(), password.get().clone(), error_message)
                    }
                },
                MatButton {
                    label: "Sign In",
                    outlined: true,
                    disabled: !can_sign_in(email, password),
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

        br {}

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

fn can_sign_in(
    email: &UseState<String>,
    password: &UseState<String>,
) -> bool {
    !email.get().is_empty() && !password.get().is_empty()
}

fn sign_in(
    cx: &Scoped<'_>,
    context: &UseSharedState<Arc<Mutex<ApplicationContext>>>,
    email: String,
    password: String,
    error_message: &UseState<Option<String>>,
) {
    let context = context.clone();
    let navigator = use_navigator(cx).clone();
    let error_message = error_message.clone();

    cx.spawn({
        async move {
            log::info!("Sign in: {:?}", email);
            error_message.set(None);
            let context = context.clone();
            let context = context.read();
            let mut context = context.lock().await;
            match context.auth_config.sign_in_with_email_password(
                email,
                password,
            ).await {
                | Ok(session) => {
                    log::info!("Sign in success");
                    context.auth_session = Some(session);
                    navigator.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::error!("Sign in failed: {:?}", error);
                    match error {
                        | fars::error::Error::ApiError {
                            status_code: _,
                            error_code,
                            response: _,
                        } => match error_code {
                            | fars::error::CommonErrorCode::InvalidLoginCredentials => {
                                error_message.set(Some("Error: Invalid email or password.".to_string()));
                            },
                            | fars::error::CommonErrorCode::UserDisabled => {
                                error_message.set(Some("Error: User disabled.".to_string()));
                            },
                            | _ => {
                                error_message.set(Some("Error: Internal error.".to_string()));
                            },
                        },
                        | _ => {
                            error_message.set(Some("Error: Internal error.".to_string()));
                        },
                    }
                },
            }
        }
    });
}
