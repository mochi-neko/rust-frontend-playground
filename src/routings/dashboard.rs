use dioxus::prelude::{
    dioxus_elements, inline_props, render, Element, Props, Scope,
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn Dashboard(cx: Scope) -> Element {
    render! {
        h1 { "Dashboard" }
    }
}
