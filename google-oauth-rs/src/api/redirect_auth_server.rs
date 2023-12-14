use std::collections::HashMap;

use reqwest::Url;

use crate::error::Error;
use crate::result::Result;

pub enum Scope {
    Email,
    Profile,
}

impl Scope {
    pub fn to_parameter(&self) -> &str {
        match self {
            | Scope::Email => "https://www.googleapis.com/auth/userinfo.email",
            | Scope::Profile => {
                "https://www.googleapis.com/auth/userinfo.profile"
            },
        }
    }
}

pub enum AccessType {
    Online,
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

pub enum Prompt {
    None,
    Consent,
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

pub enum ResponseType {
    Code,
}

impl ResponseType {
    pub fn to_parameter(&self) -> &str {
        match self {
            | ResponseType::Code => "code",
        }
    }
}

/// https://developers.google.com/identity/protocols/oauth2/web-server?hl=ja#creatingclient
pub struct RedirectToAuthServerRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Vec<Scope>,
    pub response_type: ResponseType,
    pub access_type: Option<AccessType>,
    pub state: Option<String>,
    pub include_granted_scopes: Option<bool>,
    pub enable_granular_consent: Option<bool>,
    pub login_hint: Option<String>,
    pub prompt: Option<Prompt>,
}

impl RedirectToAuthServerRequest {
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

    pub fn build_redirect_uri(self) -> Result<Url> {
        let endpoint = "https://accounts.google.com/o/oauth2/v2/auth";
        let client = reqwest::Client::new();

        let url = client
            .get(endpoint)
            .query(&self.build_query())
            .build()
            .map_err(|error| Error::RequestBuildError(error))?
            .url()
            .clone();

        Ok(url)
    }
}
