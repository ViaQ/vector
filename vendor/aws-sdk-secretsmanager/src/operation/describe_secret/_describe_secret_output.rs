// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSecretOutput {
    /// <p>The ARN of the secret.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the secret.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the secret.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, this field is omitted. Secrets created using the console use an KMS key ID.</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether automatic rotation is turned on for this secret.</p>
    /// <p>To turn on rotation, use <code>RotateSecret</code>. To turn off rotation, use <code>CancelRotateSecret</code>.</p>
    pub rotation_enabled: ::std::option::Option<bool>,
    /// <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the secret. </p>
    pub rotation_lambda_arn: ::std::option::Option<::std::string::String>,
    /// <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had rotation turned on, this field is omitted.</p>
    pub rotation_rules: ::std::option::Option<crate::types::RotationRulesType>,
    /// <p>The last date and time that Secrets Manager rotated the secret. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub last_rotated_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The last date and time that this secret was modified in any way.</p>
    pub last_changed_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p>
    pub last_accessed_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this field is omitted. When you delete a secret, Secrets Manager requires a recovery window of at least 7 days before deleting the secret. Some time after the deleted date, Secrets Manager deletes the secret, including all of its versions.</p>
    /// <p>If a secret is scheduled for deletion, then its details, including the encrypted secret value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <code>RestoreSecret</code>.</p>
    pub deleted_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The next rotation is scheduled to occur on or before this date. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub next_rotation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The list of tags attached to the secret. To add tags to a secret, use <code>TagResource</code>. To remove tags, use <code>UntagResource</code>.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>A list of the versions of the secret that have staging labels attached. Versions that don't have staging labels are considered deprecated and Secrets Manager can delete them.</p>
    /// <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three staging labels for rotation are: </p>
    /// <ul>
    /// <li> <p> <code>AWSCURRENT</code>, which indicates the current version of the secret.</p> </li>
    /// <li> <p> <code>AWSPENDING</code>, which indicates the version of the secret that contains new secret information that will become the next current version when rotation finishes.</p> <p>During rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. To check if a secret version exists, call <code>GetSecretValue</code>.</p> </li>
    /// <li> <p> <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. You can use this as the <i>last known good</i> version.</p> </li>
    /// </ul>
    /// <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
    pub version_ids_to_stages: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>>,
    /// <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p>
    pub owning_service: ::std::option::Option<::std::string::String>,
    /// <p>The date the secret was created.</p>
    pub created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p>
    pub primary_region: ::std::option::Option<::std::string::String>,
    /// <p>A list of the replicas of this secret and their status: </p>
    /// <ul>
    /// <li> <p> <code>Failed</code>, which indicates that the replica was not created.</p> </li>
    /// <li> <p> <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p> </li>
    /// <li> <p> <code>InSync</code>, which indicates that the replica was created.</p> </li>
    /// </ul>
    pub replication_status: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>>,
    _request_id: Option<String>,
}
impl DescribeSecretOutput {
    /// <p>The ARN of the secret.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the secret.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the secret.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, this field is omitted. Secrets created using the console use an KMS key ID.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>Specifies whether automatic rotation is turned on for this secret.</p>
    /// <p>To turn on rotation, use <code>RotateSecret</code>. To turn off rotation, use <code>CancelRotateSecret</code>.</p>
    pub fn rotation_enabled(&self) -> ::std::option::Option<bool> {
        self.rotation_enabled
    }
    /// <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the secret. </p>
    pub fn rotation_lambda_arn(&self) -> ::std::option::Option<&str> {
        self.rotation_lambda_arn.as_deref()
    }
    /// <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had rotation turned on, this field is omitted.</p>
    pub fn rotation_rules(&self) -> ::std::option::Option<&crate::types::RotationRulesType> {
        self.rotation_rules.as_ref()
    }
    /// <p>The last date and time that Secrets Manager rotated the secret. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn last_rotated_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_rotated_date.as_ref()
    }
    /// <p>The last date and time that this secret was modified in any way.</p>
    pub fn last_changed_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_changed_date.as_ref()
    }
    /// <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p>
    pub fn last_accessed_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_accessed_date.as_ref()
    }
    /// <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this field is omitted. When you delete a secret, Secrets Manager requires a recovery window of at least 7 days before deleting the secret. Some time after the deleted date, Secrets Manager deletes the secret, including all of its versions.</p>
    /// <p>If a secret is scheduled for deletion, then its details, including the encrypted secret value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <code>RestoreSecret</code>.</p>
    pub fn deleted_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.deleted_date.as_ref()
    }
    /// <p>The next rotation is scheduled to occur on or before this date. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn next_rotation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.next_rotation_date.as_ref()
    }
    /// <p>The list of tags attached to the secret. To add tags to a secret, use <code>TagResource</code>. To remove tags, use <code>UntagResource</code>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>A list of the versions of the secret that have staging labels attached. Versions that don't have staging labels are considered deprecated and Secrets Manager can delete them.</p>
    /// <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three staging labels for rotation are: </p>
    /// <ul>
    /// <li> <p> <code>AWSCURRENT</code>, which indicates the current version of the secret.</p> </li>
    /// <li> <p> <code>AWSPENDING</code>, which indicates the version of the secret that contains new secret information that will become the next current version when rotation finishes.</p> <p>During rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. To check if a secret version exists, call <code>GetSecretValue</code>.</p> </li>
    /// <li> <p> <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. You can use this as the <i>last known good</i> version.</p> </li>
    /// </ul>
    /// <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
    pub fn version_ids_to_stages(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>> {
        self.version_ids_to_stages.as_ref()
    }
    /// <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p>
    pub fn owning_service(&self) -> ::std::option::Option<&str> {
        self.owning_service.as_deref()
    }
    /// <p>The date the secret was created.</p>
    pub fn created_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p>
    pub fn primary_region(&self) -> ::std::option::Option<&str> {
        self.primary_region.as_deref()
    }
    /// <p>A list of the replicas of this secret and their status: </p>
    /// <ul>
    /// <li> <p> <code>Failed</code>, which indicates that the replica was not created.</p> </li>
    /// <li> <p> <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p> </li>
    /// <li> <p> <code>InSync</code>, which indicates that the replica was created.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.replication_status.is_none()`.
    pub fn replication_status(&self) -> &[crate::types::ReplicationStatusType] {
        self.replication_status.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeSecretOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSecretOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSecretOutput`](crate::operation::describe_secret::DescribeSecretOutput).
    pub fn builder() -> crate::operation::describe_secret::builders::DescribeSecretOutputBuilder {
        crate::operation::describe_secret::builders::DescribeSecretOutputBuilder::default()
    }
}

/// A builder for [`DescribeSecretOutput`](crate::operation::describe_secret::DescribeSecretOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeSecretOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) rotation_enabled: ::std::option::Option<bool>,
    pub(crate) rotation_lambda_arn: ::std::option::Option<::std::string::String>,
    pub(crate) rotation_rules: ::std::option::Option<crate::types::RotationRulesType>,
    pub(crate) last_rotated_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_changed_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_accessed_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) deleted_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) next_rotation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) version_ids_to_stages:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>>,
    pub(crate) owning_service: ::std::option::Option<::std::string::String>,
    pub(crate) created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) primary_region: ::std::option::Option<::std::string::String>,
    pub(crate) replication_status: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>>,
    _request_id: Option<String>,
}
impl DescribeSecretOutputBuilder {
    /// <p>The ARN of the secret.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the secret.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the secret.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name of the secret.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the secret.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the secret.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The description of the secret.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the secret.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the secret.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, this field is omitted. Secrets created using the console use an KMS key ID.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, this field is omitted. Secrets created using the console use an KMS key ID.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, this field is omitted. Secrets created using the console use an KMS key ID.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// <p>Specifies whether automatic rotation is turned on for this secret.</p>
    /// <p>To turn on rotation, use <code>RotateSecret</code>. To turn off rotation, use <code>CancelRotateSecret</code>.</p>
    pub fn rotation_enabled(mut self, input: bool) -> Self {
        self.rotation_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether automatic rotation is turned on for this secret.</p>
    /// <p>To turn on rotation, use <code>RotateSecret</code>. To turn off rotation, use <code>CancelRotateSecret</code>.</p>
    pub fn set_rotation_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.rotation_enabled = input;
        self
    }
    /// <p>Specifies whether automatic rotation is turned on for this secret.</p>
    /// <p>To turn on rotation, use <code>RotateSecret</code>. To turn off rotation, use <code>CancelRotateSecret</code>.</p>
    pub fn get_rotation_enabled(&self) -> &::std::option::Option<bool> {
        &self.rotation_enabled
    }
    /// <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the secret. </p>
    pub fn rotation_lambda_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rotation_lambda_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the secret. </p>
    pub fn set_rotation_lambda_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rotation_lambda_arn = input;
        self
    }
    /// <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the secret. </p>
    pub fn get_rotation_lambda_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.rotation_lambda_arn
    }
    /// <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had rotation turned on, this field is omitted.</p>
    pub fn rotation_rules(mut self, input: crate::types::RotationRulesType) -> Self {
        self.rotation_rules = ::std::option::Option::Some(input);
        self
    }
    /// <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had rotation turned on, this field is omitted.</p>
    pub fn set_rotation_rules(mut self, input: ::std::option::Option<crate::types::RotationRulesType>) -> Self {
        self.rotation_rules = input;
        self
    }
    /// <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had rotation turned on, this field is omitted.</p>
    pub fn get_rotation_rules(&self) -> &::std::option::Option<crate::types::RotationRulesType> {
        &self.rotation_rules
    }
    /// <p>The last date and time that Secrets Manager rotated the secret. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn last_rotated_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_rotated_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last date and time that Secrets Manager rotated the secret. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn set_last_rotated_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_rotated_date = input;
        self
    }
    /// <p>The last date and time that Secrets Manager rotated the secret. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn get_last_rotated_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_rotated_date
    }
    /// <p>The last date and time that this secret was modified in any way.</p>
    pub fn last_changed_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_changed_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last date and time that this secret was modified in any way.</p>
    pub fn set_last_changed_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_changed_date = input;
        self
    }
    /// <p>The last date and time that this secret was modified in any way.</p>
    pub fn get_last_changed_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_changed_date
    }
    /// <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p>
    pub fn last_accessed_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_accessed_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p>
    pub fn set_last_accessed_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_accessed_date = input;
        self
    }
    /// <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p>
    pub fn get_last_accessed_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_accessed_date
    }
    /// <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this field is omitted. When you delete a secret, Secrets Manager requires a recovery window of at least 7 days before deleting the secret. Some time after the deleted date, Secrets Manager deletes the secret, including all of its versions.</p>
    /// <p>If a secret is scheduled for deletion, then its details, including the encrypted secret value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <code>RestoreSecret</code>.</p>
    pub fn deleted_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.deleted_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this field is omitted. When you delete a secret, Secrets Manager requires a recovery window of at least 7 days before deleting the secret. Some time after the deleted date, Secrets Manager deletes the secret, including all of its versions.</p>
    /// <p>If a secret is scheduled for deletion, then its details, including the encrypted secret value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <code>RestoreSecret</code>.</p>
    pub fn set_deleted_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.deleted_date = input;
        self
    }
    /// <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this field is omitted. When you delete a secret, Secrets Manager requires a recovery window of at least 7 days before deleting the secret. Some time after the deleted date, Secrets Manager deletes the secret, including all of its versions.</p>
    /// <p>If a secret is scheduled for deletion, then its details, including the encrypted secret value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <code>RestoreSecret</code>.</p>
    pub fn get_deleted_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.deleted_date
    }
    /// <p>The next rotation is scheduled to occur on or before this date. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn next_rotation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.next_rotation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The next rotation is scheduled to occur on or before this date. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn set_next_rotation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.next_rotation_date = input;
        self
    }
    /// <p>The next rotation is scheduled to occur on or before this date. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    pub fn get_next_rotation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.next_rotation_date
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags attached to the secret. To add tags to a secret, use <code>TagResource</code>. To remove tags, use <code>UntagResource</code>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of tags attached to the secret. To add tags to a secret, use <code>TagResource</code>. To remove tags, use <code>UntagResource</code>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The list of tags attached to the secret. To add tags to a secret, use <code>TagResource</code>. To remove tags, use <code>UntagResource</code>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Adds a key-value pair to `version_ids_to_stages`.
    ///
    /// To override the contents of this collection use [`set_version_ids_to_stages`](Self::set_version_ids_to_stages).
    ///
    /// <p>A list of the versions of the secret that have staging labels attached. Versions that don't have staging labels are considered deprecated and Secrets Manager can delete them.</p>
    /// <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three staging labels for rotation are: </p>
    /// <ul>
    /// <li> <p> <code>AWSCURRENT</code>, which indicates the current version of the secret.</p> </li>
    /// <li> <p> <code>AWSPENDING</code>, which indicates the version of the secret that contains new secret information that will become the next current version when rotation finishes.</p> <p>During rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. To check if a secret version exists, call <code>GetSecretValue</code>.</p> </li>
    /// <li> <p> <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. You can use this as the <i>last known good</i> version.</p> </li>
    /// </ul>
    /// <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
    pub fn version_ids_to_stages(mut self, k: impl ::std::convert::Into<::std::string::String>, v: ::std::vec::Vec<::std::string::String>) -> Self {
        let mut hash_map = self.version_ids_to_stages.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.version_ids_to_stages = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A list of the versions of the secret that have staging labels attached. Versions that don't have staging labels are considered deprecated and Secrets Manager can delete them.</p>
    /// <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three staging labels for rotation are: </p>
    /// <ul>
    /// <li> <p> <code>AWSCURRENT</code>, which indicates the current version of the secret.</p> </li>
    /// <li> <p> <code>AWSPENDING</code>, which indicates the version of the secret that contains new secret information that will become the next current version when rotation finishes.</p> <p>During rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. To check if a secret version exists, call <code>GetSecretValue</code>.</p> </li>
    /// <li> <p> <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. You can use this as the <i>last known good</i> version.</p> </li>
    /// </ul>
    /// <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
    pub fn set_version_ids_to_stages(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>>,
    ) -> Self {
        self.version_ids_to_stages = input;
        self
    }
    /// <p>A list of the versions of the secret that have staging labels attached. Versions that don't have staging labels are considered deprecated and Secrets Manager can delete them.</p>
    /// <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three staging labels for rotation are: </p>
    /// <ul>
    /// <li> <p> <code>AWSCURRENT</code>, which indicates the current version of the secret.</p> </li>
    /// <li> <p> <code>AWSPENDING</code>, which indicates the version of the secret that contains new secret information that will become the next current version when rotation finishes.</p> <p>During rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. To check if a secret version exists, call <code>GetSecretValue</code>.</p> </li>
    /// <li> <p> <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. You can use this as the <i>last known good</i> version.</p> </li>
    /// </ul>
    /// <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
    pub fn get_version_ids_to_stages(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>> {
        &self.version_ids_to_stages
    }
    /// <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p>
    pub fn owning_service(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owning_service = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p>
    pub fn set_owning_service(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owning_service = input;
        self
    }
    /// <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p>
    pub fn get_owning_service(&self) -> &::std::option::Option<::std::string::String> {
        &self.owning_service
    }
    /// <p>The date the secret was created.</p>
    pub fn created_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the secret was created.</p>
    pub fn set_created_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_date = input;
        self
    }
    /// <p>The date the secret was created.</p>
    pub fn get_created_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_date
    }
    /// <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p>
    pub fn primary_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.primary_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p>
    pub fn set_primary_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.primary_region = input;
        self
    }
    /// <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p>
    pub fn get_primary_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.primary_region
    }
    /// Appends an item to `replication_status`.
    ///
    /// To override the contents of this collection use [`set_replication_status`](Self::set_replication_status).
    ///
    /// <p>A list of the replicas of this secret and their status: </p>
    /// <ul>
    /// <li> <p> <code>Failed</code>, which indicates that the replica was not created.</p> </li>
    /// <li> <p> <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p> </li>
    /// <li> <p> <code>InSync</code>, which indicates that the replica was created.</p> </li>
    /// </ul>
    pub fn replication_status(mut self, input: crate::types::ReplicationStatusType) -> Self {
        let mut v = self.replication_status.unwrap_or_default();
        v.push(input);
        self.replication_status = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the replicas of this secret and their status: </p>
    /// <ul>
    /// <li> <p> <code>Failed</code>, which indicates that the replica was not created.</p> </li>
    /// <li> <p> <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p> </li>
    /// <li> <p> <code>InSync</code>, which indicates that the replica was created.</p> </li>
    /// </ul>
    pub fn set_replication_status(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>>) -> Self {
        self.replication_status = input;
        self
    }
    /// <p>A list of the replicas of this secret and their status: </p>
    /// <ul>
    /// <li> <p> <code>Failed</code>, which indicates that the replica was not created.</p> </li>
    /// <li> <p> <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p> </li>
    /// <li> <p> <code>InSync</code>, which indicates that the replica was created.</p> </li>
    /// </ul>
    pub fn get_replication_status(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>> {
        &self.replication_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSecretOutput`](crate::operation::describe_secret::DescribeSecretOutput).
    pub fn build(self) -> crate::operation::describe_secret::DescribeSecretOutput {
        crate::operation::describe_secret::DescribeSecretOutput {
            arn: self.arn,
            name: self.name,
            description: self.description,
            kms_key_id: self.kms_key_id,
            rotation_enabled: self.rotation_enabled,
            rotation_lambda_arn: self.rotation_lambda_arn,
            rotation_rules: self.rotation_rules,
            last_rotated_date: self.last_rotated_date,
            last_changed_date: self.last_changed_date,
            last_accessed_date: self.last_accessed_date,
            deleted_date: self.deleted_date,
            next_rotation_date: self.next_rotation_date,
            tags: self.tags,
            version_ids_to_stages: self.version_ids_to_stages,
            owning_service: self.owning_service,
            created_date: self.created_date,
            primary_region: self.primary_region,
            replication_status: self.replication_status,
            _request_id: self._request_id,
        }
    }
}
