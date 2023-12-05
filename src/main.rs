mod auth;
mod firebase;
mod generated;
mod logging;
mod routings;
mod style;

use dioxus::prelude::{fc_to_builder, render, Element, Scope};
use dioxus_router::prelude::Router;

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<crate::routings::route::Route> {}
    }
}
