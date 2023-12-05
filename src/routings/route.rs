use dioxus::prelude::{fc_to_builder, render};
use dioxus_router::prelude::{Routable, ToRouteSegments};

use super::{not_found::NotFound, sign_up::SignUp};

#[derive(Routable, Clone)]
pub(crate) enum Route {
    #[route("/")]
    SignUp {},
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
