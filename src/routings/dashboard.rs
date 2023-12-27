use dioxus::prelude::{
    component, dioxus_elements, fc_to_builder, render, rsx, to_owned,
    use_future, use_shared_state, use_state, Element, IntoDynNode, Scope,
    Scoped, UseSharedState,
};
use dioxus_router::hooks::use_navigator;
use firebase_auth_rs::auth::{AuthSession, UserData};
use material_dioxus::{button::MatButton, text_inputs::MatTextField};

use crate::application_context::ApplicationContext;
use crate::routings::route::Route;

#[allow(non_snake_case)]
#[component(no_case_check)]
pub(crate) fn Dashboard(cx: Scope) -> Element {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx).unwrap();
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let confirm_password = use_state(cx, String::new);
    let display_name = use_state(cx, String::new);
    let photo_url = use_state(cx, String::new);

    let fetch_user_data = use_future(cx, (), move |_| {
        let context = context.clone();
        async move {
            let mut context = context.write();
            let session: Option<AuthSession> = context.auth_session.clone();
            match fetch_user_data_helper(session).await {
                | Some((new_session, user_data)) => {
                    context.auth_session = Some(new_session);
                    Some(user_data)
                },
                | None => None,
            }
        }
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
            | None => {
                render! {
                    div {
                        "Loading user data..."
                    }
                }
            },
            | Some(user_data) => {
                render_user_data(cx, user_data)
            },
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

fn render_user_data<'a>(
    cx: Scope<'a>,
    user_data: &Option<UserData>,
) -> Element<'a> {
    match user_data {
        | None => {
            render! {
                div {
                    "User data is not available"
                }
            }
        },
        | Some(user_data) => cx.render(rsx! {
            div {
                span {
                    onclick: |_| send_email_verification(cx),
                    MatButton {
                        label: "Send email verification",
                        outlined: true,
                        disabled: user_data.email_verified,
                    }
                }
            }

            h2 { "User info" }

            div {
                "Local ID: "
                span { user_data.local_id.clone() }
            }

            div {
                "E-mail: "
                span { user_data.email.clone() }
            }

            div {
                "E-mail verified: "
                span { user_data.email_verified.to_string() }
            }

            div {
                "Display name: "
                span { user_data.display_name.clone() }
            }

            div {
                "Provider user info: "
                for provider_user_info in user_data.provider_user_info.iter() {
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
                span { user_data.photo_url.clone() }
            }

            div {
                "Password hash: "
                span { "XXXX" }
            }

            div {
                "Password updated at: "
                span { user_data.password_updated_at.to_string() }
            }

            div {
                "Valid since: "
                span { user_data.valid_since.clone() }
            }

            div {
                "Disabled: "
                span { user_data.disabled.to_string() }
            }

            div {
                "Last login at: "
                span { user_data.last_login_at.clone() }
            }

            div {
                "Created at: "
                span { user_data.created_at.clone() }
            }

            div {
                "Custom auth: "
                span { user_data.custom_auth.unwrap_or(false).to_string() }
            }
        }),
    }
}

async fn fetch_user_data_helper(
    auth_option: Option<AuthSession>
) -> Option<(AuthSession, UserData)> {
    match auth_option {
        | Some(auth) => {
            log::info!("Get user data");
            match auth.get_user_data().await {
                | Ok((new_auth, user_data)) => {
                    log::info!("Get user data success");
                    Some((new_auth, user_data))
                },
                | Err(error) => {
                    log::error!("Get user data failed: {:?}", error);
                    None
                },
            }
        },
        | None => {
            log::error!("Auth context is not available");
            None
        },
    }
}

fn redirect_to_home(cx: &Scoped<'_>) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigation = use_navigator(cx).clone();

    if context
        .read()
        .auth_session
        .is_none()
    {
        // NOTE: Redirect to home
        log::info!("Redirect to home");
        navigation.push(Route::Home {});
    }
}

fn send_email_verification(cx: &Scoped<'_>) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let mut context = context.write();
            if let Some(sessoin) = &context.auth_session {
                log::info!("Send email verification");
                match sessoin
                    .clone()
                    .send_email_verification(None)
                    .await
                {
                    | Ok(new_session) => {
                        log::info!("Send email verification success");
                        context.auth_session = Some(new_session);
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
    cx: &Scoped<'_>,
    email: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let mut context = context.write();
            if let Some(session) = &context.auth_session {
                log::info!("Change email");
                match session
                    .clone()
                    .change_email(email, None)
                    .await
                {
                    | Ok(new_session) => {
                        log::info!("Change email success");
                        context.auth_session = Some(new_session);
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
    cx: &Scoped<'_>,
    password: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let mut context = context.write();
            if let Some(session) = &context.auth_session {
                log::info!("Change password");
                match session
                    .clone()
                    .change_password(password)
                    .await
                {
                    | Ok(new_session) => {
                        log::info!("Change password success");
                        context.auth_session = Some(new_session);
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
    cx: &Scoped<'_>,
    display_name: String,
    photo_url: String,
) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();

    cx.spawn({
        async move {
            let mut context = context.write();
            if let Some(session) = &context.auth_session {
                log::info!("Update profile");
                match session
                    .clone()
                    .update_profile(display_name, photo_url, vec![])
                    .await
                {
                    | Ok(new_session) => {
                        log::info!("Update profile success");
                        context.auth_session = Some(new_session);
                    },
                    | Err(error) => {
                        log::error!("Update profile failed: {:?}", error);
                    },
                }
            }
        }
    })
}

fn sign_out(cx: &Scoped<'_>) {
    // Setup hooks
    let context: UseSharedState<ApplicationContext> =
        use_shared_state::<ApplicationContext>(cx)
            .unwrap()
            .clone();
    let navigation = use_navigator(cx).clone();

    let mut context = context.write();
    if context.auth_session.is_some() {
        log::info!("Sign out");

        // NOTE: Reset auth session
        context.auth_session = None;

        // NOTE: Navigate to home
        navigation.push(Route::Home {});
    };
}

fn delete_account(cx: &Scoped<'_>) {
    // Setup hooks
    let context = use_shared_state::<ApplicationContext>(cx)
        .unwrap()
        .clone();
    let navigation = use_navigator(cx).clone();

    cx.spawn({
        async move {
            let mut context = context.write();
            if let Some(session) = &context.auth_session {
                log::info!("Delete account");
                match session
                    .clone()
                    .delete_account()
                    .await
                {
                    | Ok(_) => {
                        log::info!("Delete account success");
                        // NOTE: Reset auth context
                        context.auth_session = None;
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
