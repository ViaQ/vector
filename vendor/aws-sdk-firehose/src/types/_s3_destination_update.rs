// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an update for a destination in Amazon S3.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3DestinationUpdate {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub bucket_arn: ::std::option::Option<::std::string::String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub error_output_prefix: ::std::option::Option<::std::string::String>,
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    pub buffering_hints: ::std::option::Option<crate::types::BufferingHints>,
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    /// <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    pub compression_format: ::std::option::Option<crate::types::CompressionFormat>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    pub encryption_configuration: ::std::option::Option<crate::types::EncryptionConfiguration>,
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    pub cloud_watch_logging_options: ::std::option::Option<crate::types::CloudWatchLoggingOptions>,
}
impl S3DestinationUpdate {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn bucket_arn(&self) -> ::std::option::Option<&str> {
        self.bucket_arn.as_deref()
    }
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn error_output_prefix(&self) -> ::std::option::Option<&str> {
        self.error_output_prefix.as_deref()
    }
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    pub fn buffering_hints(&self) -> ::std::option::Option<&crate::types::BufferingHints> {
        self.buffering_hints.as_ref()
    }
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    /// <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    pub fn compression_format(&self) -> ::std::option::Option<&crate::types::CompressionFormat> {
        self.compression_format.as_ref()
    }
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    pub fn encryption_configuration(&self) -> ::std::option::Option<&crate::types::EncryptionConfiguration> {
        self.encryption_configuration.as_ref()
    }
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    pub fn cloud_watch_logging_options(&self) -> ::std::option::Option<&crate::types::CloudWatchLoggingOptions> {
        self.cloud_watch_logging_options.as_ref()
    }
}
impl S3DestinationUpdate {
    /// Creates a new builder-style object to manufacture [`S3DestinationUpdate`](crate::types::S3DestinationUpdate).
    pub fn builder() -> crate::types::builders::S3DestinationUpdateBuilder {
        crate::types::builders::S3DestinationUpdateBuilder::default()
    }
}

/// A builder for [`S3DestinationUpdate`](crate::types::S3DestinationUpdate).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct S3DestinationUpdateBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_arn: ::std::option::Option<::std::string::String>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) error_output_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) buffering_hints: ::std::option::Option<crate::types::BufferingHints>,
    pub(crate) compression_format: ::std::option::Option<crate::types::CompressionFormat>,
    pub(crate) encryption_configuration: ::std::option::Option<crate::types::EncryptionConfiguration>,
    pub(crate) cloud_watch_logging_options: ::std::option::Option<crate::types::CloudWatchLoggingOptions>,
}
impl S3DestinationUpdateBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn bucket_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn set_bucket_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_arn = input;
        self
    }
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn get_bucket_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket_arn
    }
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn error_output_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_output_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn set_error_output_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_output_prefix = input;
        self
    }
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    pub fn get_error_output_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.error_output_prefix
    }
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    pub fn buffering_hints(mut self, input: crate::types::BufferingHints) -> Self {
        self.buffering_hints = ::std::option::Option::Some(input);
        self
    }
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    pub fn set_buffering_hints(mut self, input: ::std::option::Option<crate::types::BufferingHints>) -> Self {
        self.buffering_hints = input;
        self
    }
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    pub fn get_buffering_hints(&self) -> &::std::option::Option<crate::types::BufferingHints> {
        &self.buffering_hints
    }
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    /// <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    pub fn compression_format(mut self, input: crate::types::CompressionFormat) -> Self {
        self.compression_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    /// <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    pub fn set_compression_format(mut self, input: ::std::option::Option<crate::types::CompressionFormat>) -> Self {
        self.compression_format = input;
        self
    }
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    /// <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    pub fn get_compression_format(&self) -> &::std::option::Option<crate::types::CompressionFormat> {
        &self.compression_format
    }
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    pub fn encryption_configuration(mut self, input: crate::types::EncryptionConfiguration) -> Self {
        self.encryption_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    pub fn set_encryption_configuration(mut self, input: ::std::option::Option<crate::types::EncryptionConfiguration>) -> Self {
        self.encryption_configuration = input;
        self
    }
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    pub fn get_encryption_configuration(&self) -> &::std::option::Option<crate::types::EncryptionConfiguration> {
        &self.encryption_configuration
    }
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    pub fn cloud_watch_logging_options(mut self, input: crate::types::CloudWatchLoggingOptions) -> Self {
        self.cloud_watch_logging_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    pub fn set_cloud_watch_logging_options(mut self, input: ::std::option::Option<crate::types::CloudWatchLoggingOptions>) -> Self {
        self.cloud_watch_logging_options = input;
        self
    }
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    pub fn get_cloud_watch_logging_options(&self) -> &::std::option::Option<crate::types::CloudWatchLoggingOptions> {
        &self.cloud_watch_logging_options
    }
    /// Consumes the builder and constructs a [`S3DestinationUpdate`](crate::types::S3DestinationUpdate).
    pub fn build(self) -> crate::types::S3DestinationUpdate {
        crate::types::S3DestinationUpdate {
            role_arn: self.role_arn,
            bucket_arn: self.bucket_arn,
            prefix: self.prefix,
            error_output_prefix: self.error_output_prefix,
            buffering_hints: self.buffering_hints,
            compression_format: self.compression_format,
            encryption_configuration: self.encryption_configuration,
            cloud_watch_logging_options: self.cloud_watch_logging_options,
        }
    }
}
