mod application_context;
mod credential;
mod generated;
mod logging;
mod routings;
mod style;

use async_std::sync::Mutex;
use std::sync::Arc;

use dioxus::{
    hooks::use_shared_state_provider,
    prelude::{
        dioxus_elements, fc_to_builder, render, Element, GlobalAttributes,
        Scope,
    },
};
use dioxus_router::prelude::Router;
use material_dioxus::MatTheme;

use crate::application_context::ApplicationContext;

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<Arc<Mutex<ApplicationContext>>>(cx, || {
        Arc::new(Mutex::new(ApplicationContext::default()))
    });

    render! {
        // NOTE: Failed to load style.css then use inline style
        style {
            dangerous_inner_html: crate::style::STYLE_CSS,
        }

        MatTheme { }

        Router::<crate::routings::route::Route> {}
    }
}
