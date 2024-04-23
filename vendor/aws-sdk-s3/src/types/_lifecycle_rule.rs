// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
/// <p>For more information see, <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lifecycle-mgmt.html">Managing your storage lifecycle</a> in the <i>Amazon S3 User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LifecycleRule {
    /// <p>Specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker.</p>
    pub expiration: ::std::option::Option<crate::types::LifecycleExpiration>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>Prefix identifying one or more objects to which the rule applies. This is no longer used; use <code>Filter</code> instead.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[deprecated]
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or <code>And</code> specified. <code>Filter</code> is required if the <code>LifecycleRule</code> does not contain a <code>Prefix</code> element.</p>
    pub filter: ::std::option::Option<crate::types::LifecycleRuleFilter>,
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub status: crate::types::ExpirationStatus,
    /// <p>Specifies when an Amazon S3 object transitions to a specified storage class.</p>
    pub transitions: ::std::option::Option<::std::vec::Vec<crate::types::Transition>>,
    /// <p> Specifies the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to a specific storage class at a set period in the object's lifetime. </p>
    pub noncurrent_version_transitions: ::std::option::Option<::std::vec::Vec<crate::types::NoncurrentVersionTransition>>,
    /// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
    pub noncurrent_version_expiration: ::std::option::Option<crate::types::NoncurrentVersionExpiration>,
    /// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub abort_incomplete_multipart_upload: ::std::option::Option<crate::types::AbortIncompleteMultipartUpload>,
}
impl LifecycleRule {
    /// <p>Specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker.</p>
    pub fn expiration(&self) -> ::std::option::Option<&crate::types::LifecycleExpiration> {
        self.expiration.as_ref()
    }
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Prefix identifying one or more objects to which the rule applies. This is no longer used; use <code>Filter</code> instead.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[deprecated]
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or <code>And</code> specified. <code>Filter</code> is required if the <code>LifecycleRule</code> does not contain a <code>Prefix</code> element.</p>
    pub fn filter(&self) -> ::std::option::Option<&crate::types::LifecycleRuleFilter> {
        self.filter.as_ref()
    }
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub fn status(&self) -> &crate::types::ExpirationStatus {
        &self.status
    }
    /// <p>Specifies when an Amazon S3 object transitions to a specified storage class.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.transitions.is_none()`.
    pub fn transitions(&self) -> &[crate::types::Transition] {
        self.transitions.as_deref().unwrap_or_default()
    }
    /// <p> Specifies the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to a specific storage class at a set period in the object's lifetime. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.noncurrent_version_transitions.is_none()`.
    pub fn noncurrent_version_transitions(&self) -> &[crate::types::NoncurrentVersionTransition] {
        self.noncurrent_version_transitions.as_deref().unwrap_or_default()
    }
    /// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
    pub fn noncurrent_version_expiration(&self) -> ::std::option::Option<&crate::types::NoncurrentVersionExpiration> {
        self.noncurrent_version_expiration.as_ref()
    }
    /// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn abort_incomplete_multipart_upload(&self) -> ::std::option::Option<&crate::types::AbortIncompleteMultipartUpload> {
        self.abort_incomplete_multipart_upload.as_ref()
    }
}
impl LifecycleRule {
    /// Creates a new builder-style object to manufacture [`LifecycleRule`](crate::types::LifecycleRule).
    pub fn builder() -> crate::types::builders::LifecycleRuleBuilder {
        crate::types::builders::LifecycleRuleBuilder::default()
    }
}

/// A builder for [`LifecycleRule`](crate::types::LifecycleRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LifecycleRuleBuilder {
    pub(crate) expiration: ::std::option::Option<crate::types::LifecycleExpiration>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) filter: ::std::option::Option<crate::types::LifecycleRuleFilter>,
    pub(crate) status: ::std::option::Option<crate::types::ExpirationStatus>,
    pub(crate) transitions: ::std::option::Option<::std::vec::Vec<crate::types::Transition>>,
    pub(crate) noncurrent_version_transitions: ::std::option::Option<::std::vec::Vec<crate::types::NoncurrentVersionTransition>>,
    pub(crate) noncurrent_version_expiration: ::std::option::Option<crate::types::NoncurrentVersionExpiration>,
    pub(crate) abort_incomplete_multipart_upload: ::std::option::Option<crate::types::AbortIncompleteMultipartUpload>,
}
impl LifecycleRuleBuilder {
    /// <p>Specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker.</p>
    pub fn expiration(mut self, input: crate::types::LifecycleExpiration) -> Self {
        self.expiration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker.</p>
    pub fn set_expiration(mut self, input: ::std::option::Option<crate::types::LifecycleExpiration>) -> Self {
        self.expiration = input;
        self
    }
    /// <p>Specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker.</p>
    pub fn get_expiration(&self) -> &::std::option::Option<crate::types::LifecycleExpiration> {
        &self.expiration
    }
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>Prefix identifying one or more objects to which the rule applies. This is no longer used; use <code>Filter</code> instead.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[deprecated]
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Prefix identifying one or more objects to which the rule applies. This is no longer used; use <code>Filter</code> instead.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[deprecated]
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>Prefix identifying one or more objects to which the rule applies. This is no longer used; use <code>Filter</code> instead.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[deprecated]
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or <code>And</code> specified. <code>Filter</code> is required if the <code>LifecycleRule</code> does not contain a <code>Prefix</code> element.</p>
    pub fn filter(mut self, input: crate::types::LifecycleRuleFilter) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or <code>And</code> specified. <code>Filter</code> is required if the <code>LifecycleRule</code> does not contain a <code>Prefix</code> element.</p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::LifecycleRuleFilter>) -> Self {
        self.filter = input;
        self
    }
    /// <p>The <code>Filter</code> is used to identify objects that a Lifecycle Rule applies to. A <code>Filter</code> must have exactly one of <code>Prefix</code>, <code>Tag</code>, or <code>And</code> specified. <code>Filter</code> is required if the <code>LifecycleRule</code> does not contain a <code>Prefix</code> element.</p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::LifecycleRuleFilter> {
        &self.filter
    }
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::ExpirationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ExpirationStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ExpirationStatus> {
        &self.status
    }
    /// Appends an item to `transitions`.
    ///
    /// To override the contents of this collection use [`set_transitions`](Self::set_transitions).
    ///
    /// <p>Specifies when an Amazon S3 object transitions to a specified storage class.</p>
    pub fn transitions(mut self, input: crate::types::Transition) -> Self {
        let mut v = self.transitions.unwrap_or_default();
        v.push(input);
        self.transitions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies when an Amazon S3 object transitions to a specified storage class.</p>
    pub fn set_transitions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Transition>>) -> Self {
        self.transitions = input;
        self
    }
    /// <p>Specifies when an Amazon S3 object transitions to a specified storage class.</p>
    pub fn get_transitions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Transition>> {
        &self.transitions
    }
    /// Appends an item to `noncurrent_version_transitions`.
    ///
    /// To override the contents of this collection use [`set_noncurrent_version_transitions`](Self::set_noncurrent_version_transitions).
    ///
    /// <p> Specifies the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to a specific storage class at a set period in the object's lifetime. </p>
    pub fn noncurrent_version_transitions(mut self, input: crate::types::NoncurrentVersionTransition) -> Self {
        let mut v = self.noncurrent_version_transitions.unwrap_or_default();
        v.push(input);
        self.noncurrent_version_transitions = ::std::option::Option::Some(v);
        self
    }
    /// <p> Specifies the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to a specific storage class at a set period in the object's lifetime. </p>
    pub fn set_noncurrent_version_transitions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NoncurrentVersionTransition>>,
    ) -> Self {
        self.noncurrent_version_transitions = input;
        self
    }
    /// <p> Specifies the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to a specific storage class at a set period in the object's lifetime. </p>
    pub fn get_noncurrent_version_transitions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::NoncurrentVersionTransition>> {
        &self.noncurrent_version_transitions
    }
    /// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
    pub fn noncurrent_version_expiration(mut self, input: crate::types::NoncurrentVersionExpiration) -> Self {
        self.noncurrent_version_expiration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
    pub fn set_noncurrent_version_expiration(mut self, input: ::std::option::Option<crate::types::NoncurrentVersionExpiration>) -> Self {
        self.noncurrent_version_expiration = input;
        self
    }
    /// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
    pub fn get_noncurrent_version_expiration(&self) -> &::std::option::Option<crate::types::NoncurrentVersionExpiration> {
        &self.noncurrent_version_expiration
    }
    /// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn abort_incomplete_multipart_upload(mut self, input: crate::types::AbortIncompleteMultipartUpload) -> Self {
        self.abort_incomplete_multipart_upload = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_abort_incomplete_multipart_upload(mut self, input: ::std::option::Option<crate::types::AbortIncompleteMultipartUpload>) -> Self {
        self.abort_incomplete_multipart_upload = input;
        self
    }
    /// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_abort_incomplete_multipart_upload(&self) -> &::std::option::Option<crate::types::AbortIncompleteMultipartUpload> {
        &self.abort_incomplete_multipart_upload
    }
    /// Consumes the builder and constructs a [`LifecycleRule`](crate::types::LifecycleRule).
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](crate::types::builders::LifecycleRuleBuilder::status)
    pub fn build(self) -> ::std::result::Result<crate::types::LifecycleRule, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::LifecycleRule {
            expiration: self.expiration,
            id: self.id,
            prefix: self.prefix,
            filter: self.filter,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building LifecycleRule",
                )
            })?,
            transitions: self.transitions,
            noncurrent_version_transitions: self.noncurrent_version_transitions,
            noncurrent_version_expiration: self.noncurrent_version_expiration,
            abort_incomplete_multipart_upload: self.abort_incomplete_multipart_upload,
        })
    }
}
