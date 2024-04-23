// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSMSSandboxPhoneNumbers`](crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder::set_next_token):<br>required: **false**<br><p>Token that the previous <code>ListSMSSandboxPhoneNumbersInput</code> request returns.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of phone numbers to return.</p><br>
    /// - On success, responds with [`ListSmsSandboxPhoneNumbersOutput`](crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput) with field(s):
    ///   - [`phone_numbers(Vec::<SmsSandboxPhoneNumber>)`](crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput::phone_numbers): <p>A list of the calling account's pending and verified phone numbers.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput::next_token): <p>A <code>NextToken</code> string is returned when you call the <code>ListSMSSandboxPhoneNumbersInput</code> operation if additional pages of records are available.</p>
    /// - On failure, responds with [`SdkError<ListSMSSandboxPhoneNumbersError>`](crate::operation::list_sms_sandbox_phone_numbers::ListSMSSandboxPhoneNumbersError)
    pub fn list_sms_sandbox_phone_numbers(
        &self,
    ) -> crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder {
        crate::operation::list_sms_sandbox_phone_numbers::builders::ListSMSSandboxPhoneNumbersFluentBuilder::new(self.handle.clone())
    }
}
