// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVpcEndpoint`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_arn(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::domain_arn) / [`set_domain_arn(Option<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_domain_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the domain to grant access to.</p><br>
    ///   - [`vpc_options(VpcOptions)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::vpc_options) / [`set_vpc_options(Option<VpcOptions>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_vpc_options):<br>required: **true**<br><p>Options to specify the subnets and security groups for the endpoint.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier to ensure idempotency of the request.</p><br>
    /// - On success, responds with [`CreateVpcEndpointOutput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointOutput) with field(s):
    ///   - [`vpc_endpoint(Option<VpcEndpoint>)`](crate::operation::create_vpc_endpoint::CreateVpcEndpointOutput::vpc_endpoint): <p>Information about the newly created VPC endpoint.</p>
    /// - On failure, responds with [`SdkError<CreateVpcEndpointError>`](crate::operation::create_vpc_endpoint::CreateVpcEndpointError)
    pub fn create_vpc_endpoint(&self) -> crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder {
        crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::new(self.handle.clone())
    }
}
