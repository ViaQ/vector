// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CompleteMultipartUpload`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_bucket):<br>required: **true**<br><p>Name of the bucket to which the multipart upload was initiated.</p>  <p> <b>Directory buckets</b> - When you use this operation with a directory bucket, you must use virtual-hosted-style requests in the format <code> <i>Bucket_name</i>.s3express-<i>az_id</i>.<i>region</i>.amazonaws.com</code>. Path-style requests are not supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must follow the format <code> <i>bucket_base_name</i>--<i>az-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>  <p> <b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p> <note>   <p>Access points and Object Lambda access points are not supported by directory buckets.</p>  </note>  <p> <b>S3 on Outposts</b> - When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p><br>
    ///   - [`key(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_key):<br>required: **true**<br><p>Object key for which the multipart upload was initiated.</p><br>
    ///   - [`multipart_upload(CompletedMultipartUpload)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::multipart_upload) / [`set_multipart_upload(Option<CompletedMultipartUpload>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_multipart_upload):<br>required: **false**<br><p>The container for the multipart upload request information.</p><br>
    ///   - [`upload_id(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_upload_id):<br>required: **true**<br><p>ID for the initiated multipart upload.</p><br>
    ///   - [`checksum_crc32(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::checksum_crc32) / [`set_checksum_crc32(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_checksum_crc32):<br>required: **false**<br><p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p><br>
    ///   - [`checksum_crc32_c(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::checksum_crc32_c) / [`set_checksum_crc32_c(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_checksum_crc32_c):<br>required: **false**<br><p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p><br>
    ///   - [`checksum_sha1(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::checksum_sha1) / [`set_checksum_sha1(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_checksum_sha1):<br>required: **false**<br><p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p><br>
    ///   - [`checksum_sha256(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::checksum_sha256) / [`set_checksum_sha256(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_checksum_sha256):<br>required: **false**<br><p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p><br>
    ///   - [`request_payer(RequestPayer)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_request_payer):<br>required: **false**<br><p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    ///   - [`sse_customer_algorithm(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::sse_customer_algorithm) / [`set_sse_customer_algorithm(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_sse_customer_algorithm):<br>required: **false**<br><p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is required only when the object was created using a checksum algorithm or if your bucket policy requires the use of SSE-C. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerSideEncryptionCustomerKeys.html#ssec-require-condition-key">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`sse_customer_key(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::sse_customer_key) / [`set_sse_customer_key(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_sse_customer_key):<br>required: **false**<br><p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`sse_customer_key_md5(impl Into<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::sse_customer_key_md5) / [`set_sse_customer_key_md5(Option<String>)`](crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::set_sse_customer_key_md5):<br>required: **false**<br><p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    /// - On success, responds with [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput) with field(s):
    ///   - [`location(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::location): <p>The URI that identifies the newly created object.</p>
    ///   - [`bucket(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::bucket): <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p> <note>   <p>Access points are not supported by directory buckets.</p>  </note>
    ///   - [`key(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::key): <p>The object key of the newly created object.</p>
    ///   - [`expiration(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::expiration): <p>If the object expiration is configured, this will contain the expiration date (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of <code>rule-id</code> is URL-encoded.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note>
    ///   - [`e_tag(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::e_tag): <p>Entity tag that identifies the newly created object's data. Objects with different object data will have different entity tags. The entity tag is an opaque string. The entity tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5 digest of the object data, it will contain one or more nonhexadecimal characters and/or will consist of less than 32 or more than 32 hexadecimal digits. For more information about how the entity tag is calculated, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::checksum_crc32): <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32_c(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::checksum_crc32_c): <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha1(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::checksum_sha1): <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. When you use the API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha256(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::checksum_sha256): <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`server_side_encryption(Option<ServerSideEncryption>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::server_side_encryption): <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>).</p> <note>   <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>  </note>
    ///   - [`version_id(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::version_id): <p>Version ID of the newly created object, in case the bucket has versioning turned on.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note>
    ///   - [`ssekms_key_id(Option<String>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::ssekms_key_id): <p>If present, indicates the ID of the Key Management Service (KMS) symmetric encryption customer managed key that was used for the object.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note>
    ///   - [`bucket_key_enabled(Option<bool>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::bucket_key_enabled): <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note>
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note>
    /// - On failure, responds with [`SdkError<CompleteMultipartUploadError>`](crate::operation::complete_multipart_upload::CompleteMultipartUploadError)
    pub fn complete_multipart_upload(&self) -> crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder {
        crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadFluentBuilder::new(self.handle.clone())
    }
}
