mod auth;
mod generated;
mod logging;
mod routings;
mod style;

use dioxus::{
    hooks::use_shared_state_provider,
    prelude::{
        dioxus_elements, fc_to_builder, render, Element, GlobalAttributes,
        Scope,
    },
};
use dioxus_router::prelude::Router;
use material_dioxus::MatTheme;

use crate::auth::auth_context::AuthContext;

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<Option<AuthContext>>(cx, || None);

    render! {
        // NOTE: Failed to load style.css then use inline style
        style {
            dangerous_inner_html: crate::style::STYLE_CSS,
        }

        MatTheme { }

        Router::<crate::routings::route::Route> {}
    }
}
