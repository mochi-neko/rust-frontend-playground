use dioxus::{
    hooks::use_shared_state,
    prelude::{
        dioxus_elements, fc_to_builder, inline_props, render, Element, Props,
        Scope,
    },
};
use dioxus_router::hooks::use_navigator;
use material_dioxus::button::MatButton;

use crate::{auth::auth_context::AuthContext, routings::route::Route};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn Dashboard(cx: Scope) -> Element {
    let auth_context = use_shared_state::<Option<AuthContext>>(cx).unwrap();
    let navigation = use_navigator(cx);

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
                MatButton{
                    label: "Sign out",
                    outlined: true,
                    disabled: auth_context.read().is_none(),
                }
            }
        }
    }
}
