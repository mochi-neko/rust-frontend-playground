use dioxus::prelude::{fc_to_builder, render};
use dioxus_router::prelude::{Routable, ToRouteSegments};

use super::{
    dashboard::Dashboard,
    home::Home,
    not_found::NotFound,
    oauth_google::{
        OAuthGoogle, OAuthGoogleError, RedirectToAuthServerResponseErrorQuery,
        RedirectToAuthServerResponseQuery,
    },
    reset_password::ResetPassword,
    sign_in::SignIn,
    sign_in_oauth::SignInWithOAuth,
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
    #[route("/signin/oauth")]
    SignInWithOAuth {},
    #[route("/auth/google/redirect?:query")]
    OAuthGoogle {
        query: RedirectToAuthServerResponseQuery,
    },
    #[route("/auth/google/redirect?:error")]
    OAuthGoogleError {
        error: RedirectToAuthServerResponseErrorQuery,
    },
    #[route("/reset_password")]
    ResetPassword {},
    #[route("/dashboard")]
    Dashboard {},

    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
