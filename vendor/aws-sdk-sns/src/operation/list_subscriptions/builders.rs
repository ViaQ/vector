// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_subscriptions::_list_subscriptions_output::ListSubscriptionsOutputBuilder;

pub use crate::operation::list_subscriptions::_list_subscriptions_input::ListSubscriptionsInputBuilder;

impl ListSubscriptionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_subscriptions::ListSubscriptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_subscriptions::ListSubscriptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_subscriptions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSubscriptions`.
///
/// <p>Returns a list of the requester's subscriptions. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptions</code> call to get further results.</p>
/// <p>This action is throttled at 30 transactions per second (TPS).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSubscriptionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_subscriptions::builders::ListSubscriptionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_subscriptions::ListSubscriptionsOutput,
        crate::operation::list_subscriptions::ListSubscriptionsError,
    > for ListSubscriptionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_subscriptions::ListSubscriptionsOutput,
            crate::operation::list_subscriptions::ListSubscriptionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSubscriptionsFluentBuilder {
    /// Creates a new `ListSubscriptions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSubscriptions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_subscriptions::builders::ListSubscriptionsInputBuilder {
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
        crate::operation::list_subscriptions::ListSubscriptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_subscriptions::ListSubscriptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_subscriptions::ListSubscriptions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_subscriptions::ListSubscriptions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_subscriptions::ListSubscriptionsOutput,
        crate::operation::list_subscriptions::ListSubscriptionsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_subscriptions::paginator::ListSubscriptionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_subscriptions::paginator::ListSubscriptionsPaginator {
        crate::operation::list_subscriptions::paginator::ListSubscriptionsPaginator::new(self.handle, self.inner)
    }
    /// <p>Token returned by the previous <code>ListSubscriptions</code> request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Token returned by the previous <code>ListSubscriptions</code> request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Token returned by the previous <code>ListSubscriptions</code> request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
