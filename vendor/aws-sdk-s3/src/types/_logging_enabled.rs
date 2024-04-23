// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon S3 API Reference</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LoggingEnabled {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case, you should choose a different <code>TargetPrefix</code> for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub target_bucket: ::std::string::String,
    /// <p>Container for granting information.</p>
    /// <p>Buckets that use the bucket owner enforced setting for Object Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub target_grants: ::std::option::Option<::std::vec::Vec<crate::types::TargetGrant>>,
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a single bucket, you can use a prefix to distinguish which log files came from which bucket.</p>
    pub target_prefix: ::std::string::String,
    /// <p>Amazon S3 key format for log objects.</p>
    pub target_object_key_format: ::std::option::Option<crate::types::TargetObjectKeyFormat>,
}
impl LoggingEnabled {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case, you should choose a different <code>TargetPrefix</code> for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub fn target_bucket(&self) -> &str {
        use std::ops::Deref;
        self.target_bucket.deref()
    }
    /// <p>Container for granting information.</p>
    /// <p>Buckets that use the bucket owner enforced setting for Object Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.target_grants.is_none()`.
    pub fn target_grants(&self) -> &[crate::types::TargetGrant] {
        self.target_grants.as_deref().unwrap_or_default()
    }
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a single bucket, you can use a prefix to distinguish which log files came from which bucket.</p>
    pub fn target_prefix(&self) -> &str {
        use std::ops::Deref;
        self.target_prefix.deref()
    }
    /// <p>Amazon S3 key format for log objects.</p>
    pub fn target_object_key_format(&self) -> ::std::option::Option<&crate::types::TargetObjectKeyFormat> {
        self.target_object_key_format.as_ref()
    }
}
impl LoggingEnabled {
    /// Creates a new builder-style object to manufacture [`LoggingEnabled`](crate::types::LoggingEnabled).
    pub fn builder() -> crate::types::builders::LoggingEnabledBuilder {
        crate::types::builders::LoggingEnabledBuilder::default()
    }
}

/// A builder for [`LoggingEnabled`](crate::types::LoggingEnabled).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LoggingEnabledBuilder {
    pub(crate) target_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) target_grants: ::std::option::Option<::std::vec::Vec<crate::types::TargetGrant>>,
    pub(crate) target_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) target_object_key_format: ::std::option::Option<crate::types::TargetObjectKeyFormat>,
}
impl LoggingEnabledBuilder {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case, you should choose a different <code>TargetPrefix</code> for each source bucket so that the delivered log files can be distinguished by key.</p>
    /// This field is required.
    pub fn target_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.target_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case, you should choose a different <code>TargetPrefix</code> for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub fn set_target_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.target_bucket = input;
        self
    }
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case, you should choose a different <code>TargetPrefix</code> for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub fn get_target_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.target_bucket
    }
    /// Appends an item to `target_grants`.
    ///
    /// To override the contents of this collection use [`set_target_grants`](Self::set_target_grants).
    ///
    /// <p>Container for granting information.</p>
    /// <p>Buckets that use the bucket owner enforced setting for Object Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn target_grants(mut self, input: crate::types::TargetGrant) -> Self {
        let mut v = self.target_grants.unwrap_or_default();
        v.push(input);
        self.target_grants = ::std::option::Option::Some(v);
        self
    }
    /// <p>Container for granting information.</p>
    /// <p>Buckets that use the bucket owner enforced setting for Object Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_target_grants(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TargetGrant>>) -> Self {
        self.target_grants = input;
        self
    }
    /// <p>Container for granting information.</p>
    /// <p>Buckets that use the bucket owner enforced setting for Object Ownership don't support target grants. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_target_grants(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TargetGrant>> {
        &self.target_grants
    }
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a single bucket, you can use a prefix to distinguish which log files came from which bucket.</p>
    /// This field is required.
    pub fn target_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.target_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a single bucket, you can use a prefix to distinguish which log files came from which bucket.</p>
    pub fn set_target_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.target_prefix = input;
        self
    }
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a single bucket, you can use a prefix to distinguish which log files came from which bucket.</p>
    pub fn get_target_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.target_prefix
    }
    /// <p>Amazon S3 key format for log objects.</p>
    pub fn target_object_key_format(mut self, input: crate::types::TargetObjectKeyFormat) -> Self {
        self.target_object_key_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amazon S3 key format for log objects.</p>
    pub fn set_target_object_key_format(mut self, input: ::std::option::Option<crate::types::TargetObjectKeyFormat>) -> Self {
        self.target_object_key_format = input;
        self
    }
    /// <p>Amazon S3 key format for log objects.</p>
    pub fn get_target_object_key_format(&self) -> &::std::option::Option<crate::types::TargetObjectKeyFormat> {
        &self.target_object_key_format
    }
    /// Consumes the builder and constructs a [`LoggingEnabled`](crate::types::LoggingEnabled).
    /// This method will fail if any of the following fields are not set:
    /// - [`target_bucket`](crate::types::builders::LoggingEnabledBuilder::target_bucket)
    /// - [`target_prefix`](crate::types::builders::LoggingEnabledBuilder::target_prefix)
    pub fn build(self) -> ::std::result::Result<crate::types::LoggingEnabled, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::LoggingEnabled {
            target_bucket: self.target_bucket.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "target_bucket",
                    "target_bucket was not specified but it is required when building LoggingEnabled",
                )
            })?,
            target_grants: self.target_grants,
            target_prefix: self.target_prefix.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "target_prefix",
                    "target_prefix was not specified but it is required when building LoggingEnabled",
                )
            })?,
            target_object_key_format: self.target_object_key_format,
        })
    }
}
