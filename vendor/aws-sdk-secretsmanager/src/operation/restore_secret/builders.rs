// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_secret::_restore_secret_output::RestoreSecretOutputBuilder;

pub use crate::operation::restore_secret::_restore_secret_input::RestoreSecretInputBuilder;

impl RestoreSecretInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::restore_secret::RestoreSecretOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_secret::RestoreSecretError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.restore_secret();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestoreSecret`.
///
/// <p>Cancels the scheduled deletion of a secret by removing the <code>DeletedDate</code> time stamp. You can access a secret again after it has been restored.</p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters because it might be logged. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieve-ct-entries.html">Logging Secrets Manager events with CloudTrail</a>.</p>
/// <p> <b>Required permissions: </b> <code>secretsmanager:RestoreSecret</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreSecretFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::restore_secret::builders::RestoreSecretInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::restore_secret::RestoreSecretOutput,
        crate::operation::restore_secret::RestoreSecretError,
    > for RestoreSecretFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::restore_secret::RestoreSecretOutput,
            crate::operation::restore_secret::RestoreSecretError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RestoreSecretFluentBuilder {
    /// Creates a new `RestoreSecret`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RestoreSecret as a reference.
    pub fn as_input(&self) -> &crate::operation::restore_secret::builders::RestoreSecretInputBuilder {
        &self.inner
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
    ) -> ::std::result::Result<
        crate::operation::restore_secret::RestoreSecretOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_secret::RestoreSecretError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::restore_secret::RestoreSecret::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::restore_secret::RestoreSecret::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::restore_secret::RestoreSecretOutput,
        crate::operation::restore_secret::RestoreSecretError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ARN or name of the secret to restore.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_id(input.into());
        self
    }
    /// <p>The ARN or name of the secret to restore.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_id(input);
        self
    }
    /// <p>The ARN or name of the secret to restore.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn get_secret_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_secret_id()
    }
}
