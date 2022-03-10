// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    pub(crate) client: aws_smithy_client::Client<C, M, R>,
    pub(crate) conf: crate::Config,
}

/// Client for AWS Single Sign-On
///
/// Client for invoking operations on AWS Single Sign-On. Each operation on AWS Single Sign-On is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the [`GetRoleCredentials`](crate::client::fluent_builders::GetRoleCredentials) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::GetRoleCredentials::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::GetRoleCredentials::set_role_name): <p>The friendly name of the role that is assigned to the user.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetRoleCredentials::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetRoleCredentials::set_account_id): <p>The identifier for the AWS account that is assigned to the user.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::GetRoleCredentials::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::GetRoleCredentials::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    /// - On success, responds with [`GetRoleCredentialsOutput`](crate::output::GetRoleCredentialsOutput) with field(s):
    ///   - [`role_credentials(Option<RoleCredentials>)`](crate::output::GetRoleCredentialsOutput::role_credentials): <p>The credentials for the role that is assigned to the user.</p>
    /// - On failure, responds with [`SdkError<GetRoleCredentialsError>`](crate::error::GetRoleCredentialsError)
    pub fn get_role_credentials(&self) -> fluent_builders::GetRoleCredentials<C, M, R> {
        fluent_builders::GetRoleCredentials::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListAccountRoles`](crate::client::fluent_builders::ListAccountRoles) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAccountRoles::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAccountRoles::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAccountRoles::set_next_token): <p>The page token from the previous response output when you request subsequent pages.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAccountRoles::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAccountRoles::set_max_results): <p>The number of items that clients can request per page.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::ListAccountRoles::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::ListAccountRoles::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListAccountRoles::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListAccountRoles::set_account_id): <p>The identifier for the AWS account that is assigned to the user.</p>
    /// - On success, responds with [`ListAccountRolesOutput`](crate::output::ListAccountRolesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListAccountRolesOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
    ///   - [`role_list(Option<Vec<RoleInfo>>)`](crate::output::ListAccountRolesOutput::role_list): <p>A paginated response with the list of roles and the next token if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListAccountRolesError>`](crate::error::ListAccountRolesError)
    pub fn list_account_roles(&self) -> fluent_builders::ListAccountRoles<C, M, R> {
        fluent_builders::ListAccountRoles::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListAccounts`](crate::client::fluent_builders::ListAccounts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAccounts::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAccounts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAccounts::set_next_token): <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAccounts::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAccounts::set_max_results): <p>This is the number of items clients can request per page.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::ListAccounts::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::ListAccounts::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    /// - On success, responds with [`ListAccountsOutput`](crate::output::ListAccountsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListAccountsOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
    ///   - [`account_list(Option<Vec<AccountInfo>>)`](crate::output::ListAccountsOutput::account_list): <p>A paginated response with the list of account information and the next token if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListAccountsError>`](crate::error::ListAccountsError)
    pub fn list_accounts(&self) -> fluent_builders::ListAccounts<C, M, R> {
        fluent_builders::ListAccounts::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`Logout`](crate::client::fluent_builders::Logout) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::Logout::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::Logout::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    /// - On success, responds with [`LogoutOutput`](crate::output::LogoutOutput)

    /// - On failure, responds with [`SdkError<LogoutError>`](crate::error::LogoutError)
    pub fn logout(&self) -> fluent_builders::Logout<C, M, R> {
        fluent_builders::Logout::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `GetRoleCredentials`.
    ///
    /// <p>Returns the STS short-term credentials for a given role name that is assigned to the user.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetRoleCredentials<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_role_credentials_input::Builder,
    }
    impl<C, M, R> GetRoleCredentials<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetRoleCredentials`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetRoleCredentialsOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRoleCredentialsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetRoleCredentialsInputOperationOutputAlias,
                crate::output::GetRoleCredentialsOutput,
                crate::error::GetRoleCredentialsError,
                crate::input::GetRoleCredentialsInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.role_name(input.into());
            self
        }
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_role_name(input);
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListAccountRoles`.
    ///
    /// <p>Lists all roles that are assigned to the user for a given AWS account.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListAccountRoles<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_account_roles_input::Builder,
    }
    impl<C, M, R> ListAccountRoles<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListAccountRoles`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListAccountRolesOutput,
            aws_smithy_http::result::SdkError<crate::error::ListAccountRolesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListAccountRolesInputOperationOutputAlias,
                crate::output::ListAccountRolesOutput,
                crate::error::ListAccountRolesError,
                crate::input::ListAccountRolesInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListAccountRolesPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListAccountRolesPaginator<C, M, R> {
            crate::paginator::ListAccountRolesPaginator::new(self.handle, self.inner)
        }
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListAccounts`.
    ///
    /// <p>Lists all AWS accounts assigned to the user. These AWS accounts are assigned by the administrator of the account. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/useraccess.html#assignusers">Assign User Access</a> in the <i>AWS SSO User Guide</i>. This operation returns a paginated response.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListAccounts<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_accounts_input::Builder,
    }
    impl<C, M, R> ListAccounts<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListAccounts`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListAccountsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListAccountsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListAccountsInputOperationOutputAlias,
                crate::output::ListAccountsOutput,
                crate::error::ListAccountsError,
                crate::input::ListAccountsInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListAccountsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListAccountsPaginator<C, M, R> {
            crate::paginator::ListAccountsPaginator::new(self.handle, self.inner)
        }
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `Logout`.
    ///
    /// <p>Removes the client- and server-side session that is associated with the user.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct Logout<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::logout_input::Builder,
    }
    impl<C, M, R> Logout<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `Logout`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::LogoutOutput,
            aws_smithy_http::result::SdkError<crate::error::LogoutError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::LogoutInputOperationOutputAlias,
                crate::output::LogoutOutput,
                crate::error::LogoutError,
                crate::input::LogoutInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
    }
}

impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
