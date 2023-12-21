use dioxus::{
    html::GlobalAttributes,
    prelude::{component, dioxus_elements, render, Element, Props, Scope},
};

#[allow(non_snake_case)]
#[component(no_case_check)]
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
