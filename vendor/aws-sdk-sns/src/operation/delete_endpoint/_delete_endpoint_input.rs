// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for DeleteEndpoint action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteEndpointInput {
    /// <p>EndpointArn of endpoint to delete.</p>
    pub endpoint_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteEndpointInput {
    /// <p>EndpointArn of endpoint to delete.</p>
    pub fn endpoint_arn(&self) -> ::std::option::Option<&str> {
        self.endpoint_arn.as_deref()
    }
}
impl DeleteEndpointInput {
    /// Creates a new builder-style object to manufacture [`DeleteEndpointInput`](crate::operation::delete_endpoint::DeleteEndpointInput).
    pub fn builder() -> crate::operation::delete_endpoint::builders::DeleteEndpointInputBuilder {
        crate::operation::delete_endpoint::builders::DeleteEndpointInputBuilder::default()
    }
}

/// A builder for [`DeleteEndpointInput`](crate::operation::delete_endpoint::DeleteEndpointInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteEndpointInputBuilder {
    pub(crate) endpoint_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteEndpointInputBuilder {
    /// <p>EndpointArn of endpoint to delete.</p>
    /// This field is required.
    pub fn endpoint_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>EndpointArn of endpoint to delete.</p>
    pub fn set_endpoint_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint_arn = input;
        self
    }
    /// <p>EndpointArn of endpoint to delete.</p>
    pub fn get_endpoint_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.endpoint_arn
    }
    /// Consumes the builder and constructs a [`DeleteEndpointInput`](crate::operation::delete_endpoint::DeleteEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_endpoint::DeleteEndpointInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_endpoint::DeleteEndpointInput {
            endpoint_arn: self.endpoint_arn,
        })
    }
}
