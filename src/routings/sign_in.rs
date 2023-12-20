use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, to_owned,
    use_shared_state, use_state, Element, GlobalAttributes, Props, Scope,
    Scoped, UseState,
};
use dioxus_router::{components::Link, hooks::use_navigator};
use material_dioxus::{MatButton, MatTextField};

use crate::application_context::ApplicationContext;
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignIn(cx: Scope) -> Element {
    // Setup hooks
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let error_message = use_state::<Option<String>>(cx, || None);

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
                        sign_in(cx, email.get().clone(), password.get().clone(), error_message)
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

        br {}

        div {
            label {
                "Back to "
            }

            Link {
                to: Route::Home {},
                "home",
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

fn sign_in(
    cx: &Scoped<'_, SignInProps>,
    email: String,
    password: String,
    error_message: &UseState<Option<String>>,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigator = use_navigator(cx).clone();
    let error_message = error_message.clone();

    cx.spawn({
        async move {
            log::info!("Sign in: {:?}", email);
            error_message.set(None);
            let mut context = context.write();
            match crate::auth::sign_in(&context.client, email, password).await {
                | Ok(auth_context) => {
                    log::info!("Sign in success");
                    // NOTE: Update auth context
                    context.set_auth(auth_context);
                    // NOTE: Navigate to dashboard
                    navigator.push(Route::Dashboard {});
                },
                | Err(error) => {
                    log::error!("Sign in failed: {:?}", error);
                    match error {
                        | crate::error::Error::FirebaseAuthError {
                            inner,
                        } => match inner {
                            | firebase_auth_rs::error::Error::ApiError {
                                status_code: _,
                                error_code,
                                response: _,
                            } => match error_code {
                                | firebase_auth_rs::error::CommonErrorCode::InvalidLoginCredentials => {
                                    error_message.set(Some("Error: Invalid email or password.".to_string()));
                                },
                                | firebase_auth_rs::error::CommonErrorCode::UserDisabled => {
                                    error_message.set(Some("Error: User disabled.".to_string()));
                                },
                                | _ => {
                                    error_message.set(Some("Error: Internal error.".to_string()));
                                },
                            },
                            | _ => {
                                error_message.set(Some("Error: Internal error.".to_string()));
                            },
                        },
                    }
                },
            }
        }
    });
}
