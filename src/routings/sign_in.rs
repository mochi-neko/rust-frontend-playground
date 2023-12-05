use dioxus::prelude::{
    dioxus_elements, inline_props, render, Element, Props, Scope,
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn SignIn(cx: Scope) -> Element {
    render! {
        h1 { "Sign in" }
    }
}
