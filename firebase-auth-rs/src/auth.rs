//! Provides an interface for authenticating users with Firebase Auth.
use crate::error::Error;
use crate::result::Result;

/// Authentication state for a user of Firebase Auth.
pub struct Auth {
    /// HTTP client.
    client: reqwest::Client,
    /// Firebase project API key.
    api_key: String,
    /// Firebase Auth tokens.
    tokens: Tokens,
}

/// Tokens returned by the Firebase Auth API.
struct Tokens {
    /// Firebase Auth ID token.
    id_token: String,
    /// The number of seconds in which the ID token expires.
    expires_in: u64,
    /// Firebase Auth refresh token.
    refresh_token: String,
}

/// User data.
pub struct UserData {
    /// The uid of the account.
    pub local_id: String,
    /// The email of the account.
    pub email: String,
    /// Whether or not the account's email has been verified.
    pub email_verified: bool,
    /// The display name for the account.
    pub display_name: Option<String>,
    /// The photo url of the account.
    pub photo_url: Option<String>,
    /// The timestamp, in seconds, which marks a boundary, before which Firebase ID token are considered revoked.
    pub valid_since: String,
    /// Whether the account is disabled or not.
    pub disabled: bool,
    /// The timestamp, in milliseconds, that the account last logged in at.
    pub last_login_at: String,
    /// The timestamp, in milliseconds, that the account was created at.
    pub created_at: String,
}

/// Timeout options for HTTP client.
pub struct Timeout {
    /// Connection timeout duration.
    connection_timeout: std::time::Duration,
    /// Request timeout duration.
    request_timeout: std::time::Duration,
}

impl Default for Timeout {
    fn default() -> Self {
        Self {
            connection_timeout: std::time::Duration::from_secs(10),
            request_timeout: std::time::Duration::from_secs(60),
        }
    }
}

/// Calls an API with refreshing tokens then return value with new Auth.
macro_rules! call_api_with_refreshing_tokens_with_return_value {
    // Has arguments and return value with Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr), *) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call(&auth, $($api_call_args), *).await {
                    Ok(result) => return Ok((auth, result)),
                    Err(error) => match error {
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return value with Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr,) => {{
        call_api_with_refreshing_tokens_with_return_value!($auth, $api_call, $retry_count, ())
    }};
}

/// Calls an API with refreshing tokens then return not value with new Auth.
macro_rules! call_api_with_refreshing_tokens_without_return_value {
    // Has arguments and return only Auth.
    ($auth:expr, $api_call_unit:expr, $retry_count:expr, $($api_call_args:expr), *) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call_unit(&auth, $($api_call_args), *).await {
                    Ok(_) => return Ok(auth),
                    Err(error) => match error {
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return only Auth.
    ($auth:expr, $api_call_unit:expr, $retry_count:expr,) => {{
        call_api_with_refreshing_tokens_without_return_value!($auth, $api_call_unit, $retry_count, ())
    }};
}

/// Calls an API with refreshing tokens then return new Auth.
macro_rules! call_api_with_refreshing_tokens_with_return_auth {
    // Has arguments and return Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr),*) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call(&auth, $($api_call_args),*).await {
                    Ok(new_auth) => return Ok(new_auth),
                    Err(error) => match error {
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr) => {{
        call_api_with_refreshing_tokens_with_return_auth!($auth, $api_call, $retry_count, )
    }};
}

/// Calls an API with refreshing tokens then return no Auth.
macro_rules! call_api_with_refreshing_tokens_without_auth {
    // Has arguments and return no Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr),*) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call(&auth, $($api_call_args),*).await {
                    Ok(_) => return Ok(()),
                    Err(error) => match error {
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return no Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr) => {{
        call_api_with_refreshing_tokens_without_auth!($auth, $api_call, $retry_count, )
    }};
}

/// Implements internal API callings for `Auth`.
impl Auth {
    async fn fetch_providers_for_email_internal(
        &self,
        email: String,
        continue_uri: String,
    ) -> Result<Vec<String>> {
        // Create request payload.
        let request_payload =
            crate::api::fetch_providers_for_email::FetchProvidersForEmailRequestBodyPayload::new(
                email,
                continue_uri,
            );

        // Send request.
        let response =
            crate::api::fetch_providers_for_email::fetch_providers_for_email(
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        Ok(response.all_providers)
    }

    async fn change_email_internal(
        &self,
        new_email: String,
        locale: Option<String>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::change_email::ChangeEmailRequestBodyPayload::new(
                self.tokens.id_token.clone(),
                new_email,
                false,
            );

        // Send request.
        crate::api::change_email::change_email(
            &self.client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }

    async fn change_password_internal(
        &self,
        new_password: String,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::change_password::ChangePasswordRequestBodyPayload::new(
                self.tokens.id_token.clone(),
                new_password,
                false,
            );

        // Send request.
        crate::api::change_password::change_password(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn send_reset_password_email_internal(
        &self,
        email: String,
        locale: Option<String>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::send_password_reset_email::SendPasswordResetEmailRequestBodyPayload::new(email);

        // Send request.
        crate::api::send_password_reset_email::send_password_reset_email(
            &self.client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }

    async fn update_profile_internal(
        &self,
        display_name: String,
        photo_url: String,
        delete_attribute: Vec<crate::api::update_profile::DeleteAttribute>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::update_profile::UpdateProfileRequestBodyPayload::new(
                self.tokens.id_token.clone(),
                display_name,
                photo_url,
                delete_attribute,
                false,
            );

        // Send request.
        crate::api::update_profile::update_profile(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn get_user_data_internal(&self) -> Result<UserData> {
        // Create request payload.
        let request_payload =
            crate::api::get_user_data::GetUserDataRequestBodyPayload::new(
                self.tokens.id_token.clone(),
            );

        // Send request.
        let response = crate::api::get_user_data::get_user_data(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Take the first user from vector.
        let user = response
            .users
            .get(0)
            .ok_or(Error::NotFoundAnyUserData)?;

        Ok(UserData {
            local_id: user.local_id.clone(),
            email: user.email.clone(),
            email_verified: user.email_verified,
            display_name: user.display_name.clone(),
            photo_url: user.photo_url.clone(),
            valid_since: user.valid_since.clone(),
            disabled: user.disabled.unwrap_or(false),
            last_login_at: user.last_login_at.clone(),
            created_at: user.created_at.clone(),
        })
    }

    async fn link_with_email_password_internal(
        &self,
        email: String,
        password: String,
    ) -> Result<Self> {
        // Create request payload.
        let request_payload =
            crate::api::sign_in_with_email_password::SignInWithEmailPasswordRequestBodyPayload::new(
                email,
                password,
            );

        // Send request.
        let response_payload =
            crate::api::sign_in_with_email_password::sign_in_with_email_password(
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Update tokens.
        Ok(Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            tokens: Tokens {
                id_token: response_payload.id_token,
                expires_in: response_payload
                    .expires_in
                    .parse()
                    .map_err(|error| Error::NumberParseError {
                        error,
                    })?,
                refresh_token: response_payload.refresh_token,
            },
        })
    }

    async fn link_with_oauth_credential_internal(
        &self,
        request_uri: String,
        post_body: crate::api::sign_in_with_oauth_credential::IdpPostBody,
    ) -> Result<Self> {
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
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Update tokens.
        Ok(Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            tokens: Tokens {
                id_token: response_payload.id_token,
                expires_in: response_payload
                    .expires_in
                    .parse()
                    .map_err(|error| Error::NumberParseError {
                        error,
                    })?,
                refresh_token: response_payload.refresh_token,
            },
        })
    }

    async fn unlink_provider_internal(
        &self,
        id_token: String,
        delete_provider: Vec<String>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::unlink_provider::UnlinkProviderRequestBodyPayload::new(
                id_token,
                delete_provider,
            );

        // Send request.
        crate::api::unlink_provider::unlink_provider(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn send_email_verification_internal(
        &self,
        locale: Option<String>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::send_email_verification::SendEmailVerificationRequestBodyPayload::new(
                self.tokens.id_token.clone(),
            );

        // Send request.
        crate::api::send_email_verification::send_email_verification(
            &self.client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }

    async fn delete_account_internal(&self) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::delete_account::DeleteAccountRequestBodyPayload::new(
                self.tokens.id_token.clone(),
            );

        // Send request.
        crate::api::delete_account::delete_account(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }
}

/// Implements builder functions for `Auth`.
impl Auth {
    pub async fn sign_up_with_email_password(
        api_key: String,
        email: String,
        password: String,
        timeout: Option<Timeout>,
    ) -> Result<Self> {
        // Create a shared HTTP client.
        let timeout = timeout.unwrap_or_default();
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(timeout.connection_timeout)
            .timeout(timeout.request_timeout)
            .build()
            .unwrap();

        // Create request payload.
        let request_payload =
            crate::api::sign_up_with_email_password::SignUpWithEmailPasswordRequestBodyPayload::new(email, password);

        // Send request.
        let response_payload =
            crate::api::sign_up_with_email_password::sign_up_with_email_password(
                &client,
                &api_key,
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

        // Create auth.
        Ok(Self {
            client,
            api_key,
            tokens,
        })
    }

    pub async fn sign_in_with_email_password(
        api_key: String,
        email: String,
        password: String,
        timeout: Option<Timeout>,
    ) -> Result<Self> {
        // Create a shared HTTP client.
        let timeout = timeout.unwrap_or_default();
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(timeout.connection_timeout)
            .timeout(timeout.request_timeout)
            .build()
            .unwrap();

        // Create request payload.
        let request_payload =
            crate::api::sign_in_with_email_password::SignInWithEmailPasswordRequestBodyPayload::new(email, password);

        // Send request.
        let response_payload =
            crate::api::sign_in_with_email_password::sign_in_with_email_password(
                &client,
                &api_key,
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

        // Create auth.
        Ok(Self {
            client,
            api_key,
            tokens,
        })
    }

    pub async fn sign_in_anonymously(
        api_key: String,
        timeout: Option<Timeout>,
    ) -> Result<Self> {
        // Create a shared HTTP client.
        let timeout = timeout.unwrap_or_default();
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(timeout.connection_timeout)
            .timeout(timeout.request_timeout)
            .build()
            .unwrap();

        // Create request payload.
        let request_payload =
            crate::api::sign_in_anonymously::SignInAnonymouslyRequestBodyPayload::new();

        // Send request.
        let response_payload =
            crate::api::sign_in_anonymously::sign_in_anonymously(
                &client,
                &api_key,
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

        // Create auth.
        Ok(Self {
            client,
            api_key,
            tokens,
        })
    }

    pub async fn sign_in_oauth_credencial(
        api_key: String,
        request_uri: String,
        post_body: crate::api::sign_in_with_oauth_credential::IdpPostBody,
        timeout: Option<Timeout>,
    ) -> Result<Self> {
        // Create a shared HTTP client.
        let timeout = timeout.unwrap_or_default();
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(timeout.connection_timeout)
            .timeout(timeout.request_timeout)
            .build()
            .unwrap();

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
                &api_key,
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

        Ok(Self {
            client,
            api_key,
            tokens,
        })
    }
}

/// Implements public API callings for `Auth`.
impl Auth {
    pub async fn refresh_tokens(self) -> Result<Self> {
        // Create request payload.
        let request_payload = crate::api::exchange_refresh_token::ExchangeRefreshTokenRequestBodyPayload::new(
            self.tokens
                .refresh_token
                .clone(),
        );

        // Send request.
        let response =
            crate::api::exchange_refresh_token::exchange_refresh_token(
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Update tokens.
        Ok(Self {
            client: self.client,
            api_key: self.api_key,
            tokens: Tokens {
                id_token: response.id_token,
                expires_in: response
                    .expires_in
                    .parse()
                    .map_err(|error| Error::NumberParseError {
                        error,
                    })?,
                refresh_token: response.refresh_token,
            },
        })
    }

    pub async fn fetch_providers_for_email(
        self,
        email: String,
        continue_uri: String,
    ) -> Result<(Auth, Vec<String>)> {
        call_api_with_refreshing_tokens_with_return_value!(
            self,
            Auth::fetch_providers_for_email_internal,
            1,
            email.clone(),
            continue_uri.clone()
        )
        .await
    }

    pub async fn send_password_reset_email(
        self,
        email: String,
        locale: Option<String>,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            Auth::send_reset_password_email_internal,
            1,
            email.clone(),
            locale.clone()
        )
        .await
    }

    pub async fn change_email(
        self,
        new_email: String,
        locale: Option<String>,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            Auth::change_email_internal,
            1,
            new_email.clone(),
            locale.clone()
        )
        .await
    }

    pub async fn change_password(
        self,
        new_password: String,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            Auth::change_password_internal,
            1,
            new_password.clone()
        )
        .await
    }

    pub async fn update_profile(
        self,
        display_name: String,
        photo_url: String,
        delete_attribute: Vec<crate::api::update_profile::DeleteAttribute>,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            Auth::update_profile_internal,
            1,
            display_name.clone(),
            photo_url.clone(),
            delete_attribute.clone()
        )
        .await
    }

    pub async fn get_user_data(self) -> Result<(Auth, UserData)> {
        call_api_with_refreshing_tokens_with_return_value!(
            self,
            Auth::get_user_data_internal,
            1,
        )
        .await
    }

    pub async fn link_with_email_password(
        self,
        email: String,
        password: String,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_with_return_auth!(
            self,
            Auth::link_with_email_password_internal,
            1,
            email.clone(),
            password.clone()
        )
        .await
    }

    pub async fn link_with_oauth_credential(
        self,
        request_uri: String,
        post_body: crate::api::sign_in_with_oauth_credential::IdpPostBody,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_with_return_auth!(
            self,
            Auth::link_with_oauth_credential_internal,
            1,
            request_uri.clone(),
            post_body.clone()
        )
        .await
    }

    pub async fn unlink_provider(
        self,
        id_token: String,
        delete_provider: Vec<String>,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            Auth::unlink_provider_internal,
            1,
            id_token.clone(),
            delete_provider.clone()
        )
        .await
    }

    pub async fn send_email_verification(
        self,
        locale: Option<String>,
    ) -> Result<Auth> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            Auth::send_email_verification_internal,
            1,
            locale.clone()
        )
        .await
    }

    pub async fn delete_account(self) -> Result<()> {
        call_api_with_refreshing_tokens_without_auth!(
            self,
            Auth::delete_account_internal,
            1,
        )
        .await
    }
}
