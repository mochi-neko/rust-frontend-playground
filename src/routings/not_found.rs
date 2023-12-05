use dioxus::{
    html::GlobalAttributes,
    prelude::{dioxus_elements, inline_props, render, Element, Props, Scope},
};

#[allow(non_snake_case)]
#[inline_props]
pub(crate) fn NotFound(
    cx: Scope,
    route: Vec<String>,
) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
