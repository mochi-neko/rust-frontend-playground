use async_std::sync::Mutex;
use std::sync::Arc;

use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, to_owned,
    use_shared_state, use_state, Element, GlobalAttributes, IntoDynNode, Scope,
    Scoped, UseSharedState, UseState,
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use crate::application_context::ApplicationContext;
use crate::credential::{is_valid_email, is_valid_password};
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn SignUp(cx: Scope) -> Element {
    // Setup hooks
    let context =
        use_shared_state::<Arc<Mutex<ApplicationContext>>>(cx).unwrap();
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);
    let error_message = use_state::<Option<String>>(cx, || None);
    let navigator = use_navigator(cx);

    render! {
        h1 { "Sign up" }

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

            if email.get().is_empty() {
                render! { span {} }
            }
            else if is_valid_email(email.get().clone()) {
                render! {
                    span {
                        color: "green",
                        label {
                            "✓"
                        }
                    }
                }
            } else {
                render! {
                    span {
                        color: "red",
                        label {
                            " Please enter a valid e-mail address."
                        }
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

            if password.get().is_empty() {
                render! { span {} }
            }
            else if is_valid_password(password.get().clone()) {
                render! {
                    span {
                        color: "green",
                        label {
                            "✓"
                        }
                    }
                }
            } else {
                render! {
                    span {
                        color: "red",
                        label {
                            " Please enter a valid password more than 6 characters."
                        }
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
                        confirm_password.set(event)
                    }
                }
            }

            if confirm_password.get().is_empty() {
                render! { span {} }
            }
            else if password.get() == confirm_password.get() {
                render! {
                    span {
                        color: "green",
                        label {
                            "✓"
                        }
                    }
                }
            } else {
                render! {
                    span {
                        color: "red",
                        label {
                            " Passwords do not match."
                        }
                    }
                }
            }
        }

        div {
            span {
                onclick: move |_| {
                    if can_sign_up(email, password, confirm_password)
                    {
                        sign_up(cx, context, email.get().clone(), password.get().clone(), error_message)
                    }
                },
                MatButton {
                    label: "Sign Up",
                    outlined: true,
                    disabled: !can_sign_up(email, password, confirm_password),
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

fn can_sign_up(
    email: &UseState<String>,
    password: &UseState<String>,
    confirm_password: &UseState<String>,
) -> bool {
    let email = email.get();
    let password = password.get();
    let confirm_password = confirm_password.get();

    is_valid_email(email.clone())
        && is_valid_password(password.clone())
        && password == confirm_password
}

fn sign_up(
    cx: &Scoped<'_>,
    context: &UseSharedState<Arc<Mutex<ApplicationContext>>>,
    email: String,
    password: String,
    error_message: &UseState<Option<String>>,
) {
    // Setup hooks
    let context = context.clone();
    let navigator = use_navigator(cx).clone();
    let error_message = error_message.clone();

    cx.spawn(async move {
        log::info!("Sign up: {:?}", email);
        error_message.set(None);
        let context = context.clone();
        let context = context.read();
        let mut context = context.lock().await;
        match context.auth_config.sign_up_with_email_password(
            email,
            password,
        ).await {
            | Ok(session) => {
                log::info!("Sign up success");
                context.auth_session = Some(session);
                navigator.push(Route::Dashboard {});
            },
            | Err(error) => {
                log::error!("Sign up failed: {:?}", error);
                match error {
                    | fars::error::Error::ApiError {
                        status_code: _,
                        error_code,
                        response: _,
                    } => match error_code {
                        | fars::error::CommonErrorCode::EmailExists => {
                            error_message.set(Some("Error: E-mail address already exists.".to_string()));
                        },
                        | fars::error::CommonErrorCode::OperationNotAllowed => {
                            error_message.set(Some("Error: Operation not allowed.".to_string()));
                        },
                        | fars::error::CommonErrorCode::TooManyAttemptsTryLater => {
                            error_message.set(Some("Error: Too many attempts. Please try again later.".to_string()));
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
        };
    });
}
