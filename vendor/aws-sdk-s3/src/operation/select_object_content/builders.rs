// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::select_object_content::_select_object_content_output::SelectObjectContentOutputBuilder;

pub use crate::operation::select_object_content::_select_object_content_input::SelectObjectContentInputBuilder;

impl SelectObjectContentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::select_object_content::SelectObjectContentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::select_object_content::SelectObjectContentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.select_object_content();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SelectObjectContent`.
///
/// <note>
/// <p>This operation is not supported by directory buckets.</p>
/// </note>
/// <p>This action filters the contents of an Amazon S3 object based on a simple structured query language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON, CSV, or Apache Parquet) of the object. Amazon S3 uses this format to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
/// <p>This functionality is not supported for Amazon S3 on Outposts.</p>
/// <p>For more information about Amazon S3 Select, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/selecting-content-from-objects.html">Selecting Content from Objects</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-glacier-select-sql-reference-select.html">SELECT Command</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p></p>
/// <dl>
/// <dt>
/// Permissions
/// </dt>
/// <dd>
/// <p>You must have the <code>s3:GetObject</code> permission for this operation.&nbsp;Amazon S3 Select does not support anonymous access. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a Policy</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </dd>
/// <dt>
/// Object Data Formats
/// </dt>
/// <dd>
/// <p>You can use Amazon S3 Select to query objects that have the following format properties:</p>
/// <ul>
/// <li> <p> <i>CSV, JSON, and Parquet</i> - Objects must be in CSV, JSON, or Parquet format.</p> </li>
/// <li> <p> <i>UTF-8</i> - UTF-8 is the only encoding type Amazon S3 Select supports.</p> </li>
/// <li> <p> <i>GZIP or BZIP2</i> - CSV and JSON files can be compressed using GZIP or BZIP2. GZIP and BZIP2 are the only compression formats that Amazon S3 Select supports for CSV and JSON files. Amazon S3 Select supports columnar compression for Parquet using GZIP or Snappy. Amazon S3 Select does not support whole-object compression for Parquet objects.</p> </li>
/// <li> <p> <i>Server-side encryption</i> - Amazon S3 Select supports querying objects that are protected with server-side encryption.</p> <p>For objects that are encrypted with customer-provided encryption keys (SSE-C), you must use HTTPS, and you must use the headers that are documented in the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>. For more information about SSE-C, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using Customer-Provided Encryption Keys)</a> in the <i>Amazon S3 User Guide</i>.</p> <p>For objects that are encrypted with Amazon S3 managed keys (SSE-S3) and Amazon Web Services KMS keys (SSE-KMS), server-side encryption is handled transparently, so you don't need to specify anything. For more information about server-side encryption, including SSE-S3 and SSE-KMS, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/serv-side-encryption.html">Protecting Data Using Server-Side Encryption</a> in the <i>Amazon S3 User Guide</i>.</p> </li>
/// </ul>
/// </dd>
/// <dt>
/// Working with the Response Body
/// </dt>
/// <dd>
/// <p>Given the response size is unknown, Amazon S3 Select streams the response as a series of messages and includes a <code>Transfer-Encoding</code> header with <code>chunked</code> as its value in the response. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTSelectObjectAppendix.html">Appendix: SelectObjectContent Response</a>.</p>
/// </dd>
/// <dt>
/// GetObject Support
/// </dt>
/// <dd>
/// <p>The <code>SelectObjectContent</code> action does not support the following <code>GetObject</code> functionality. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a>.</p>
/// <ul>
/// <li> <p> <code>Range</code>: Although you can specify a scan range for an Amazon S3 Select request (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_SelectObjectContent.html#AmazonS3-SelectObjectContent-request-ScanRange">SelectObjectContentRequest - ScanRange</a> in the request parameters), you cannot specify the range of bytes of an object to return. </p> </li>
/// <li> <p>The <code>GLACIER</code>, <code>DEEP_ARCHIVE</code>, and <code>REDUCED_REDUNDANCY</code> storage classes, or the <code>ARCHIVE_ACCESS</code> and <code>DEEP_ARCHIVE_ACCESS</code> access tiers of the <code>INTELLIGENT_TIERING</code> storage class: You cannot query objects in the <code>GLACIER</code>, <code>DEEP_ARCHIVE</code>, or <code>REDUCED_REDUNDANCY</code> storage classes, nor objects in the <code>ARCHIVE_ACCESS</code> or <code>DEEP_ARCHIVE_ACCESS</code> access tiers of the <code>INTELLIGENT_TIERING</code> storage class. For more information about storage classes, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage-class-intro.html">Using Amazon S3 storage classes</a> in the <i>Amazon S3 User Guide</i>.</p> </li>
/// </ul>
/// </dd>
/// <dt>
/// Special Errors
/// </dt>
/// <dd>
/// <p>For a list of special errors for this operation, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html#SelectObjectContentErrorCodeList">List of SELECT Object Content Error Codes</a> </p>
/// </dd>
/// </dl>
/// <p>The following operations are related to <code>SelectObjectContent</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycleConfiguration.html">GetBucketLifecycleConfiguration</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html">PutBucketLifecycleConfiguration</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SelectObjectContentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::select_object_content::builders::SelectObjectContentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::select_object_content::SelectObjectContentOutput,
        crate::operation::select_object_content::SelectObjectContentError,
    > for SelectObjectContentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::select_object_content::SelectObjectContentOutput,
            crate::operation::select_object_content::SelectObjectContentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SelectObjectContentFluentBuilder {
    /// Creates a new `SelectObjectContent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SelectObjectContent as a reference.
    pub fn as_input(&self) -> &crate::operation::select_object_content::builders::SelectObjectContentInputBuilder {
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
        crate::operation::select_object_content::SelectObjectContentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::select_object_content::SelectObjectContentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::select_object_content::SelectObjectContent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::select_object_content::SelectObjectContent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::select_object_content::SelectObjectContentOutput,
        crate::operation::select_object_content::SelectObjectContentError,
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
    /// <p>The S3 bucket.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The S3 bucket.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The S3 bucket.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The object key.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>The object key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>The object key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key()
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn sse_customer_algorithm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_algorithm(input.into());
        self
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_sse_customer_algorithm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_algorithm(input);
        self
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_sse_customer_algorithm(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_algorithm()
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn sse_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key(input.into());
        self
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_sse_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key(input);
        self
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_sse_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_key()
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn sse_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key_md5(input.into());
        self
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_sse_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key_md5(input);
        self
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_sse_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_key_md5()
    }
    /// <p>The expression that is used to query the object.</p>
    pub fn expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expression(input.into());
        self
    }
    /// <p>The expression that is used to query the object.</p>
    pub fn set_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expression(input);
        self
    }
    /// <p>The expression that is used to query the object.</p>
    pub fn get_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expression()
    }
    /// <p>The type of the provided expression (for example, SQL).</p>
    pub fn expression_type(mut self, input: crate::types::ExpressionType) -> Self {
        self.inner = self.inner.expression_type(input);
        self
    }
    /// <p>The type of the provided expression (for example, SQL).</p>
    pub fn set_expression_type(mut self, input: ::std::option::Option<crate::types::ExpressionType>) -> Self {
        self.inner = self.inner.set_expression_type(input);
        self
    }
    /// <p>The type of the provided expression (for example, SQL).</p>
    pub fn get_expression_type(&self) -> &::std::option::Option<crate::types::ExpressionType> {
        self.inner.get_expression_type()
    }
    /// <p>Specifies if periodic request progress information should be enabled.</p>
    pub fn request_progress(mut self, input: crate::types::RequestProgress) -> Self {
        self.inner = self.inner.request_progress(input);
        self
    }
    /// <p>Specifies if periodic request progress information should be enabled.</p>
    pub fn set_request_progress(mut self, input: ::std::option::Option<crate::types::RequestProgress>) -> Self {
        self.inner = self.inner.set_request_progress(input);
        self
    }
    /// <p>Specifies if periodic request progress information should be enabled.</p>
    pub fn get_request_progress(&self) -> &::std::option::Option<crate::types::RequestProgress> {
        self.inner.get_request_progress()
    }
    /// <p>Describes the format of the data in the object that is being queried.</p>
    pub fn input_serialization(mut self, input: crate::types::InputSerialization) -> Self {
        self.inner = self.inner.input_serialization(input);
        self
    }
    /// <p>Describes the format of the data in the object that is being queried.</p>
    pub fn set_input_serialization(mut self, input: ::std::option::Option<crate::types::InputSerialization>) -> Self {
        self.inner = self.inner.set_input_serialization(input);
        self
    }
    /// <p>Describes the format of the data in the object that is being queried.</p>
    pub fn get_input_serialization(&self) -> &::std::option::Option<crate::types::InputSerialization> {
        self.inner.get_input_serialization()
    }
    /// <p>Describes the format of the data that you want Amazon S3 to return in response.</p>
    pub fn output_serialization(mut self, input: crate::types::OutputSerialization) -> Self {
        self.inner = self.inner.output_serialization(input);
        self
    }
    /// <p>Describes the format of the data that you want Amazon S3 to return in response.</p>
    pub fn set_output_serialization(mut self, input: ::std::option::Option<crate::types::OutputSerialization>) -> Self {
        self.inner = self.inner.set_output_serialization(input);
        self
    }
    /// <p>Describes the format of the data that you want Amazon S3 to return in response.</p>
    pub fn get_output_serialization(&self) -> &::std::option::Option<crate::types::OutputSerialization> {
        self.inner.get_output_serialization()
    }
    /// <p>Specifies the byte range of the object to get the records from. A record is processed when its first byte is contained by the range. This parameter is optional, but when specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the start and end of the range.</p>
    /// <p> <code>ScanRange</code>may be used in the following ways:</p>
    /// <ul>
    /// <li> <p> <code>
    /// <scanrange>
    /// <start>
    /// 50
    /// </start>
    /// <end>
    /// 100
    /// </end>
    /// </scanrange></code> - process only the records starting between the bytes 50 and 100 (inclusive, counting from zero)</p> </li>
    /// <li> <p> <code>
    /// <scanrange>
    /// <start>
    /// 50
    /// </start>
    /// </scanrange></code> - process only the records starting after the byte 50</p> </li>
    /// <li> <p> <code>
    /// <scanrange>
    /// <end>
    /// 50
    /// </end>
    /// </scanrange></code> - process only the records within the last 50 bytes of the file.</p> </li>
    /// </ul>
    pub fn scan_range(mut self, input: crate::types::ScanRange) -> Self {
        self.inner = self.inner.scan_range(input);
        self
    }
    /// <p>Specifies the byte range of the object to get the records from. A record is processed when its first byte is contained by the range. This parameter is optional, but when specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the start and end of the range.</p>
    /// <p> <code>ScanRange</code>may be used in the following ways:</p>
    /// <ul>
    /// <li> <p> <code>
    /// <scanrange>
    /// <start>
    /// 50
    /// </start>
    /// <end>
    /// 100
    /// </end>
    /// </scanrange></code> - process only the records starting between the bytes 50 and 100 (inclusive, counting from zero)</p> </li>
    /// <li> <p> <code>
    /// <scanrange>
    /// <start>
    /// 50
    /// </start>
    /// </scanrange></code> - process only the records starting after the byte 50</p> </li>
    /// <li> <p> <code>
    /// <scanrange>
    /// <end>
    /// 50
    /// </end>
    /// </scanrange></code> - process only the records within the last 50 bytes of the file.</p> </li>
    /// </ul>
    pub fn set_scan_range(mut self, input: ::std::option::Option<crate::types::ScanRange>) -> Self {
        self.inner = self.inner.set_scan_range(input);
        self
    }
    /// <p>Specifies the byte range of the object to get the records from. A record is processed when its first byte is contained by the range. This parameter is optional, but when specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the start and end of the range.</p>
    /// <p> <code>ScanRange</code>may be used in the following ways:</p>
    /// <ul>
    /// <li> <p> <code>
    /// <scanrange>
    /// <start>
    /// 50
    /// </start>
    /// <end>
    /// 100
    /// </end>
    /// </scanrange></code> - process only the records starting between the bytes 50 and 100 (inclusive, counting from zero)</p> </li>
    /// <li> <p> <code>
    /// <scanrange>
    /// <start>
    /// 50
    /// </start>
    /// </scanrange></code> - process only the records starting after the byte 50</p> </li>
    /// <li> <p> <code>
    /// <scanrange>
    /// <end>
    /// 50
    /// </end>
    /// </scanrange></code> - process only the records within the last 50 bytes of the file.</p> </li>
    /// </ul>
    pub fn get_scan_range(&self) -> &::std::option::Option<crate::types::ScanRange> {
        self.inner.get_scan_range()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
}
