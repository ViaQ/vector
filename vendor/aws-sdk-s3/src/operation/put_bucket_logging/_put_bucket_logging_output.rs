// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutBucketLoggingOutput {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl crate::s3_request_id::RequestIdExt for PutBucketLoggingOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for PutBucketLoggingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutBucketLoggingOutput {
    /// Creates a new builder-style object to manufacture [`PutBucketLoggingOutput`](crate::operation::put_bucket_logging::PutBucketLoggingOutput).
    pub fn builder() -> crate::operation::put_bucket_logging::builders::PutBucketLoggingOutputBuilder {
        crate::operation::put_bucket_logging::builders::PutBucketLoggingOutputBuilder::default()
    }
}

/// A builder for [`PutBucketLoggingOutput`](crate::operation::put_bucket_logging::PutBucketLoggingOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutBucketLoggingOutputBuilder {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl PutBucketLoggingOutputBuilder {
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
    /// Consumes the builder and constructs a [`PutBucketLoggingOutput`](crate::operation::put_bucket_logging::PutBucketLoggingOutput).
    pub fn build(self) -> crate::operation::put_bucket_logging::PutBucketLoggingOutput {
        crate::operation::put_bucket_logging::PutBucketLoggingOutput {
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
