// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSubscriptionsByTopic`](crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`topic_arn(impl Into<String>)`](crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder::topic_arn) / [`set_topic_arn(Option<String>)`](crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder::set_topic_arn):<br>required: **true**<br><p>The ARN of the topic for which you wish to find subscriptions.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder::set_next_token):<br>required: **false**<br><p>Token returned by the previous <code>ListSubscriptionsByTopic</code> request.</p><br>
    /// - On success, responds with [`ListSubscriptionsByTopicOutput`](crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicOutput) with field(s):
    ///   - [`subscriptions(Option<Vec::<Subscription>>)`](crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicOutput::subscriptions): <p>A list of subscriptions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicOutput::next_token): <p>Token to pass along to the next <code>ListSubscriptionsByTopic</code> request. This element is returned if there are more subscriptions to retrieve.</p>
    /// - On failure, responds with [`SdkError<ListSubscriptionsByTopicError>`](crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicError)
    pub fn list_subscriptions_by_topic(&self) -> crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder {
        crate::operation::list_subscriptions_by_topic::builders::ListSubscriptionsByTopicFluentBuilder::new(self.handle.clone())
    }
}
