// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DryRunResults {
    /// <p> Specifies the deployment mechanism through which the update shall be applied on the domain. Possible responses are <code>Blue/Green</code> (The update will require a blue/green deployment.) <code>DynamicUpdate</code> (The update can be applied in-place without a Blue/Green deployment required.) <code>Undetermined</code> (The domain is undergoing an update which needs to complete before the deployment type can be predicted.) <code>None</code> (The configuration change matches the current configuration and will not result in any update.) </p>
    pub deployment_type: ::std::option::Option<::std::string::String>,
    /// <p>Contains an optional message associated with the DryRunResults.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl DryRunResults {
    /// <p> Specifies the deployment mechanism through which the update shall be applied on the domain. Possible responses are <code>Blue/Green</code> (The update will require a blue/green deployment.) <code>DynamicUpdate</code> (The update can be applied in-place without a Blue/Green deployment required.) <code>Undetermined</code> (The domain is undergoing an update which needs to complete before the deployment type can be predicted.) <code>None</code> (The configuration change matches the current configuration and will not result in any update.) </p>
    pub fn deployment_type(&self) -> ::std::option::Option<&str> {
        self.deployment_type.as_deref()
    }
    /// <p>Contains an optional message associated with the DryRunResults.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl DryRunResults {
    /// Creates a new builder-style object to manufacture [`DryRunResults`](crate::types::DryRunResults).
    pub fn builder() -> crate::types::builders::DryRunResultsBuilder {
        crate::types::builders::DryRunResultsBuilder::default()
    }
}

/// A builder for [`DryRunResults`](crate::types::DryRunResults).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DryRunResultsBuilder {
    pub(crate) deployment_type: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl DryRunResultsBuilder {
    /// <p> Specifies the deployment mechanism through which the update shall be applied on the domain. Possible responses are <code>Blue/Green</code> (The update will require a blue/green deployment.) <code>DynamicUpdate</code> (The update can be applied in-place without a Blue/Green deployment required.) <code>Undetermined</code> (The domain is undergoing an update which needs to complete before the deployment type can be predicted.) <code>None</code> (The configuration change matches the current configuration and will not result in any update.) </p>
    pub fn deployment_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.deployment_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Specifies the deployment mechanism through which the update shall be applied on the domain. Possible responses are <code>Blue/Green</code> (The update will require a blue/green deployment.) <code>DynamicUpdate</code> (The update can be applied in-place without a Blue/Green deployment required.) <code>Undetermined</code> (The domain is undergoing an update which needs to complete before the deployment type can be predicted.) <code>None</code> (The configuration change matches the current configuration and will not result in any update.) </p>
    pub fn set_deployment_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.deployment_type = input;
        self
    }
    /// <p> Specifies the deployment mechanism through which the update shall be applied on the domain. Possible responses are <code>Blue/Green</code> (The update will require a blue/green deployment.) <code>DynamicUpdate</code> (The update can be applied in-place without a Blue/Green deployment required.) <code>Undetermined</code> (The domain is undergoing an update which needs to complete before the deployment type can be predicted.) <code>None</code> (The configuration change matches the current configuration and will not result in any update.) </p>
    pub fn get_deployment_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.deployment_type
    }
    /// <p>Contains an optional message associated with the DryRunResults.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains an optional message associated with the DryRunResults.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>Contains an optional message associated with the DryRunResults.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`DryRunResults`](crate::types::DryRunResults).
    pub fn build(self) -> crate::types::DryRunResults {
        crate::types::DryRunResults {
            deployment_type: self.deployment_type,
            message: self.message,
        }
    }
}
