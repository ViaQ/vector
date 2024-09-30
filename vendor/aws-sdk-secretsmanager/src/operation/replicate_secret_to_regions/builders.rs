// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::replicate_secret_to_regions::_replicate_secret_to_regions_output::ReplicateSecretToRegionsOutputBuilder;

pub use crate::operation::replicate_secret_to_regions::_replicate_secret_to_regions_input::ReplicateSecretToRegionsInputBuilder;

impl ReplicateSecretToRegionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.replicate_secret_to_regions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ReplicateSecretToRegions`.
///
/// <p>Replicates the secret to a new Regions. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/create-manage-multi-region-secrets.html">Multi-Region secrets</a>.</p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters because it might be logged. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieve-ct-entries.html">Logging Secrets Manager events with CloudTrail</a>.</p>
/// <p> <b>Required permissions: </b> <code>secretsmanager:ReplicateSecretToRegions</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReplicateSecretToRegionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::replicate_secret_to_regions::builders::ReplicateSecretToRegionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput,
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsError,
    > for ReplicateSecretToRegionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput,
            crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ReplicateSecretToRegionsFluentBuilder {
    /// Creates a new `ReplicateSecretToRegions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ReplicateSecretToRegions as a reference.
    pub fn as_input(&self) -> &crate::operation::replicate_secret_to_regions::builders::ReplicateSecretToRegionsInputBuilder {
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
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::replicate_secret_to_regions::ReplicateSecretToRegions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput,
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsError,
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
    /// <p>The ARN or name of the secret to replicate.</p>
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_id(input.into());
        self
    }
    /// <p>The ARN or name of the secret to replicate.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_id(input);
        self
    }
    /// <p>The ARN or name of the secret to replicate.</p>
    pub fn get_secret_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_secret_id()
    }
    /// Appends an item to `AddReplicaRegions`.
    ///
    /// To override the contents of this collection use [`set_add_replica_regions`](Self::set_add_replica_regions).
    ///
    /// <p>A list of Regions in which to replicate the secret.</p>
    pub fn add_replica_regions(mut self, input: crate::types::ReplicaRegionType) -> Self {
        self.inner = self.inner.add_replica_regions(input);
        self
    }
    /// <p>A list of Regions in which to replicate the secret.</p>
    pub fn set_add_replica_regions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReplicaRegionType>>) -> Self {
        self.inner = self.inner.set_add_replica_regions(input);
        self
    }
    /// <p>A list of Regions in which to replicate the secret.</p>
    pub fn get_add_replica_regions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReplicaRegionType>> {
        self.inner.get_add_replica_regions()
    }
    /// <p>Specifies whether to overwrite a secret with the same name in the destination Region. By default, secrets aren't overwritten.</p>
    pub fn force_overwrite_replica_secret(mut self, input: bool) -> Self {
        self.inner = self.inner.force_overwrite_replica_secret(input);
        self
    }
    /// <p>Specifies whether to overwrite a secret with the same name in the destination Region. By default, secrets aren't overwritten.</p>
    pub fn set_force_overwrite_replica_secret(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_overwrite_replica_secret(input);
        self
    }
    /// <p>Specifies whether to overwrite a secret with the same name in the destination Region. By default, secrets aren't overwritten.</p>
    pub fn get_force_overwrite_replica_secret(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_overwrite_replica_secret()
    }
}
