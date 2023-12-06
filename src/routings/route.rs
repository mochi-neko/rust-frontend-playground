use dioxus::prelude::{fc_to_builder, render};
use dioxus_router::prelude::{Routable, ToRouteSegments};

use super::{
    dashboard::Dashboard, home::Home, not_found::NotFound, sign_in::SignIn,
    sign_up::SignUp,
};

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/signup")]
    SignUp {},
    #[route("/signin")]
    SignIn {},
    #[route("/dashboard")]
    Dashboard {},

    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}