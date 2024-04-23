// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates how to transform ingested log events to metric data in a CloudWatch metric.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetricTransformation {
    /// <p>The name of the CloudWatch metric.</p>
    pub metric_name: ::std::string::String,
    /// <p>A custom namespace to contain your metric in CloudWatch. Use namespaces to group together metrics that are similar. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Namespace">Namespaces</a>.</p>
    pub metric_namespace: ::std::string::String,
    /// <p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>
    pub metric_value: ::std::string::String,
    /// <p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>
    pub default_value: ::std::option::Option<f64>,
    /// <p>The fields to use as dimensions for the metric. One metric filter can include as many as three dimensions.</p> <important>
    /// <p>Metrics extracted from log events are charged as custom metrics. To prevent unexpected high charges, do not specify high-cardinality fields such as <code>IPAddress</code> or <code>requestID</code> as dimensions. Each different value found for a dimension is treated as a separate metric and accrues charges as a separate custom metric. </p>
    /// <p>CloudWatch Logs disables a metric filter if it generates 1000 different name/value pairs for your specified dimensions within a certain amount of time. This helps to prevent accidental high charges.</p>
    /// <p>You can also set up a billing alarm to alert you if your charges are higher than expected. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/monitor_estimated_charges_with_cloudwatch.html"> Creating a Billing Alarm to Monitor Your Estimated Amazon Web Services Charges</a>. </p>
    /// </important>
    pub dimensions: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>The unit to assign to the metric. If you omit this, the unit is set as <code>None</code>.</p>
    pub unit: ::std::option::Option<crate::types::StandardUnit>,
}
impl MetricTransformation {
    /// <p>The name of the CloudWatch metric.</p>
    pub fn metric_name(&self) -> &str {
        use std::ops::Deref;
        self.metric_name.deref()
    }
    /// <p>A custom namespace to contain your metric in CloudWatch. Use namespaces to group together metrics that are similar. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Namespace">Namespaces</a>.</p>
    pub fn metric_namespace(&self) -> &str {
        use std::ops::Deref;
        self.metric_namespace.deref()
    }
    /// <p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>
    pub fn metric_value(&self) -> &str {
        use std::ops::Deref;
        self.metric_value.deref()
    }
    /// <p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>
    pub fn default_value(&self) -> ::std::option::Option<f64> {
        self.default_value
    }
    /// <p>The fields to use as dimensions for the metric. One metric filter can include as many as three dimensions.</p> <important>
    /// <p>Metrics extracted from log events are charged as custom metrics. To prevent unexpected high charges, do not specify high-cardinality fields such as <code>IPAddress</code> or <code>requestID</code> as dimensions. Each different value found for a dimension is treated as a separate metric and accrues charges as a separate custom metric. </p>
    /// <p>CloudWatch Logs disables a metric filter if it generates 1000 different name/value pairs for your specified dimensions within a certain amount of time. This helps to prevent accidental high charges.</p>
    /// <p>You can also set up a billing alarm to alert you if your charges are higher than expected. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/monitor_estimated_charges_with_cloudwatch.html"> Creating a Billing Alarm to Monitor Your Estimated Amazon Web Services Charges</a>. </p>
    /// </important>
    pub fn dimensions(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.dimensions.as_ref()
    }
    /// <p>The unit to assign to the metric. If you omit this, the unit is set as <code>None</code>.</p>
    pub fn unit(&self) -> ::std::option::Option<&crate::types::StandardUnit> {
        self.unit.as_ref()
    }
}
impl MetricTransformation {
    /// Creates a new builder-style object to manufacture [`MetricTransformation`](crate::types::MetricTransformation).
    pub fn builder() -> crate::types::builders::MetricTransformationBuilder {
        crate::types::builders::MetricTransformationBuilder::default()
    }
}

/// A builder for [`MetricTransformation`](crate::types::MetricTransformation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MetricTransformationBuilder {
    pub(crate) metric_name: ::std::option::Option<::std::string::String>,
    pub(crate) metric_namespace: ::std::option::Option<::std::string::String>,
    pub(crate) metric_value: ::std::option::Option<::std::string::String>,
    pub(crate) default_value: ::std::option::Option<f64>,
    pub(crate) dimensions: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) unit: ::std::option::Option<crate::types::StandardUnit>,
}
impl MetricTransformationBuilder {
    /// <p>The name of the CloudWatch metric.</p>
    /// This field is required.
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the CloudWatch metric.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric_name = input;
        self
    }
    /// <p>The name of the CloudWatch metric.</p>
    pub fn get_metric_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.metric_name
    }
    /// <p>A custom namespace to contain your metric in CloudWatch. Use namespaces to group together metrics that are similar. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Namespace">Namespaces</a>.</p>
    /// This field is required.
    pub fn metric_namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric_namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A custom namespace to contain your metric in CloudWatch. Use namespaces to group together metrics that are similar. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Namespace">Namespaces</a>.</p>
    pub fn set_metric_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric_namespace = input;
        self
    }
    /// <p>A custom namespace to contain your metric in CloudWatch. Use namespaces to group together metrics that are similar. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Namespace">Namespaces</a>.</p>
    pub fn get_metric_namespace(&self) -> &::std::option::Option<::std::string::String> {
        &self.metric_namespace
    }
    /// <p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>
    /// This field is required.
    pub fn metric_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>
    pub fn set_metric_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric_value = input;
        self
    }
    /// <p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>
    pub fn get_metric_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.metric_value
    }
    /// <p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>
    pub fn default_value(mut self, input: f64) -> Self {
        self.default_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>
    pub fn set_default_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.default_value = input;
        self
    }
    /// <p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>
    pub fn get_default_value(&self) -> &::std::option::Option<f64> {
        &self.default_value
    }
    /// Adds a key-value pair to `dimensions`.
    ///
    /// To override the contents of this collection use [`set_dimensions`](Self::set_dimensions).
    ///
    /// <p>The fields to use as dimensions for the metric. One metric filter can include as many as three dimensions.</p> <important>
    /// <p>Metrics extracted from log events are charged as custom metrics. To prevent unexpected high charges, do not specify high-cardinality fields such as <code>IPAddress</code> or <code>requestID</code> as dimensions. Each different value found for a dimension is treated as a separate metric and accrues charges as a separate custom metric. </p>
    /// <p>CloudWatch Logs disables a metric filter if it generates 1000 different name/value pairs for your specified dimensions within a certain amount of time. This helps to prevent accidental high charges.</p>
    /// <p>You can also set up a billing alarm to alert you if your charges are higher than expected. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/monitor_estimated_charges_with_cloudwatch.html"> Creating a Billing Alarm to Monitor Your Estimated Amazon Web Services Charges</a>. </p>
    /// </important>
    pub fn dimensions(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.dimensions.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.dimensions = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The fields to use as dimensions for the metric. One metric filter can include as many as three dimensions.</p> <important>
    /// <p>Metrics extracted from log events are charged as custom metrics. To prevent unexpected high charges, do not specify high-cardinality fields such as <code>IPAddress</code> or <code>requestID</code> as dimensions. Each different value found for a dimension is treated as a separate metric and accrues charges as a separate custom metric. </p>
    /// <p>CloudWatch Logs disables a metric filter if it generates 1000 different name/value pairs for your specified dimensions within a certain amount of time. This helps to prevent accidental high charges.</p>
    /// <p>You can also set up a billing alarm to alert you if your charges are higher than expected. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/monitor_estimated_charges_with_cloudwatch.html"> Creating a Billing Alarm to Monitor Your Estimated Amazon Web Services Charges</a>. </p>
    /// </important>
    pub fn set_dimensions(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.dimensions = input;
        self
    }
    /// <p>The fields to use as dimensions for the metric. One metric filter can include as many as three dimensions.</p> <important>
    /// <p>Metrics extracted from log events are charged as custom metrics. To prevent unexpected high charges, do not specify high-cardinality fields such as <code>IPAddress</code> or <code>requestID</code> as dimensions. Each different value found for a dimension is treated as a separate metric and accrues charges as a separate custom metric. </p>
    /// <p>CloudWatch Logs disables a metric filter if it generates 1000 different name/value pairs for your specified dimensions within a certain amount of time. This helps to prevent accidental high charges.</p>
    /// <p>You can also set up a billing alarm to alert you if your charges are higher than expected. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/monitor_estimated_charges_with_cloudwatch.html"> Creating a Billing Alarm to Monitor Your Estimated Amazon Web Services Charges</a>. </p>
    /// </important>
    pub fn get_dimensions(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.dimensions
    }
    /// <p>The unit to assign to the metric. If you omit this, the unit is set as <code>None</code>.</p>
    pub fn unit(mut self, input: crate::types::StandardUnit) -> Self {
        self.unit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The unit to assign to the metric. If you omit this, the unit is set as <code>None</code>.</p>
    pub fn set_unit(mut self, input: ::std::option::Option<crate::types::StandardUnit>) -> Self {
        self.unit = input;
        self
    }
    /// <p>The unit to assign to the metric. If you omit this, the unit is set as <code>None</code>.</p>
    pub fn get_unit(&self) -> &::std::option::Option<crate::types::StandardUnit> {
        &self.unit
    }
    /// Consumes the builder and constructs a [`MetricTransformation`](crate::types::MetricTransformation).
    /// This method will fail if any of the following fields are not set:
    /// - [`metric_name`](crate::types::builders::MetricTransformationBuilder::metric_name)
    /// - [`metric_namespace`](crate::types::builders::MetricTransformationBuilder::metric_namespace)
    /// - [`metric_value`](crate::types::builders::MetricTransformationBuilder::metric_value)
    pub fn build(self) -> ::std::result::Result<crate::types::MetricTransformation, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::MetricTransformation {
            metric_name: self.metric_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "metric_name",
                    "metric_name was not specified but it is required when building MetricTransformation",
                )
            })?,
            metric_namespace: self.metric_namespace.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "metric_namespace",
                    "metric_namespace was not specified but it is required when building MetricTransformation",
                )
            })?,
            metric_value: self.metric_value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "metric_value",
                    "metric_value was not specified but it is required when building MetricTransformation",
                )
            })?,
            default_value: self.default_value,
            dimensions: self.dimensions,
            unit: self.unit,
        })
    }
}
