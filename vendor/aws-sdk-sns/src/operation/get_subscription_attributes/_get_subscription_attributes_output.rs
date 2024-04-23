// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Response for GetSubscriptionAttributes action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSubscriptionAttributesOutput {
    /// <p>A map of the subscription's attributes. Attributes in this map include the following:</p>
    /// <ul>
    /// <li> <p> <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the subscription confirmation request was authenticated.</p> </li>
    /// <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the subscription's delivery policy.</p> </li>
    /// <li> <p> <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults.</p> </li>
    /// <li> <p> <code>FilterPolicy</code> – The filter policy JSON that is assigned to the subscription. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html">Amazon SNS Message Filtering</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// <li> <p> <code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li> <p> <code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p> </li>
    /// <li> <p> <code>MessageBody</code> – The filter is applied on the message body.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>Owner</code> – The Amazon Web Services account ID of the subscription's owner.</p> </li>
    /// <li> <p> <code>PendingConfirmation</code> – <code>true</code> if the subscription hasn't been confirmed. To confirm a pending subscription, call the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li>
    /// <li> <p> <code>RawMessageDelivery</code> – <code>true</code> if raw message delivery is enabled for the subscription. Raw messages are free of JSON formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p> </li>
    /// <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li>
    /// <li> <p> <code>SubscriptionArn</code> – The subscription's ARN.</p> </li>
    /// <li> <p> <code>TopicArn</code> – The topic ARN that the subscription is associated with.</p> </li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Kinesis Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li> <p> <code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li> <p>Permission to write to the Kinesis Data Firehose delivery stream</p> </li>
    /// <li> <p>Amazon SNS listed as a trusted entity</p> </li>
    /// </ul> <p>Specifying a valid ARN for this attribute is required for Kinesis Data Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Kinesis Data Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// </ul>
    pub attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl GetSubscriptionAttributesOutput {
    /// <p>A map of the subscription's attributes. Attributes in this map include the following:</p>
    /// <ul>
    /// <li> <p> <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the subscription confirmation request was authenticated.</p> </li>
    /// <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the subscription's delivery policy.</p> </li>
    /// <li> <p> <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults.</p> </li>
    /// <li> <p> <code>FilterPolicy</code> – The filter policy JSON that is assigned to the subscription. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html">Amazon SNS Message Filtering</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// <li> <p> <code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li> <p> <code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p> </li>
    /// <li> <p> <code>MessageBody</code> – The filter is applied on the message body.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>Owner</code> – The Amazon Web Services account ID of the subscription's owner.</p> </li>
    /// <li> <p> <code>PendingConfirmation</code> – <code>true</code> if the subscription hasn't been confirmed. To confirm a pending subscription, call the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li>
    /// <li> <p> <code>RawMessageDelivery</code> – <code>true</code> if raw message delivery is enabled for the subscription. Raw messages are free of JSON formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p> </li>
    /// <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li>
    /// <li> <p> <code>SubscriptionArn</code> – The subscription's ARN.</p> </li>
    /// <li> <p> <code>TopicArn</code> – The topic ARN that the subscription is associated with.</p> </li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Kinesis Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li> <p> <code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li> <p>Permission to write to the Kinesis Data Firehose delivery stream</p> </li>
    /// <li> <p>Amazon SNS listed as a trusted entity</p> </li>
    /// </ul> <p>Specifying a valid ARN for this attribute is required for Kinesis Data Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Kinesis Data Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// </ul>
    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.attributes.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetSubscriptionAttributesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSubscriptionAttributesOutput {
    /// Creates a new builder-style object to manufacture [`GetSubscriptionAttributesOutput`](crate::operation::get_subscription_attributes::GetSubscriptionAttributesOutput).
    pub fn builder() -> crate::operation::get_subscription_attributes::builders::GetSubscriptionAttributesOutputBuilder {
        crate::operation::get_subscription_attributes::builders::GetSubscriptionAttributesOutputBuilder::default()
    }
}

/// A builder for [`GetSubscriptionAttributesOutput`](crate::operation::get_subscription_attributes::GetSubscriptionAttributesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetSubscriptionAttributesOutputBuilder {
    pub(crate) attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl GetSubscriptionAttributesOutputBuilder {
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>A map of the subscription's attributes. Attributes in this map include the following:</p>
    /// <ul>
    /// <li> <p> <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the subscription confirmation request was authenticated.</p> </li>
    /// <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the subscription's delivery policy.</p> </li>
    /// <li> <p> <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults.</p> </li>
    /// <li> <p> <code>FilterPolicy</code> – The filter policy JSON that is assigned to the subscription. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html">Amazon SNS Message Filtering</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// <li> <p> <code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li> <p> <code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p> </li>
    /// <li> <p> <code>MessageBody</code> – The filter is applied on the message body.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>Owner</code> – The Amazon Web Services account ID of the subscription's owner.</p> </li>
    /// <li> <p> <code>PendingConfirmation</code> – <code>true</code> if the subscription hasn't been confirmed. To confirm a pending subscription, call the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li>
    /// <li> <p> <code>RawMessageDelivery</code> – <code>true</code> if raw message delivery is enabled for the subscription. Raw messages are free of JSON formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p> </li>
    /// <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li>
    /// <li> <p> <code>SubscriptionArn</code> – The subscription's ARN.</p> </li>
    /// <li> <p> <code>TopicArn</code> – The topic ARN that the subscription is associated with.</p> </li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Kinesis Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li> <p> <code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li> <p>Permission to write to the Kinesis Data Firehose delivery stream</p> </li>
    /// <li> <p>Amazon SNS listed as a trusted entity</p> </li>
    /// </ul> <p>Specifying a valid ARN for this attribute is required for Kinesis Data Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Kinesis Data Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// </ul>
    pub fn attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of the subscription's attributes. Attributes in this map include the following:</p>
    /// <ul>
    /// <li> <p> <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the subscription confirmation request was authenticated.</p> </li>
    /// <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the subscription's delivery policy.</p> </li>
    /// <li> <p> <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults.</p> </li>
    /// <li> <p> <code>FilterPolicy</code> – The filter policy JSON that is assigned to the subscription. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html">Amazon SNS Message Filtering</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// <li> <p> <code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li> <p> <code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p> </li>
    /// <li> <p> <code>MessageBody</code> – The filter is applied on the message body.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>Owner</code> – The Amazon Web Services account ID of the subscription's owner.</p> </li>
    /// <li> <p> <code>PendingConfirmation</code> – <code>true</code> if the subscription hasn't been confirmed. To confirm a pending subscription, call the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li>
    /// <li> <p> <code>RawMessageDelivery</code> – <code>true</code> if raw message delivery is enabled for the subscription. Raw messages are free of JSON formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p> </li>
    /// <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li>
    /// <li> <p> <code>SubscriptionArn</code> – The subscription's ARN.</p> </li>
    /// <li> <p> <code>TopicArn</code> – The topic ARN that the subscription is associated with.</p> </li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Kinesis Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li> <p> <code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li> <p>Permission to write to the Kinesis Data Firehose delivery stream</p> </li>
    /// <li> <p>Amazon SNS listed as a trusted entity</p> </li>
    /// </ul> <p>Specifying a valid ARN for this attribute is required for Kinesis Data Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Kinesis Data Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// </ul>
    pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.attributes = input;
        self
    }
    /// <p>A map of the subscription's attributes. Attributes in this map include the following:</p>
    /// <ul>
    /// <li> <p> <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the subscription confirmation request was authenticated.</p> </li>
    /// <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the subscription's delivery policy.</p> </li>
    /// <li> <p> <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults.</p> </li>
    /// <li> <p> <code>FilterPolicy</code> – The filter policy JSON that is assigned to the subscription. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html">Amazon SNS Message Filtering</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// <li> <p> <code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li> <p> <code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p> </li>
    /// <li> <p> <code>MessageBody</code> – The filter is applied on the message body.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>Owner</code> – The Amazon Web Services account ID of the subscription's owner.</p> </li>
    /// <li> <p> <code>PendingConfirmation</code> – <code>true</code> if the subscription hasn't been confirmed. To confirm a pending subscription, call the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li>
    /// <li> <p> <code>RawMessageDelivery</code> – <code>true</code> if raw message delivery is enabled for the subscription. Raw messages are free of JSON formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p> </li>
    /// <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li>
    /// <li> <p> <code>SubscriptionArn</code> – The subscription's ARN.</p> </li>
    /// <li> <p> <code>TopicArn</code> – The topic ARN that the subscription is associated with.</p> </li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Kinesis Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li> <p> <code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li> <p>Permission to write to the Kinesis Data Firehose delivery stream</p> </li>
    /// <li> <p>Amazon SNS listed as a trusted entity</p> </li>
    /// </ul> <p>Specifying a valid ARN for this attribute is required for Kinesis Data Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Kinesis Data Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p> </li>
    /// </ul>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.attributes
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetSubscriptionAttributesOutput`](crate::operation::get_subscription_attributes::GetSubscriptionAttributesOutput).
    pub fn build(self) -> crate::operation::get_subscription_attributes::GetSubscriptionAttributesOutput {
        crate::operation::get_subscription_attributes::GetSubscriptionAttributesOutput {
            attributes: self.attributes,
            _request_id: self._request_id,
        }
    }
}
