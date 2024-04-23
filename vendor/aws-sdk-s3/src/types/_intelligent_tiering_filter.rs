// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The <code>Filter</code> is used to identify objects that the S3 Intelligent-Tiering configuration applies to.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IntelligentTieringFilter {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>A container of a key value name pair.</p>
    pub tag: ::std::option::Option<crate::types::Tag>,
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub and: ::std::option::Option<crate::types::IntelligentTieringAndOperator>,
}
impl IntelligentTieringFilter {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>A container of a key value name pair.</p>
    pub fn tag(&self) -> ::std::option::Option<&crate::types::Tag> {
        self.tag.as_ref()
    }
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub fn and(&self) -> ::std::option::Option<&crate::types::IntelligentTieringAndOperator> {
        self.and.as_ref()
    }
}
impl IntelligentTieringFilter {
    /// Creates a new builder-style object to manufacture [`IntelligentTieringFilter`](crate::types::IntelligentTieringFilter).
    pub fn builder() -> crate::types::builders::IntelligentTieringFilterBuilder {
        crate::types::builders::IntelligentTieringFilterBuilder::default()
    }
}

/// A builder for [`IntelligentTieringFilter`](crate::types::IntelligentTieringFilter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IntelligentTieringFilterBuilder {
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) tag: ::std::option::Option<crate::types::Tag>,
    pub(crate) and: ::std::option::Option<crate::types::IntelligentTieringAndOperator>,
}
impl IntelligentTieringFilterBuilder {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// <p>A container of a key value name pair.</p>
    pub fn tag(mut self, input: crate::types::Tag) -> Self {
        self.tag = ::std::option::Option::Some(input);
        self
    }
    /// <p>A container of a key value name pair.</p>
    pub fn set_tag(mut self, input: ::std::option::Option<crate::types::Tag>) -> Self {
        self.tag = input;
        self
    }
    /// <p>A container of a key value name pair.</p>
    pub fn get_tag(&self) -> &::std::option::Option<crate::types::Tag> {
        &self.tag
    }
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub fn and(mut self, input: crate::types::IntelligentTieringAndOperator) -> Self {
        self.and = ::std::option::Option::Some(input);
        self
    }
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub fn set_and(mut self, input: ::std::option::Option<crate::types::IntelligentTieringAndOperator>) -> Self {
        self.and = input;
        self
    }
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub fn get_and(&self) -> &::std::option::Option<crate::types::IntelligentTieringAndOperator> {
        &self.and
    }
    /// Consumes the builder and constructs a [`IntelligentTieringFilter`](crate::types::IntelligentTieringFilter).
    pub fn build(self) -> crate::types::IntelligentTieringFilter {
        crate::types::IntelligentTieringFilter {
            prefix: self.prefix,
            tag: self.tag,
            and: self.and,
        }
    }
}
