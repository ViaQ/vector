// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_elasticsearch_domains::_describe_elasticsearch_domains_output::DescribeElasticsearchDomainsOutputBuilder;

pub use crate::operation::describe_elasticsearch_domains::_describe_elasticsearch_domains_input::DescribeElasticsearchDomainsInputBuilder;

impl DescribeElasticsearchDomainsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_elasticsearch_domains();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeElasticsearchDomains`.
///
/// <p>Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeElasticsearchDomainsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput,
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsError,
    > for DescribeElasticsearchDomainsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput,
            crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeElasticsearchDomainsFluentBuilder {
    /// Creates a new `DescribeElasticsearchDomains`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeElasticsearchDomains as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsInputBuilder {
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
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomains::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomains::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput,
        crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsError,
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
    /// Appends an item to `DomainNames`.
    ///
    /// To override the contents of this collection use [`set_domain_names`](Self::set_domain_names).
    ///
    /// <p>The Elasticsearch domains for which you want information.</p>
    pub fn domain_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_names(input.into());
        self
    }
    /// <p>The Elasticsearch domains for which you want information.</p>
    pub fn set_domain_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_domain_names(input);
        self
    }
    /// <p>The Elasticsearch domains for which you want information.</p>
    pub fn get_domain_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_domain_names()
    }
}
