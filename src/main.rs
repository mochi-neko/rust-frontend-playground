mod auth;
mod firebase;
mod generated;
mod logging;
mod routings;
mod style;

use dioxus::prelude::{
    dioxus_elements, fc_to_builder, render, Element, GlobalAttributes, Scope,
};
use dioxus_router::prelude::Router;
use material_dioxus::MatTheme;

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    render! {
        style {
            // NOTE: Failed to load style.css then use inline style
            dangerous_inner_html: crate::style::STYLE_CSS,
        }

        MatTheme { }

        Router::<crate::routings::route::Route> {}
    }
}
