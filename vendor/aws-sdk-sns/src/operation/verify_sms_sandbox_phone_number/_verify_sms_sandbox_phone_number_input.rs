// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifySmsSandboxPhoneNumberInput {
    /// <p>The destination phone number to verify.</p>
    pub phone_number: ::std::option::Option<::std::string::String>,
    /// <p>The OTP sent to the destination number from the <code>CreateSMSSandBoxPhoneNumber</code> call.</p>
    pub one_time_password: ::std::option::Option<::std::string::String>,
}
impl VerifySmsSandboxPhoneNumberInput {
    /// <p>The destination phone number to verify.</p>
    pub fn phone_number(&self) -> ::std::option::Option<&str> {
        self.phone_number.as_deref()
    }
    /// <p>The OTP sent to the destination number from the <code>CreateSMSSandBoxPhoneNumber</code> call.</p>
    pub fn one_time_password(&self) -> ::std::option::Option<&str> {
        self.one_time_password.as_deref()
    }
}
impl VerifySmsSandboxPhoneNumberInput {
    /// Creates a new builder-style object to manufacture [`VerifySmsSandboxPhoneNumberInput`](crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberInput).
    pub fn builder() -> crate::operation::verify_sms_sandbox_phone_number::builders::VerifySmsSandboxPhoneNumberInputBuilder {
        crate::operation::verify_sms_sandbox_phone_number::builders::VerifySmsSandboxPhoneNumberInputBuilder::default()
    }
}

/// A builder for [`VerifySmsSandboxPhoneNumberInput`](crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VerifySmsSandboxPhoneNumberInputBuilder {
    pub(crate) phone_number: ::std::option::Option<::std::string::String>,
    pub(crate) one_time_password: ::std::option::Option<::std::string::String>,
}
impl VerifySmsSandboxPhoneNumberInputBuilder {
    /// <p>The destination phone number to verify.</p>
    /// This field is required.
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.phone_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination phone number to verify.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.phone_number = input;
        self
    }
    /// <p>The destination phone number to verify.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.phone_number
    }
    /// <p>The OTP sent to the destination number from the <code>CreateSMSSandBoxPhoneNumber</code> call.</p>
    /// This field is required.
    pub fn one_time_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.one_time_password = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The OTP sent to the destination number from the <code>CreateSMSSandBoxPhoneNumber</code> call.</p>
    pub fn set_one_time_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.one_time_password = input;
        self
    }
    /// <p>The OTP sent to the destination number from the <code>CreateSMSSandBoxPhoneNumber</code> call.</p>
    pub fn get_one_time_password(&self) -> &::std::option::Option<::std::string::String> {
        &self.one_time_password
    }
    /// Consumes the builder and constructs a [`VerifySmsSandboxPhoneNumberInput`](crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberInput {
            phone_number: self.phone_number,
            one_time_password: self.one_time_password,
        })
    }
}
