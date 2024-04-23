// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ListPartsInput {
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p> <b>Directory buckets</b> - When you use this operation with a directory bucket, you must use virtual-hosted-style requests in the format <code> <i>Bucket_name</i>.s3express-<i>az_id</i>.<i>region</i>.amazonaws.com</code>. Path-style requests are not supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must follow the format <code> <i>bucket_base_name</i>--<i>az-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p> <b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>Access points and Object Lambda access points are not supported by directory buckets.</p>
    /// </note>
    /// <p> <b>S3 on Outposts</b> - When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>Sets the maximum number of parts to return.</p>
    pub max_parts: ::std::option::Option<i32>,
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub part_number_marker: ::std::option::Option<::std::string::String>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: ::std::option::Option<::std::string::String>,
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub request_payer: ::std::option::Option<crate::types::RequestPayer>,
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub sse_customer_algorithm: ::std::option::Option<::std::string::String>,
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub sse_customer_key: ::std::option::Option<::std::string::String>,
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub sse_customer_key_md5: ::std::option::Option<::std::string::String>,
}
impl ListPartsInput {
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p> <b>Directory buckets</b> - When you use this operation with a directory bucket, you must use virtual-hosted-style requests in the format <code> <i>Bucket_name</i>.s3express-<i>az_id</i>.<i>region</i>.amazonaws.com</code>. Path-style requests are not supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must follow the format <code> <i>bucket_base_name</i>--<i>az-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p> <b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>Access points and Object Lambda access points are not supported by directory buckets.</p>
    /// </note>
    /// <p> <b>S3 on Outposts</b> - When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn max_parts(&self) -> ::std::option::Option<i32> {
        self.max_parts
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn part_number_marker(&self) -> ::std::option::Option<&str> {
        self.part_number_marker.as_deref()
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub fn upload_id(&self) -> ::std::option::Option<&str> {
        self.upload_id.as_deref()
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_payer(&self) -> ::std::option::Option<&crate::types::RequestPayer> {
        self.request_payer.as_ref()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_algorithm(&self) -> ::std::option::Option<&str> {
        self.sse_customer_algorithm.as_deref()
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_key(&self) -> ::std::option::Option<&str> {
        self.sse_customer_key.as_deref()
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_key_md5(&self) -> ::std::option::Option<&str> {
        self.sse_customer_key_md5.as_deref()
    }
}
impl ::std::fmt::Debug for ListPartsInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListPartsInput");
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("max_parts", &self.max_parts);
        formatter.field("part_number_marker", &self.part_number_marker);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("request_payer", &self.request_payer);
        formatter.field("expected_bucket_owner", &self.expected_bucket_owner);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key", &"*** Sensitive Data Redacted ***");
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.finish()
    }
}
impl ListPartsInput {
    /// Creates a new builder-style object to manufacture [`ListPartsInput`](crate::operation::list_parts::ListPartsInput).
    pub fn builder() -> crate::operation::list_parts::builders::ListPartsInputBuilder {
        crate::operation::list_parts::builders::ListPartsInputBuilder::default()
    }
}

/// A builder for [`ListPartsInput`](crate::operation::list_parts::ListPartsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ListPartsInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) max_parts: ::std::option::Option<i32>,
    pub(crate) part_number_marker: ::std::option::Option<::std::string::String>,
    pub(crate) upload_id: ::std::option::Option<::std::string::String>,
    pub(crate) request_payer: ::std::option::Option<crate::types::RequestPayer>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
    pub(crate) sse_customer_algorithm: ::std::option::Option<::std::string::String>,
    pub(crate) sse_customer_key: ::std::option::Option<::std::string::String>,
    pub(crate) sse_customer_key_md5: ::std::option::Option<::std::string::String>,
}
impl ListPartsInputBuilder {
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p> <b>Directory buckets</b> - When you use this operation with a directory bucket, you must use virtual-hosted-style requests in the format <code> <i>Bucket_name</i>.s3express-<i>az_id</i>.<i>region</i>.amazonaws.com</code>. Path-style requests are not supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must follow the format <code> <i>bucket_base_name</i>--<i>az-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p> <b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>Access points and Object Lambda access points are not supported by directory buckets.</p>
    /// </note>
    /// <p> <b>S3 on Outposts</b> - When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p> <b>Directory buckets</b> - When you use this operation with a directory bucket, you must use virtual-hosted-style requests in the format <code> <i>Bucket_name</i>.s3express-<i>az_id</i>.<i>region</i>.amazonaws.com</code>. Path-style requests are not supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must follow the format <code> <i>bucket_base_name</i>--<i>az-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p> <b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>Access points and Object Lambda access points are not supported by directory buckets.</p>
    /// </note>
    /// <p> <b>S3 on Outposts</b> - When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p> <b>Directory buckets</b> - When you use this operation with a directory bucket, you must use virtual-hosted-style requests in the format <code> <i>Bucket_name</i>.s3express-<i>az_id</i>.<i>region</i>.amazonaws.com</code>. Path-style requests are not supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must follow the format <code> <i>bucket_base_name</i>--<i>az-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p> <b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>Access points and Object Lambda access points are not supported by directory buckets.</p>
    /// </note>
    /// <p> <b>S3 on Outposts</b> - When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    /// This field is required.
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn max_parts(mut self, input: i32) -> Self {
        self.max_parts = ::std::option::Option::Some(input);
        self
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn set_max_parts(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_parts = input;
        self
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn get_max_parts(&self) -> &::std::option::Option<i32> {
        &self.max_parts
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn part_number_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.part_number_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn set_part_number_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.part_number_marker = input;
        self
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn get_part_number_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.part_number_marker
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    /// This field is required.
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id = input;
        self
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.request_payer = ::std::option::Option::Some(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_request_payer(mut self, input: ::std::option::Option<crate::types::RequestPayer>) -> Self {
        self.request_payer = input;
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_request_payer(&self) -> &::std::option::Option<crate::types::RequestPayer> {
        &self.request_payer
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
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_algorithm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sse_customer_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_sse_customer_algorithm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sse_customer_algorithm = input;
        self
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_sse_customer_algorithm(&self) -> &::std::option::Option<::std::string::String> {
        &self.sse_customer_algorithm
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sse_customer_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_sse_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sse_customer_key = input;
        self
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_sse_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.sse_customer_key
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sse_customer_key_md5 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_sse_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sse_customer_key_md5 = input;
        self
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_sse_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.sse_customer_key_md5
    }
    /// Consumes the builder and constructs a [`ListPartsInput`](crate::operation::list_parts::ListPartsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_parts::ListPartsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_parts::ListPartsInput {
            bucket: self.bucket,
            key: self.key,
            max_parts: self.max_parts,
            part_number_marker: self.part_number_marker,
            upload_id: self.upload_id,
            request_payer: self.request_payer,
            expected_bucket_owner: self.expected_bucket_owner,
            sse_customer_algorithm: self.sse_customer_algorithm,
            sse_customer_key: self.sse_customer_key,
            sse_customer_key_md5: self.sse_customer_key_md5,
        })
    }
}
impl ::std::fmt::Debug for ListPartsInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListPartsInputBuilder");
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("max_parts", &self.max_parts);
        formatter.field("part_number_marker", &self.part_number_marker);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("request_payer", &self.request_payer);
        formatter.field("expected_bucket_owner", &self.expected_bucket_owner);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key", &"*** Sensitive Data Redacted ***");
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.finish()
    }
}
