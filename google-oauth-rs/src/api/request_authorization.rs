//! Set authorization parameters to request authentification code.
//! See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#creatingclient)
use std::collections::HashMap;

use reqwest::Url;

use crate::error::Error;
use crate::result::Result;

const AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";

/// Scope of access.
pub enum Scope {
    /// OpenID.
    OpenID,
    /// Email.
    Email,
    /// Profile.
    Profile,
}

impl Scope {
    pub fn to_parameter(&self) -> &str {
        match self {
            | Scope::OpenID => "openid",
            | Scope::Email => "https://www.googleapis.com/auth/userinfo.email",
            | Scope::Profile => {
                "https://www.googleapis.com/auth/userinfo.profile"
            },
        }
    }

    pub fn from_string(scope: &str) -> Vec<Scope> {
        scope
            .split(" ")
            .map(|scope| match scope {
                | "openid" => Scope::OpenID,
                | "https://www.googleapis.com/auth/userinfo.email" => {
                    Scope::Email
                },
                | "https://www.googleapis.com/auth/userinfo.profile" => {
                    Scope::Profile
                },
                | _ => panic!("Invalid scope: {}", scope),
            })
            .collect::<Vec<Scope>>()
    }
}

/// Token access type.
pub enum AccessType {
    /// Refreshes access token when user is present at the browser.
    Online,
    /// Refreshes access token when user is NOT present at the browser.
    Offline,
}

impl AccessType {
    pub fn to_parameter(&self) -> &str {
        match self {
            | AccessType::Online => "online",
            | AccessType::Offline => "offline",
        }
    }
}

/// Consent prompt.
pub enum Prompt {
    /// Do not display any authentication or consent screens.
    /// Must not be specified with other values.
    None,
    /// Prompt the user for consent.
    Consent,
    /// Prompt the user to select an account.
    SelectAccount,
}

impl Prompt {
    pub fn to_parameter(&self) -> &str {
        match self {
            | Prompt::None => "none",
            | Prompt::Consent => "consent",
            | Prompt::SelectAccount => "select_account",
        }
    }
}

/// Response type.
pub enum ResponseType {
    /// Authorization code.
    Code,
}

impl ResponseType {
    pub fn to_parameter(&self) -> &str {
        match self {
            | ResponseType::Code => "code",
        }
    }
}

/// Parameters to request authorization to auth server.
/// See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#creatingclient).
pub struct AuthorizationRequestParameters {
    /// The client ID for your application.
    /// You can find this value in the API Console Credentials page.
    pub client_id: String,
    /// Determines where the API server redirects the user after the user completes the authorization flow.
    /// The value must exactly match one of the authorized redirect URIs for the OAuth 2.0 client, which you configured in your client's API Console Credentials page.
    /// If this value doesn't match an authorized redirect URI for the provided client_id you will get a redirect_uri_mismatch error.
    pub redirect_uri: String,
    /// Determines whether the Google OAuth 2.0 endpoint returns an authorization code.
    /// Set the parameter value to code for web server applications.
    pub response_type: ResponseType,
    /// A space-delimited list of scopes that identify the resources that your application could access on the user's behalf.
    /// These values inform the consent screen that Google displays to the user.
    /// Scopes enable your application to only request access to the resources that it needs while also enabling users to control the amount of access that they grant to your application.
    /// Thus, there is an inverse relationship between the number of scopes requested and the likelihood of obtaining user consent.
    /// We recommend that your application request access to authorization scopes in context whenever possible.
    /// By requesting access to user data in context, via incremental authorization, you help users to more easily understand why your application needs the access it is requesting.
    pub scope: Vec<Scope>,
    /// Indicates whether your application can refresh access tokens when the user is not present at the browser.
    /// Valid parameter values are online, which is the default value, and offline.
    /// Set the value to offline if your application needs to refresh access tokens when the user is not present at the browser.
    /// This is the method of refreshing access tokens described later in this document.
    /// This value instructs the Google authorization server to return a refresh token and an access token the first time that your application exchanges an authorization code for tokens.
    pub access_type: Option<AccessType>,
    /// Specifies any string value that your application uses to maintain state between your authorization request and the authorization server's response.
    /// The server returns the exact value that you send as a name=value pair in the URL query component (?) of the redirect_uri after the user consents to or denies your application's access request.
    /// You can use this parameter for several purposes, such as directing the user to the correct resource in your application, sending nonces, and mitigating cross-site request forgery.
    /// Since your redirect_uri can be guessed, using a state value can increase your assurance that an incoming connection is the result of an authentication request.
    /// If you generate a random string or encode the hash of a cookie or another value that captures the client's state, you can validate the response to additionally ensure that the request and response originated in the same browser, providing protection against attacks such as cross-site request forgery.
    /// See the OpenID Connect documentation for an example of how to create and confirm a state token.
    pub state: Option<String>,
    /// Enables applications to use incremental authorization to request access to additional scopes in context.
    /// If you set this parameter's value to true and the authorization request is granted, then the new access token will also cover any scopes to which the user previously granted the application access.
    /// See the incremental authorization section for examples.
    pub include_granted_scopes: Option<bool>,
    /// Defaults to true. If set to false, more granular Google Account permissions will be disabled for OAuth client IDs created before 2019.
    /// No effect for newer OAuth client IDs, since more granular permissions is always enabled for them.
    pub enable_granular_consent: Option<bool>,
    /// If your application knows which user is trying to authenticate, it can use this parameter to provide a hint to the Google Authentication Server.
    /// The server uses the hint to simplify the login flow either by prefilling the email field in the sign-in form or by selecting the appropriate multi-login session.
    /// Set the parameter value to an email address or sub identifier, which is equivalent to the user's Google ID.
    pub login_hint: Option<String>,
    /// A space-delimited, case-sensitive list of prompts to present the user.
    /// If you don't specify this parameter, the user will be prompted only the first time your project requests access.
    /// See Prompting re-consent for more information.
    pub prompt: Option<Prompt>,
}

impl AuthorizationRequestParameters {
    fn build_query(self) -> HashMap<&'static str, String> {
        let mut query = HashMap::new();

        // Required
        query.insert("client_id", self.client_id);
        query.insert("redirect_uri", self.redirect_uri);
        query.insert(
            "scope",
            self.scope
                .iter()
                .map(|scope| scope.to_parameter())
                .collect::<Vec<&str>>()
                .join(" "),
        );
        query.insert(
            "response_type",
            self.response_type
                .to_parameter()
                .to_string(),
        );

        // Optional
        if let Some(access_type) = self.access_type {
            query.insert(
                "access_type",
                access_type
                    .to_parameter()
                    .to_string(),
            );
        }
        if let Some(state) = self.state {
            query.insert("state", state);
        }
        if let Some(include_granted_scopes) = self.include_granted_scopes {
            query.insert(
                "include_granted_scopes",
                include_granted_scopes.to_string(),
            );
        }
        if let Some(enable_granular_consent) = self.enable_granular_consent {
            query.insert(
                "enable_granular_consent",
                enable_granular_consent.to_string(),
            );
        }
        if let Some(login_hint) = self.login_hint {
            query.insert("login_hint", login_hint);
        }
        if let Some(prompt) = self.prompt {
            query.insert(
                "prompt",
                prompt
                    .to_parameter()
                    .to_string(),
            );
        }

        query
    }

    /// Builds redirect URI from request parameters.
    pub fn build_redirect_uri(self) -> Result<Url> {
        let client = reqwest::Client::new();

        let url = client
            .get(AUTH_ENDPOINT)
            .query(&self.build_query())
            .build()
            .map_err(|error| Error::RequestBuildError(error))?
            .url()
            .clone();

        Ok(url)
    }
}

/// Response query parameters in redirect URI from auth server.
/// See also [reference](https://developers.google.com/identity/protocols/oauth2/web-server#handlingresponse).
pub struct AuthorizationRedirectResponseQuery {
    /// Authentication code.
    pub code: String,
    /// Scopes to access.
    pub scope: String,
    /// Authorized user index.
    pub authuser: usize,
    /// Consent prompt.
    pub prompt: String,
    /// Identification of request.
    pub state: Option<String>,
}

impl AuthorizationRedirectResponseQuery {
    /// Parses query parameters from redirect URI.
    pub fn from_query(query: &str) -> Self {
        let query_parameters = parse_query_str(query);

        AuthorizationRedirectResponseQuery {
            code: query_parameters
                .get("code")
                .cloned()
                .unwrap_or_default(),
            scope: query_parameters
                .get("scope")
                .cloned()
                .unwrap_or_default(),
            authuser: query_parameters
                .get("authuser")
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
            prompt: query_parameters
                .get("prompt")
                .cloned()
                .unwrap_or_default(),
            state: query_parameters
                .get("state")
                .cloned(),
        }
    }
}

fn parse_query_str(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for param in query.split("&") {
        let mut param = param.split("=");
        let key = param
            .next()
            .unwrap_or_default();
        let value = param
            .next()
            .unwrap_or_default();
        params.insert(key.to_string(), value.to_string());
    }

    params
}
