//! Configuration for the Firebase Auth.
use crate::error::Error;
use crate::result::Result;
use crate::session::{AuthSession, Tokens};

/// Configuration for the Firebase Auth.
#[derive(Clone)]
pub struct AuthConfig {
    /// Firebase project API key.
    api_key: String,
    /// Timeout options for HTTP client.
    timeout: Timeout,
}

/// Timeout options for HTTP client.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timeout {
    /// Connection timeout duration.
    pub connection_timeout: std::time::Duration,
    /// Request timeout duration.
    pub request_timeout: std::time::Duration,
}

impl Default for Timeout {
    fn default() -> Self {
        Self {
            connection_timeout: std::time::Duration::from_secs(10),
            request_timeout: std::time::Duration::from_secs(60),
        }
    }
}

impl AuthConfig {
    /// Creates a new [`AuthConfig`] instance.
    ///
    /// ## Arguments
    /// - `api_key` - Your Firebase project API key.
    /// - `timeout` - Timeout options for HTTP client.
    ///
    /// ## Returns
    /// The [`AuthConfig`] instance.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// // Do something with config.
    /// ```
    pub fn new(
        api_key: String,
        timeout: Option<Timeout>,
    ) -> Self {
        Self {
            api_key,
            timeout: timeout.unwrap_or_default(),
        }
    }

    /// Builds a new HTTP client from config.
    fn build_client(&self) -> Result<reqwest::Client> {
        reqwest::ClientBuilder::new()
            .connect_timeout(
                self.timeout
                    .connection_timeout,
            )
            .timeout(self.timeout.request_timeout)
            .build()
            .map_err(|error| Error::HttpClientBuildError(error))
    }

    /// Signs up a new user with the given email and password.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to sign up.
    /// - `password` - The password of the user to sign up.
    ///
    /// ## Returns
    /// The session for the signed up user.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// let session = config.sign_up_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_up_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_up_with_email_password::SignUpWithEmailPasswordRequestBodyPayload::new(email, password);

        // Send request.
        let response_payload =
        crate::api::sign_up_with_email_password::sign_up_with_email_password(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create tokens.
        let tokens = Tokens {
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        };

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            tokens,
        })
    }

    /// Signs in a user with the given email and password.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to sign in.
    /// - `password` - The password of the user to sign in.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_in_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_in_with_email_password::SignInWithEmailPasswordRequestBodyPayload::new(email, password);

        // Send request.
        let response_payload =
        crate::api::sign_in_with_email_password::sign_in_with_email_password(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create tokens.
        let tokens = Tokens {
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        };

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            tokens,
        })
    }

    /// Signs in as an anonymous user.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// let session = config.sign_in_anonymously().await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_in_anonymously(&self) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_in_anonymously::SignInAnonymouslyRequestBodyPayload::new();

        // Send request.
        let response_payload =
            crate::api::sign_in_anonymously::sign_in_anonymously(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create tokens.
        let tokens = Tokens {
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        };

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            tokens,
        })
    }

    /// Signs in a user with the given OAuth credential.
    ///
    /// ## Arguments
    /// - `request_uri` - The URI to which the IDP redirects the user back.
    /// - `post_body` - The POST body passed to the IDP containing the OAuth credential and provider ID.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    /// use firebase_auth_rs::api::sign_in_with_oauth_credential::IdpPostBody;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// let session = config.sign_in_oauth_credencial(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-oauth-open-id-token".to_string(),
    ///     },
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_in_oauth_credencial(
        &self,
        request_uri: String,
        post_body: crate::api::sign_in_with_oauth_credential::IdpPostBody,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_in_with_oauth_credential::SignInWithOAuthCredentialRequestBodyPayload::new(
                request_uri,
                post_body,
                false,
            );

        // Send request.
        let response_payload =
            crate::api::sign_in_with_oauth_credential::sign_in_with_oauth_credential(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create tokens.
        let tokens = Tokens {
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        };

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            tokens,
        })
    }

    /// Exchanges a refresh token for an ID token and new refresh token.
    ///
    /// ## Arguments
    /// - `refresh_token` - A Firebase Auth refresh token.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// let session = config.exchange_refresh_tokens(
    ///     "user-firebase-refresh-token".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn exchange_refresh_tokens(
        &self,
        refresh_token: String,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload = crate::api::exchange_refresh_token::ExchangeRefreshTokenRequestBodyPayload::new(
            refresh_token,
        );

        // Send request.
        let response_payload =
            crate::api::exchange_refresh_token::exchange_refresh_token(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create tokens.
        let tokens = Tokens {
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        };

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            tokens,
        })
    }

    /// Fetches the list of all IDPs for the specified email.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to fetch providers.
    /// - `continue_uri` - The URI to which the IDP redirects the user back.
    ///
    /// ## Returns
    /// The list of all IDPs for the specified email.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// let providers = config.fetch_providers_for_email(
    ///     "user@example".to_string(),
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with providers.
    /// ```
    pub async fn fetch_providers_for_email(
        &self,
        email: String,
        continue_uri: String,
    ) -> Result<Vec<String>> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
        crate::api::fetch_providers_for_email::FetchProvidersForEmailRequestBodyPayload::new(
            email,
            continue_uri,
        );

        // Send request.
        let response_payload =
            crate::api::fetch_providers_for_email::fetch_providers_for_email(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        Ok(response_payload.all_providers)
    }

    /// Sends a password reset email to the given email address.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to send password reset email.
    /// - `locale` - The optional language code corresponding to the user's locale.
    ///
    /// ## Example
    /// ```
    /// use firebase_auth_rs::auth::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    ///     None,
    /// );
    ///
    /// config.send_reset_password_email(
    ///     "user@example".to_string(),
    ///     None,
    /// ).await.unwrap();
    ///
    /// // Do something.
    /// ```
    pub async fn send_reset_password_email(
        &self,
        email: String,
        locale: Option<String>,
    ) -> Result<()> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::send_password_reset_email::SendPasswordResetEmailRequestBodyPayload::new(email);

        // Send request.
        crate::api::send_password_reset_email::send_password_reset_email(
            &client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }
}
