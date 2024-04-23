// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateKmsKey`](crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`log_group_name(impl Into<String>)`](crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder::log_group_name) / [`set_log_group_name(Option<String>)`](crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder::set_log_group_name):<br>required: **false**<br><p>The name of the log group.</p>  <p>In your <code>DisassociateKmsKey</code> operation, you must specify either the <code>resourceIdentifier</code> parameter or the <code>logGroup</code> parameter, but you can't specify both.</p><br>
    ///   - [`resource_identifier(impl Into<String>)`](crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder::resource_identifier) / [`set_resource_identifier(Option<String>)`](crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder::set_resource_identifier):<br>required: **false**<br><p>Specifies the target for this operation. You must specify one of the following:</p>  <ul>   <li> <p>Specify the ARN of a log group to stop having CloudWatch Logs use the KMS key to encrypt log events that are ingested and stored by that log group. After you run this operation, CloudWatch Logs encrypts ingested log events with the default CloudWatch Logs method. The log group ARN must be in the following format. Replace <i>REGION</i> and <i>ACCOUNT_ID</i> with your Region and account ID.</p> <p> <code>arn:aws:logs:<i>REGION</i>:<i>ACCOUNT_ID</i>:log-group:<i>LOG_GROUP_NAME</i> </code> </p> </li>   <li> <p>Specify the following ARN to stop using this key to encrypt the results of future <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_StartQuery.html">StartQuery</a> operations in this account. Replace <i>REGION</i> and <i>ACCOUNT_ID</i> with your Region and account ID.</p> <p> <code>arn:aws:logs:<i>REGION</i>:<i>ACCOUNT_ID</i>:query-result:*</code> </p> </li>  </ul>  <p>In your <code>DisssociateKmsKey</code> operation, you must specify either the <code>resourceIdentifier</code> parameter or the <code>logGroup</code> parameter, but you can't specify both.</p><br>
    /// - On success, responds with [`DisassociateKmsKeyOutput`](crate::operation::disassociate_kms_key::DisassociateKmsKeyOutput)
    /// - On failure, responds with [`SdkError<DisassociateKmsKeyError>`](crate::operation::disassociate_kms_key::DisassociateKmsKeyError)
    pub fn disassociate_kms_key(&self) -> crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder {
        crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyFluentBuilder::new(self.handle.clone())
    }
}
