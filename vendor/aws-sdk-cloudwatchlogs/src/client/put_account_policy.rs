// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAccountPolicy`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_name(impl Into<String>)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::set_policy_name):<br>required: **true**<br><p>A name for the policy. This must be unique within the account.</p><br>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::set_policy_document):<br>required: **true**<br><p>Specify the data protection policy, in JSON.</p>  <p>This policy must include two JSON blocks:</p>  <ul>   <li> <p>The first block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Audit</code> action. The <code>DataIdentifer</code> array lists the types of sensitive data that you want to mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that you can mask</a>.</p> <p>The <code>Operation</code> property with an <code>Audit</code> action is required to find the sensitive data terms. This <code>Audit</code> action must contain a <code>FindingsDestination</code> object. You can optionally use that <code>FindingsDestination</code> object to list one or more destinations to send audit findings to. If you specify destinations such as log groups, Kinesis Data Firehose streams, and S3 buckets, they must already exist.</p> </li>   <li> <p>The second block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Deidentify</code> action. The <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array in the first block of the policy.</p> <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object. The <code> "MaskConfig": {}</code> object must be empty.</p> </li>  </ul>  <p>For an example data protection policy, see the <b>Examples</b> section on this page.</p> <important>   <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>  </important>  <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The <code>Name</code> is different than the operation's <code>policyName</code> parameter, and is used as a dimension when CloudWatch Logs reports audit findings metrics to CloudWatch.</p>  <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters.</p><br>
    ///   - [`policy_type(PolicyType)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::policy_type) / [`set_policy_type(Option<PolicyType>)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::set_policy_type):<br>required: **true**<br><p>Currently the only valid value for this parameter is <code>DATA_PROTECTION_POLICY</code>.</p><br>
    ///   - [`scope(Scope)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::scope) / [`set_scope(Option<Scope>)`](crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::set_scope):<br>required: **false**<br><p>Currently the only valid value for this parameter is <code>ALL</code>, which specifies that the data protection policy applies to all log groups in the account. If you omit this parameter, the default of <code>ALL</code> is used.</p><br>
    /// - On success, responds with [`PutAccountPolicyOutput`](crate::operation::put_account_policy::PutAccountPolicyOutput) with field(s):
    ///   - [`account_policy(Option<AccountPolicy>)`](crate::operation::put_account_policy::PutAccountPolicyOutput::account_policy): <p>The account policy that you created.</p>
    /// - On failure, responds with [`SdkError<PutAccountPolicyError>`](crate::operation::put_account_policy::PutAccountPolicyError)
    pub fn put_account_policy(&self) -> crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder {
        crate::operation::put_account_policy::builders::PutAccountPolicyFluentBuilder::new(self.handle.clone())
    }
}
