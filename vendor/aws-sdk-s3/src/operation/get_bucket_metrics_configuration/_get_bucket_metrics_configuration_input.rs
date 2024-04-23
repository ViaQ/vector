// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketMetricsConfigurationInput {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl GetBucketMetricsConfigurationInput {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl GetBucketMetricsConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetBucketMetricsConfigurationInput`](crate::operation::get_bucket_metrics_configuration::GetBucketMetricsConfigurationInput).
    pub fn builder() -> crate::operation::get_bucket_metrics_configuration::builders::GetBucketMetricsConfigurationInputBuilder {
        crate::operation::get_bucket_metrics_configuration::builders::GetBucketMetricsConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetBucketMetricsConfigurationInput`](crate::operation::get_bucket_metrics_configuration::GetBucketMetricsConfigurationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetBucketMetricsConfigurationInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl GetBucketMetricsConfigurationInputBuilder {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p>
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_bucket_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_bucket_owner
    }
    /// Consumes the builder and constructs a [`GetBucketMetricsConfigurationInput`](crate::operation::get_bucket_metrics_configuration::GetBucketMetricsConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_bucket_metrics_configuration::GetBucketMetricsConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_bucket_metrics_configuration::GetBucketMetricsConfigurationInput {
            bucket: self.bucket,
            id: self.id,
            expected_bucket_owner: self.expected_bucket_owner,
        })
    }
}
