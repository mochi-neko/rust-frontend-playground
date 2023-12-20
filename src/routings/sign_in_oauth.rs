use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, Element, Props, Scope,
};
use dioxus_router::hooks::use_navigator;
use material_dioxus::MatButton;

use crate::routings::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignInWithOAuth(cx: Scope) -> Element {
    render! {
        h1 { "Sign in with OAuth" }

        div {
            MatButton {
                label: "Sign in with Google",
                outlined: true,
                icon: "/resources/google/web_dark_sq_SI@1x.png",
                _onclick: |_| {
                    log::info!("Sign in with Google");
                },
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    let navigator = use_navigator(cx).clone();
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
