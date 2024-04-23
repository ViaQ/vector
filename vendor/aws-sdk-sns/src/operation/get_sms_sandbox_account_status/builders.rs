// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_sms_sandbox_account_status::_get_sms_sandbox_account_status_output::GetSmsSandboxAccountStatusOutputBuilder;

pub use crate::operation::get_sms_sandbox_account_status::_get_sms_sandbox_account_status_input::GetSmsSandboxAccountStatusInputBuilder;

impl GetSmsSandboxAccountStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_sms_sandbox_account_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetSMSSandboxAccountStatus`.
///
/// <p>Retrieves the SMS sandbox status for the calling Amazon Web Services account in the target Amazon Web Services Region.</p>
/// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSMSSandboxAccountStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
        crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
    > for GetSMSSandboxAccountStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
            crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetSMSSandboxAccountStatusFluentBuilder {
    /// Creates a new `GetSMSSandboxAccountStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetSMSSandboxAccountStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusInputBuilder {
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
        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_sms_sandbox_account_status::GetSmsSandboxAccountStatusOutput,
        crate::operation::get_sms_sandbox_account_status::GetSMSSandboxAccountStatusError,
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
}
