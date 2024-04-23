// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the location where the bucket will be created.</p>
/// <p>For directory buckets, the location type is Availability Zone. For more information about directory buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-buckets-overview.html">Directory buckets</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
/// <p>This functionality is only supported by directory buckets.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LocationInfo {
    /// <p>The type of location where the bucket will be created.</p>
    pub r#type: ::std::option::Option<crate::types::LocationType>,
    /// <p>The name of the location where the bucket will be created.</p>
    /// <p>For directory buckets, the AZ ID of the Availability Zone where the bucket will be created. An example AZ ID value is <code>usw2-az2</code>.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl LocationInfo {
    /// <p>The type of location where the bucket will be created.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::LocationType> {
        self.r#type.as_ref()
    }
    /// <p>The name of the location where the bucket will be created.</p>
    /// <p>For directory buckets, the AZ ID of the Availability Zone where the bucket will be created. An example AZ ID value is <code>usw2-az2</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl LocationInfo {
    /// Creates a new builder-style object to manufacture [`LocationInfo`](crate::types::LocationInfo).
    pub fn builder() -> crate::types::builders::LocationInfoBuilder {
        crate::types::builders::LocationInfoBuilder::default()
    }
}

/// A builder for [`LocationInfo`](crate::types::LocationInfo).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LocationInfoBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::LocationType>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl LocationInfoBuilder {
    /// <p>The type of location where the bucket will be created.</p>
    pub fn r#type(mut self, input: crate::types::LocationType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of location where the bucket will be created.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::LocationType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of location where the bucket will be created.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::LocationType> {
        &self.r#type
    }
    /// <p>The name of the location where the bucket will be created.</p>
    /// <p>For directory buckets, the AZ ID of the Availability Zone where the bucket will be created. An example AZ ID value is <code>usw2-az2</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the location where the bucket will be created.</p>
    /// <p>For directory buckets, the AZ ID of the Availability Zone where the bucket will be created. An example AZ ID value is <code>usw2-az2</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the location where the bucket will be created.</p>
    /// <p>For directory buckets, the AZ ID of the Availability Zone where the bucket will be created. An example AZ ID value is <code>usw2-az2</code>.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`LocationInfo`](crate::types::LocationInfo).
    pub fn build(self) -> crate::types::LocationInfo {
        crate::types::LocationInfo {
            r#type: self.r#type,
            name: self.name,
        }
    }
}
