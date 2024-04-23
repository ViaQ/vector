// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. For more information about when Amazon S3 considers a bucket or object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PublicAccessBlockConfiguration {
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes the following behavior:</p>
    /// <ul>
    /// <li> <p>PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is public.</p> </li>
    /// <li> <p>PUT Object calls fail if the request includes a public ACL.</p> </li>
    /// <li> <p>PUT Bucket calls fail if the request includes a public ACL.</p> </li>
    /// </ul>
    /// <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub block_public_acls: ::std::option::Option<bool>,
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on this bucket and objects in this bucket.</p>
    /// <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set.</p>
    pub ignore_public_acls: ::std::option::Option<bool>,
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the specified bucket policy allows public access. </p>
    /// <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub block_public_policy: ::std::option::Option<bool>,
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting this element to <code>TRUE</code> restricts access to this bucket to only Amazon Web Service principals and authorized users within this account if the bucket has a public policy.</p>
    /// <p>Enabling this setting doesn't affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked.</p>
    pub restrict_public_buckets: ::std::option::Option<bool>,
}
impl PublicAccessBlockConfiguration {
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes the following behavior:</p>
    /// <ul>
    /// <li> <p>PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is public.</p> </li>
    /// <li> <p>PUT Object calls fail if the request includes a public ACL.</p> </li>
    /// <li> <p>PUT Bucket calls fail if the request includes a public ACL.</p> </li>
    /// </ul>
    /// <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub fn block_public_acls(&self) -> ::std::option::Option<bool> {
        self.block_public_acls
    }
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on this bucket and objects in this bucket.</p>
    /// <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set.</p>
    pub fn ignore_public_acls(&self) -> ::std::option::Option<bool> {
        self.ignore_public_acls
    }
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the specified bucket policy allows public access. </p>
    /// <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub fn block_public_policy(&self) -> ::std::option::Option<bool> {
        self.block_public_policy
    }
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting this element to <code>TRUE</code> restricts access to this bucket to only Amazon Web Service principals and authorized users within this account if the bucket has a public policy.</p>
    /// <p>Enabling this setting doesn't affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked.</p>
    pub fn restrict_public_buckets(&self) -> ::std::option::Option<bool> {
        self.restrict_public_buckets
    }
}
impl PublicAccessBlockConfiguration {
    /// Creates a new builder-style object to manufacture [`PublicAccessBlockConfiguration`](crate::types::PublicAccessBlockConfiguration).
    pub fn builder() -> crate::types::builders::PublicAccessBlockConfigurationBuilder {
        crate::types::builders::PublicAccessBlockConfigurationBuilder::default()
    }
}

/// A builder for [`PublicAccessBlockConfiguration`](crate::types::PublicAccessBlockConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PublicAccessBlockConfigurationBuilder {
    pub(crate) block_public_acls: ::std::option::Option<bool>,
    pub(crate) ignore_public_acls: ::std::option::Option<bool>,
    pub(crate) block_public_policy: ::std::option::Option<bool>,
    pub(crate) restrict_public_buckets: ::std::option::Option<bool>,
}
impl PublicAccessBlockConfigurationBuilder {
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes the following behavior:</p>
    /// <ul>
    /// <li> <p>PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is public.</p> </li>
    /// <li> <p>PUT Object calls fail if the request includes a public ACL.</p> </li>
    /// <li> <p>PUT Bucket calls fail if the request includes a public ACL.</p> </li>
    /// </ul>
    /// <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub fn block_public_acls(mut self, input: bool) -> Self {
        self.block_public_acls = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes the following behavior:</p>
    /// <ul>
    /// <li> <p>PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is public.</p> </li>
    /// <li> <p>PUT Object calls fail if the request includes a public ACL.</p> </li>
    /// <li> <p>PUT Bucket calls fail if the request includes a public ACL.</p> </li>
    /// </ul>
    /// <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub fn set_block_public_acls(mut self, input: ::std::option::Option<bool>) -> Self {
        self.block_public_acls = input;
        self
    }
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes the following behavior:</p>
    /// <ul>
    /// <li> <p>PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is public.</p> </li>
    /// <li> <p>PUT Object calls fail if the request includes a public ACL.</p> </li>
    /// <li> <p>PUT Bucket calls fail if the request includes a public ACL.</p> </li>
    /// </ul>
    /// <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub fn get_block_public_acls(&self) -> &::std::option::Option<bool> {
        &self.block_public_acls
    }
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on this bucket and objects in this bucket.</p>
    /// <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set.</p>
    pub fn ignore_public_acls(mut self, input: bool) -> Self {
        self.ignore_public_acls = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on this bucket and objects in this bucket.</p>
    /// <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set.</p>
    pub fn set_ignore_public_acls(mut self, input: ::std::option::Option<bool>) -> Self {
        self.ignore_public_acls = input;
        self
    }
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on this bucket and objects in this bucket.</p>
    /// <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set.</p>
    pub fn get_ignore_public_acls(&self) -> &::std::option::Option<bool> {
        &self.ignore_public_acls
    }
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the specified bucket policy allows public access. </p>
    /// <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub fn block_public_policy(mut self, input: bool) -> Self {
        self.block_public_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the specified bucket policy allows public access. </p>
    /// <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub fn set_block_public_policy(mut self, input: ::std::option::Option<bool>) -> Self {
        self.block_public_policy = input;
        self
    }
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the specified bucket policy allows public access. </p>
    /// <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub fn get_block_public_policy(&self) -> &::std::option::Option<bool> {
        &self.block_public_policy
    }
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting this element to <code>TRUE</code> restricts access to this bucket to only Amazon Web Service principals and authorized users within this account if the bucket has a public policy.</p>
    /// <p>Enabling this setting doesn't affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked.</p>
    pub fn restrict_public_buckets(mut self, input: bool) -> Self {
        self.restrict_public_buckets = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting this element to <code>TRUE</code> restricts access to this bucket to only Amazon Web Service principals and authorized users within this account if the bucket has a public policy.</p>
    /// <p>Enabling this setting doesn't affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked.</p>
    pub fn set_restrict_public_buckets(mut self, input: ::std::option::Option<bool>) -> Self {
        self.restrict_public_buckets = input;
        self
    }
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting this element to <code>TRUE</code> restricts access to this bucket to only Amazon Web Service principals and authorized users within this account if the bucket has a public policy.</p>
    /// <p>Enabling this setting doesn't affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked.</p>
    pub fn get_restrict_public_buckets(&self) -> &::std::option::Option<bool> {
        &self.restrict_public_buckets
    }
    /// Consumes the builder and constructs a [`PublicAccessBlockConfiguration`](crate::types::PublicAccessBlockConfiguration).
    pub fn build(self) -> crate::types::PublicAccessBlockConfiguration {
        crate::types::PublicAccessBlockConfiguration {
            block_public_acls: self.block_public_acls,
            ignore_public_acls: self.ignore_public_acls,
            block_public_policy: self.block_public_policy,
            restrict_public_buckets: self.restrict_public_buckets,
        }
    }
}
