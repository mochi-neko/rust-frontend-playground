use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, Element, Props, Scope,
};
use dioxus_router::hooks::use_navigator;
use material_dioxus::MatButton;

use super::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Home" }

        div {
            span {
                onclick: |_| {
                    let navigator = use_navigator(cx).clone();
                    navigator.push(Route::SignUp { });
                },
                MatButton {
                    label: "Sign up with email",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    let navigator = use_navigator(cx).clone();
                    navigator.push(Route::SignIn { });
                },
                MatButton {
                    label: "Sign in with email",
                    outlined: true,
                }
            }
        }

        br {}

        div {
            span {
                onclick: |_| {
                    let navigator = use_navigator(cx).clone();
                    navigator.push(Route::SignInWithOAuth { });
                },
                MatButton {
                    label: "Sign in with OAuth",
                    outlined: true,
                }
            }
        }
    }
}
