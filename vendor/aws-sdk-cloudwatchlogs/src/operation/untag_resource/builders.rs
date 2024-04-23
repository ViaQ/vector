// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::untag_resource::_untag_resource_output::UntagResourceOutputBuilder;

pub use crate::operation::untag_resource::_untag_resource_input::UntagResourceInputBuilder;

impl UntagResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::untag_resource::UntagResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::untag_resource::UntagResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.untag_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UntagResource`.
///
/// <p>Removes one or more tags from the specified resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UntagResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::untag_resource::builders::UntagResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::untag_resource::UntagResourceOutput,
        crate::operation::untag_resource::UntagResourceError,
    > for UntagResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::untag_resource::UntagResourceOutput,
            crate::operation::untag_resource::UntagResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UntagResourceFluentBuilder {
    /// Creates a new `UntagResource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UntagResource as a reference.
    pub fn as_input(&self) -> &crate::operation::untag_resource::builders::UntagResourceInputBuilder {
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
        crate::operation::untag_resource::UntagResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::untag_resource::UntagResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::untag_resource::UntagResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::untag_resource::UntagResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::untag_resource::UntagResourceOutput,
        crate::operation::untag_resource::UntagResourceError,
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
    /// <p>The ARN of the CloudWatch Logs resource that you're removing tags from.</p>
    /// <p>The ARN format of a log group is <code>arn:aws:logs:<i>Region</i>:<i>account-id</i>:log-group:<i>log-group-name</i> </code> </p>
    /// <p>The ARN format of a destination is <code>arn:aws:logs:<i>Region</i>:<i>account-id</i>:destination:<i>destination-name</i> </code> </p>
    /// <p>For more information about ARN format, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/iam-access-control-overview-cwl.html">CloudWatch Logs resources and operations</a>.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN of the CloudWatch Logs resource that you're removing tags from.</p>
    /// <p>The ARN format of a log group is <code>arn:aws:logs:<i>Region</i>:<i>account-id</i>:log-group:<i>log-group-name</i> </code> </p>
    /// <p>The ARN format of a destination is <code>arn:aws:logs:<i>Region</i>:<i>account-id</i>:destination:<i>destination-name</i> </code> </p>
    /// <p>For more information about ARN format, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/iam-access-control-overview-cwl.html">CloudWatch Logs resources and operations</a>.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The ARN of the CloudWatch Logs resource that you're removing tags from.</p>
    /// <p>The ARN format of a log group is <code>arn:aws:logs:<i>Region</i>:<i>account-id</i>:log-group:<i>log-group-name</i> </code> </p>
    /// <p>The ARN format of a destination is <code>arn:aws:logs:<i>Region</i>:<i>account-id</i>:destination:<i>destination-name</i> </code> </p>
    /// <p>For more information about ARN format, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/iam-access-control-overview-cwl.html">CloudWatch Logs resources and operations</a>.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// Appends an item to `tagKeys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>The list of tag keys to remove from the resource.</p>
    pub fn tag_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tag_keys(input.into());
        self
    }
    /// <p>The list of tag keys to remove from the resource.</p>
    pub fn set_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_tag_keys(input);
        self
    }
    /// <p>The list of tag keys to remove from the resource.</p>
    pub fn get_tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_tag_keys()
    }
}
