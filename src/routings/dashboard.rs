use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, to_owned, use_future,
    use_shared_state, use_state, Element, Props, Scope, Scoped, UseSharedState,
};
use dioxus_router::hooks::use_navigator;
use firebase_auth_rs::api::get_user_data::GetUserDataResponsePayload;
use material_dioxus::{button::MatButton, text_inputs::MatTextField};

use crate::application_context::ApplicationContext;
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn Dashboard(cx: Scope) -> Element {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx).unwrap();
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);
    let display_name = use_state(cx, String::new);
    let photo_url = use_state(cx, String::new);

    let fetch_user_data = use_future(cx, (), move |_| {
        get_user_data(context.read().clone())
    });

    redirect_to_home(cx);

    render! {
        h1 { "Dashboard" }

        div {
            span {
                onclick: move |_| {
                    log::info!("Update user data");
                    fetch_user_data.restart();
                },
                MatButton {
                    label: "Update user data",
                    outlined: true,
                }
            }
        }

        match fetch_user_data.value() {
            | Some(Err(error)) => {
                render! {
                    div {
                        "Failed to get user data: "
                        error.to_string()
                    }
                }
            },
            | None => {
                render! {
                    div {
                        "Loading user data..."
                    }
                }
            },
            | Some(Ok(user_data)) => {
                match user_data.users.get(0) {
                    | None => {
                        render! {
                            div {
                                "User not found"
                            }
                        }
                    },
                    | Some(user) => {
                        render! {
                            div {
                                span {
                                    onclick: |_| send_email_verification(cx),
                                    MatButton {
                                        label: "Send email verification",
                                        outlined: true,
                                        disabled: user.email_verified,
                                    }
                                }
                            }

                            h2 { "User info" }

                            div {
                                "Local ID: "
                                span { user.local_id.clone() }
                            }

                            div {
                                "E-mail: "
                                span { user.email.clone() }
                            }

                            div {
                                "E-mail verified: "
                                span { user.email_verified.to_string() }
                            }

                            div {
                                "Display name: "
                                span { user.display_name.clone() }
                            }

                            div {
                                "Provider user info: "
                                for provider_user_info in user.provider_user_info.iter() {
                                    div {
                                        "- Provider ID: "
                                        span { provider_user_info.provider_id.clone() }
                                    }

                                    div {
                                        "- Display name: "
                                        span { provider_user_info.display_name.clone() }
                                    }

                                    div {
                                        "- Photo URL: "
                                        span { provider_user_info.photo_url.clone() }
                                    }

                                    div {
                                        "- Federated ID: "
                                        span { provider_user_info.federated_id.clone() }
                                    }

                                    div {
                                        "- Email: "
                                        span { provider_user_info.email.clone() }
                                    }

                                    div {
                                        "- Raw ID: "
                                        span { provider_user_info.raw_id.clone() }
                                    }

                                    div {
                                        "- Screen name: "
                                        span { provider_user_info.screen_name.clone() }
                                    }
                                }
                            }

                            div {
                                "Photo URL: "
                                span { user.photo_url.clone() }
                            }

                            div {
                                "Password hash: "
                                span { "XXXX" }
                            }

                            div {
                                "Password updated at: "
                                span { user.password_updated_at.to_string() }
                            }

                            div {
                                "Valid since: "
                                span { user.valid_since.clone() }
                            }

                            div {
                                "Disabled: "
                                span { user.disabled.unwrap_or(false).to_string() }
                            }

                            div {
                                "Last login at: "
                                span { user.last_login_at.clone() }
                            }

                            div {
                                "Created at: "
                                span { user.created_at.clone() }
                            }

                            div {
                                "Custom auth: "
                                span { user.custom_auth.unwrap_or(false).to_string() }
                            }
                        }
                    },
                }
            }
        }

        br {}

        h2 { "Update credentials" }

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

        div
        {
            span {
                onclick: |_| change_email(cx, email.get().clone()),
                MatButton {
                    label: "Change e-mail",
                    outlined: true,
                    disabled: email.get().is_empty(),
                }
            }
        }

        br {}

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
        }

        div {
            span {
                onclick: |_| change_password(cx, password.get().clone()),
                MatButton {
                    label: "Change password",
                    outlined: true,
                    disabled: password.get().is_empty()
                        || confirm_password.get().is_empty(),
                }
            }
        }

        h2 { "Update user profile" }

        div {
            MatTextField {
                label: "Display name",
                value: display_name.get(),
                _oninput: {
                    to_owned![display_name];
                    move |event :String| {
                        display_name.set(event)
                    }
                }
            }
        }

        div {
            MatTextField {
                label: "Photo URL",
                value: photo_url.get(),
                _oninput: {
                    to_owned![photo_url];
                    move |event :String| {
                        photo_url.set(event)
                    }
                }
            }
        }

        div {
            span {
                onclick: |_| {
                    update_profile(cx, display_name.get().clone(), photo_url.get().clone());
                    fetch_user_data.restart();
                },
                MatButton {
                    label: "Update profile",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| sign_out(cx),
                MatButton {
                    label: "Sign out",
                    outlined: true,
                }
            }
        }

        div {
            span {
                onclick: |_| delete_account(cx),
                MatButton {
                    label: "Delete account",
                    outlined: true,
                }
            }
        }
    }
}

async fn get_user_data(
    context: ApplicationContext
) -> anyhow::Result<GetUserDataResponsePayload> {
    if let Some(auth_context) = &context.auth {
        log::info!("Get user data");
        match crate::auth::get_user_data(&context.client, auth_context).await {
            | Ok(data) => {
                log::info!("Get user data success");
                Ok(data)
            },
            | Err(error) => {
                log::error!("Get user data failed: {:?}", error);
                Err(error)
            },
        }
    } else {
        log::error!("Get user data failed: Auth context not found");
        Err(anyhow::anyhow!(
            "Auth context not found"
        ))
    }
}

fn redirect_to_home(cx: &Scoped<'_, DashboardProps>) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigation = use_navigator(cx).clone();

    if context.read().auth.is_none() {
        // NOTE: Redirect to home
        log::info!("Redirect to home");
        navigation.push(Route::Home {});
    }
}

fn send_email_verification(cx: &Scoped<'_, DashboardProps>) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let context = context.read();
            if let Some(auth_context) = &context.auth {
                log::info!("Send email verification");
                match crate::auth::send_email_verification(
                    &context.client,
                    auth_context,
                )
                .await
                {
                    | Ok(_) => {
                        log::info!("Send email verification success");
                    },
                    | Err(error) => {
                        log::error!(
                            "Send email verification failed: {:?}",
                            error
                        );
                    },
                }
            }
        }
    })
}

fn change_email(
    cx: &Scoped<'_, DashboardProps>,
    email: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let context = context.read();
            if let Some(auth_context) = &context.auth {
                log::info!("Change email");
                match crate::auth::change_email(
                    &context.client,
                    auth_context,
                    email,
                )
                .await
                {
                    | Ok(_) => {
                        log::info!("Change email success");
                    },
                    | Err(error) => {
                        log::error!("Change email failed: {:?}", error);
                    },
                }
            }
        }
    })
}

fn change_password(
    cx: &Scoped<'_, DashboardProps>,
    password: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let context = context.read();
            if let Some(auth_context) = &context.auth {
                log::info!("Change password");
                match crate::auth::change_password(
                    &context.client,
                    auth_context,
                    password,
                )
                .await
                {
                    | Ok(_) => {
                        log::info!("Change password success");
                    },
                    | Err(error) => {
                        log::error!("Change password failed: {:?}", error);
                    },
                }
            }
        }
    })
}

fn update_profile(
    cx: &Scoped<'_, DashboardProps>,
    display_name: String,
    photo_url: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let context = context.read();
            if let Some(auth_context) = &context.auth {
                log::info!("Update profile");
                match crate::auth::update_profile(
                    &context.client,
                    auth_context,
                    display_name,
                    photo_url,
                    vec![],
                )
                .await
                {
                    | Ok(_) => {
                        log::info!("Update profile success");
                    },
                    | Err(error) => {
                        log::error!("Update profile failed: {:?}", error);
                    },
                }
            }
        }
    })
}

fn sign_out(cx: &Scoped<'_, DashboardProps>) {
    // Setup hooks
    let context: UseSharedState<ApplicationContext> =
        use_shared_state::<ApplicationContext>(cx)
            .unwrap()
            .clone();
    let navigation = use_navigator(cx).clone();

    let mut context = context.write();
    if context.auth.is_some() {
        log::info!("Sign out");

        // NOTE: Reset auth context
        context.auth = None;

        // NOTE: Navigate to home
        navigation.push(Route::Home {});
    };
}

fn delete_account(cx: &Scoped<'_, DashboardProps>) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigation = use_navigator(cx).clone();

    cx.spawn({
        async move {
            let mut context = context.write();
            if let Some(auth_context) = &context.auth {
                log::info!("Delete account");
                match crate::auth::delete_account(&context.client, auth_context)
                    .await
                {
                    | Ok(_) => {
                        log::info!("Delete account success");
                        // NOTE: Reset auth context
                        context.auth = None;
                        // NOTE: Navigate to home
                        navigation.push(Route::Home {});
                    },
                    | Err(error) => {
                        log::error!("Delete account failed: {:?}", error);
                    },
                }
            }
        }
    });
}
