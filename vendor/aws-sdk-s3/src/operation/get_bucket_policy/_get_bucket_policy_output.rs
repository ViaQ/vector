// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketPolicyOutput {
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: ::std::option::Option<::std::string::String>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketPolicyOutput {
    /// <p>The bucket policy as a JSON document.</p>
    pub fn policy(&self) -> ::std::option::Option<&str> {
        self.policy.as_deref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetBucketPolicyOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetBucketPolicyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetBucketPolicyOutput {
    /// Creates a new builder-style object to manufacture [`GetBucketPolicyOutput`](crate::operation::get_bucket_policy::GetBucketPolicyOutput).
    pub fn builder() -> crate::operation::get_bucket_policy::builders::GetBucketPolicyOutputBuilder {
        crate::operation::get_bucket_policy::builders::GetBucketPolicyOutputBuilder::default()
    }
}

/// A builder for [`GetBucketPolicyOutput`](crate::operation::get_bucket_policy::GetBucketPolicyOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetBucketPolicyOutputBuilder {
    pub(crate) policy: ::std::option::Option<::std::string::String>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketPolicyOutputBuilder {
    /// <p>The bucket policy as a JSON document.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The bucket policy as a JSON document.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// <p>The bucket policy as a JSON document.</p>
    pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetBucketPolicyOutput`](crate::operation::get_bucket_policy::GetBucketPolicyOutput).
    pub fn build(self) -> crate::operation::get_bucket_policy::GetBucketPolicyOutput {
        crate::operation::get_bucket_policy::GetBucketPolicyOutput {
            policy: self.policy,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
