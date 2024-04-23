// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure contains information about one <i>delivery source</i> in your account. A delivery source is an Amazon Web Services resource that sends logs to an Amazon Web Services destination. The destination can be CloudWatch Logs, Amazon S3, or Kinesis Data Firehose.</p>
/// <p>Only some Amazon Web Services services support being configured as a delivery source. These services are listed as <b>Supported [V2 Permissions]</b> in the table at <a href="https://docs.aws.amazon.com/     AmazonCloudWatch/latest/logs/AWS-logs-and-resource-policy.html#AWS-vended-logs-permissions">Enabling logging from Amazon Web Services services.</a> </p>
/// <p>To configure logs delivery between a supported Amazon Web Services service and a destination, you must do the following:</p>
/// <ul>
/// <li> <p>Create a delivery source, which is a logical object that represents the resource that is actually sending the logs. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDeliverySource.html">PutDeliverySource</a>.</p> </li>
/// <li> <p>Create a <i>delivery destination</i>, which is a logical object that represents the actual delivery destination. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDeliveryDestination.html">PutDeliveryDestination</a>.</p> </li>
/// <li> <p>If you are delivering logs cross-account, you must use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDeliveryDestinationolicy.html">PutDeliveryDestinationPolicy</a> in the destination account to assign an IAM policy to the destination. This policy allows delivery to that destination. </p> </li>
/// <li> <p>Create a <i>delivery</i> by pairing exactly one delivery source and one delivery destination. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_CreateDelivery.html">CreateDelivery</a>.</p> </li>
/// </ul>
/// <p>You can configure a single delivery source to send logs to multiple destinations by creating multiple deliveries. You can also create multiple deliveries to configure multiple delivery sources to send logs to the same delivery destination.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeliverySource {
    /// <p>The unique name of the delivery source.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies this delivery source.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>This array contains the ARN of the Amazon Web Services resource that sends logs and is represented by this delivery source. Currently, only one ARN can be in the array.</p>
    pub resource_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The Amazon Web Services service that is sending logs.</p>
    pub service: ::std::option::Option<::std::string::String>,
    /// <p>The type of log that the source is sending. For valid values for this parameter, see the documentation for the source service.</p>
    pub log_type: ::std::option::Option<::std::string::String>,
    /// <p>The tags that have been assigned to this delivery source.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl DeliverySource {
    /// <p>The unique name of the delivery source.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies this delivery source.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>This array contains the ARN of the Amazon Web Services resource that sends logs and is represented by this delivery source. Currently, only one ARN can be in the array.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.resource_arns.is_none()`.
    pub fn resource_arns(&self) -> &[::std::string::String] {
        self.resource_arns.as_deref().unwrap_or_default()
    }
    /// <p>The Amazon Web Services service that is sending logs.</p>
    pub fn service(&self) -> ::std::option::Option<&str> {
        self.service.as_deref()
    }
    /// <p>The type of log that the source is sending. For valid values for this parameter, see the documentation for the source service.</p>
    pub fn log_type(&self) -> ::std::option::Option<&str> {
        self.log_type.as_deref()
    }
    /// <p>The tags that have been assigned to this delivery source.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl DeliverySource {
    /// Creates a new builder-style object to manufacture [`DeliverySource`](crate::types::DeliverySource).
    pub fn builder() -> crate::types::builders::DeliverySourceBuilder {
        crate::types::builders::DeliverySourceBuilder::default()
    }
}

/// A builder for [`DeliverySource`](crate::types::DeliverySource).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeliverySourceBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) service: ::std::option::Option<::std::string::String>,
    pub(crate) log_type: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl DeliverySourceBuilder {
    /// <p>The unique name of the delivery source.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique name of the delivery source.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The unique name of the delivery source.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies this delivery source.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies this delivery source.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies this delivery source.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Appends an item to `resource_arns`.
    ///
    /// To override the contents of this collection use [`set_resource_arns`](Self::set_resource_arns).
    ///
    /// <p>This array contains the ARN of the Amazon Web Services resource that sends logs and is represented by this delivery source. Currently, only one ARN can be in the array.</p>
    pub fn resource_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.resource_arns.unwrap_or_default();
        v.push(input.into());
        self.resource_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>This array contains the ARN of the Amazon Web Services resource that sends logs and is represented by this delivery source. Currently, only one ARN can be in the array.</p>
    pub fn set_resource_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.resource_arns = input;
        self
    }
    /// <p>This array contains the ARN of the Amazon Web Services resource that sends logs and is represented by this delivery source. Currently, only one ARN can be in the array.</p>
    pub fn get_resource_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.resource_arns
    }
    /// <p>The Amazon Web Services service that is sending logs.</p>
    pub fn service(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services service that is sending logs.</p>
    pub fn set_service(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service = input;
        self
    }
    /// <p>The Amazon Web Services service that is sending logs.</p>
    pub fn get_service(&self) -> &::std::option::Option<::std::string::String> {
        &self.service
    }
    /// <p>The type of log that the source is sending. For valid values for this parameter, see the documentation for the source service.</p>
    pub fn log_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of log that the source is sending. For valid values for this parameter, see the documentation for the source service.</p>
    pub fn set_log_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_type = input;
        self
    }
    /// <p>The type of log that the source is sending. For valid values for this parameter, see the documentation for the source service.</p>
    pub fn get_log_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_type
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that have been assigned to this delivery source.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags that have been assigned to this delivery source.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags that have been assigned to this delivery source.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`DeliverySource`](crate::types::DeliverySource).
    pub fn build(self) -> crate::types::DeliverySource {
        crate::types::DeliverySource {
            name: self.name,
            arn: self.arn,
            resource_arns: self.resource_arns,
            service: self.service,
            log_type: self.log_type,
            tags: self.tags,
        }
    }
}
