use dioxus::{
    hooks::use_state,
    prelude::{
        dioxus_elements, fc_to_builder, inline_props, render, use_future,
        use_shared_state, Element, Props, Scope,
    },
};
use dioxus_router::hooks::use_navigator;
use firebase_rust::auth::get_user_data::GetUserDataResponsePayload;
use material_dioxus::button::MatButton;

use crate::{
    auth::{
        auth_context::AuthContext,
        get_user_data::{get_user_data, GetUserDataInfo},
    },
    routings::route::Route,
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn Dashboard(cx: Scope) -> Element {
    let auth_context = use_shared_state::<Option<AuthContext>>(cx).unwrap();
    let navigation = use_navigator(cx);

    let user_info =
        use_state::<Option<GetUserDataResponsePayload>>(cx, || None);
    let get_user_data = use_future(cx, (), |_| {
        let auth_context = auth_context.clone();
        let user_info = user_info.clone();

        async move {
            if auth_context.read().is_none() {
                return;
            }
            let auth_context = auth_context.read();
            let info: GetUserDataInfo = GetUserDataInfo {
                id_token: auth_context
                    .as_ref()
                    .unwrap()
                    .id_token
                    .clone(),
            };

            log::info!("Get user data");
            let result = get_user_data(&info).await;
            match result {
                | Ok(data) => {
                    log::info!("Get user data success");
                    user_info.set(Some(data));
                },
                | Err(error) => {
                    log::error!("Get user data failed: {:?}", error);
                    user_info.set(None);
                },
            }
        }
    });

    if auth_context.read().is_none() {
        navigation.push(Route::Home {});
    }

    render! {
        h1 { "Dashboard" }

        div {
            span {
                onclick: move |_| {
                    if auth_context.read().is_none()
                    {
                        return;
                    }

                    log::info!("Sign out");

                    // NOTE: Reset auth context
                    let mut context = auth_context.write();
                    *context = None;

                    // NOTE: Navigate to home
                    navigation.push(Route::Home {});
                },
                MatButton {
                    label: "Sign out",
                    outlined: true,
                    disabled: auth_context.read().is_none(),
                }
            }
        }

        div {
            span {
                onclick: move |_| {
                    log::info!("Update user data");
                    get_user_data.restart();
                },
                MatButton{
                    label: "Update user data",
                    outlined: true,
                }
            }
        }

        if let Some(user_info) = user_info.get().as_ref() {
            if let Some(user) = user_info.users.get(0) {
                render! {
                    div {
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

                        // TODO: provider user info
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
                    }
                }
            }
            else {
                render! {
                    div {
                        "User not found"
                    }
                }
            }
        }
    }
}
