// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutSecretValue`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`secret_id(impl Into<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::secret_id) / [`set_secret_id(Option<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::set_secret_id):<br>required: **true**<br><p>The ARN or name of the secret to add a new version to.</p>  <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>  <p>If the secret doesn't already exist, use <code>CreateSecret</code> instead.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::set_client_request_token):<br>required: **false**<br><p>A unique identifier for the new version of the secret. </p> <note>   <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDKs to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes it as the value for this parameter in the request. </p>  </note>  <p>If you generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> and include it in the request.</p>  <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret. </p>  <ul>   <li> <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created. </p> </li>   <li> <p>If a version with this value already exists and that version's <code>SecretString</code> or <code>SecretBinary</code> values are the same as those in the request then the request is ignored. The operation is idempotent. </p> </li>   <li> <p>If a version with this value already exists and the version of the <code>SecretString</code> and <code>SecretBinary</code> values are different from those in the request, then the request fails because you can't modify a secret version. You can only create new versions to store new secret values.</p> </li>  </ul>  <p>This value becomes the <code>VersionId</code> of the new version.</p><br>
    ///   - [`secret_binary(Blob)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::secret_binary) / [`set_secret_binary(Option<Blob>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::set_secret_binary):<br>required: **false**<br><p>The binary data to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then pass the contents of the file as a parameter. </p>  <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>  <p>You can't access this value from the Secrets Manager console.</p><br>
    ///   - [`secret_string(impl Into<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::secret_string) / [`set_secret_string(Option<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::set_secret_string):<br>required: **false**<br><p>The text to encrypt and store in the new version of the secret. </p>  <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>  <p>We recommend you create the secret string as JSON key/value pairs, as shown in the example.</p><br>
    ///   - [`version_stages(impl Into<String>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::version_stages) / [`set_version_stages(Option<Vec::<String>>)`](crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::set_version_stages):<br>required: **false**<br><p>A list of staging labels to attach to this version of the secret. Secrets Manager uses staging labels to track versions of a secret through the rotation process.</p>  <p>If you specify a staging label that's already associated with a different version of the same secret, then Secrets Manager removes the label from the other version and attaches it to this version. If you specify <code>AWSCURRENT</code>, and it is already attached to another version, then Secrets Manager also moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p>  <p>If you don't include <code>VersionStages</code>, then Secrets Manager automatically moves the staging label <code>AWSCURRENT</code> to this version.</p><br>
    /// - On success, responds with [`PutSecretValueOutput`](crate::operation::put_secret_value::PutSecretValueOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::put_secret_value::PutSecretValueOutput::arn): <p>The ARN of the secret.</p>
    ///   - [`name(Option<String>)`](crate::operation::put_secret_value::PutSecretValueOutput::name): <p>The name of the secret.</p>
    ///   - [`version_id(Option<String>)`](crate::operation::put_secret_value::PutSecretValueOutput::version_id): <p>The unique identifier of the version of the secret.</p>
    ///   - [`version_stages(Option<Vec::<String>>)`](crate::operation::put_secret_value::PutSecretValueOutput::version_stages): <p>The list of staging labels that are currently attached to this version of the secret. Secrets Manager uses staging labels to track a version as it progresses through the secret rotation process.</p>
    /// - On failure, responds with [`SdkError<PutSecretValueError>`](crate::operation::put_secret_value::PutSecretValueError)
    pub fn put_secret_value(&self) -> crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder {
        crate::operation::put_secret_value::builders::PutSecretValueFluentBuilder::new(self.handle.clone())
    }
}
