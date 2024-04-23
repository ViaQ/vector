// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_tags_from_stream::_remove_tags_from_stream_output::RemoveTagsFromStreamOutputBuilder;

pub use crate::operation::remove_tags_from_stream::_remove_tags_from_stream_input::RemoveTagsFromStreamInputBuilder;

impl RemoveTagsFromStreamInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_tags_from_stream::RemoveTagsFromStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_tags_from_stream();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveTagsFromStream`.
///
/// <p>Removes tags from the specified Kinesis data stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <note>
/// <p>When invoking this API, you must use either the <code>StreamARN</code> or the <code>StreamName</code> parameter, or both. It is recommended that you use the <code>StreamARN</code> input parameter when you invoke this API.</p>
/// </note>
/// <p>If you specify a tag that does not exist, it is ignored.</p>
/// <p> <code>RemoveTagsFromStream</code> has a limit of five transactions per second per account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveTagsFromStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput,
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamError,
    > for RemoveTagsFromStreamFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput,
            crate::operation::remove_tags_from_stream::RemoveTagsFromStreamError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemoveTagsFromStreamFluentBuilder {
    /// Creates a new `RemoveTagsFromStream`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveTagsFromStream as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamInputBuilder {
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
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_tags_from_stream::RemoveTagsFromStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_tags_from_stream::RemoveTagsFromStream::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_tags_from_stream::RemoveTagsFromStream::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput,
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamError,
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
    /// <p>The name of the stream.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The name of the stream.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_name()
    }
    /// Appends an item to `TagKeys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>A list of tag keys. Each corresponding tag is removed from the stream.</p>
    pub fn tag_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tag_keys(input.into());
        self
    }
    /// <p>A list of tag keys. Each corresponding tag is removed from the stream.</p>
    pub fn set_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_tag_keys(input);
        self
    }
    /// <p>A list of tag keys. Each corresponding tag is removed from the stream.</p>
    pub fn get_tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_tag_keys()
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_arn()
    }
}
