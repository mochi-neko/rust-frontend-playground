use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, Element, Props, Scope,
};
use dioxus_router::components::Link;

use super::route::Route;

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Home" }

        div {
            Link {
                to: Route::SignUp {},
                "Sign up with email",
            }
        }

        div {
            Link {
                to: Route::SignIn {},
                "Sign in with email",
            }
        }

        div {
            Link {
                to: Route::SignInWithOAuth {},
                "Sign in with OAuth",
            }
        }
    }
}
