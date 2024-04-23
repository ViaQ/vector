// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccessKeyInfo`](crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_key_id(impl Into<String>)`](crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder::access_key_id) / [`set_access_key_id(Option<String>)`](crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder::set_access_key_id):<br>required: **true**<br><p>The identifier of an access key.</p>  <p>This parameter allows (through its regex pattern) a string of characters that can consist of any upper- or lowercase letter or digit.</p><br>
    /// - On success, responds with [`GetAccessKeyInfoOutput`](crate::operation::get_access_key_info::GetAccessKeyInfoOutput) with field(s):
    ///   - [`account(Option<String>)`](crate::operation::get_access_key_info::GetAccessKeyInfoOutput::account): <p>The number used to identify the Amazon Web Services account.</p>
    /// - On failure, responds with [`SdkError<GetAccessKeyInfoError>`](crate::operation::get_access_key_info::GetAccessKeyInfoError)
    pub fn get_access_key_info(&self) -> crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder {
        crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder::new(self.handle.clone())
    }
}
