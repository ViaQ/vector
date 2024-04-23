// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListStreamConsumersOutput {
    /// <p>An array of JSON objects. Each object represents one registered consumer.</p>
    pub consumers: ::std::option::Option<::std::vec::Vec<crate::types::Consumer>>,
    /// <p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of registered consumers, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers. For more information about the use of this pagination token when calling the <code>ListStreamConsumers</code> operation, see <code>ListStreamConsumersInput$NextToken</code>.</p> <important>
    /// <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p>
    /// </important>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListStreamConsumersOutput {
    /// <p>An array of JSON objects. Each object represents one registered consumer.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.consumers.is_none()`.
    pub fn consumers(&self) -> &[crate::types::Consumer] {
        self.consumers.as_deref().unwrap_or_default()
    }
    /// <p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of registered consumers, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers. For more information about the use of this pagination token when calling the <code>ListStreamConsumers</code> operation, see <code>ListStreamConsumersInput$NextToken</code>.</p> <important>
    /// <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p>
    /// </important>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListStreamConsumersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListStreamConsumersOutput {
    /// Creates a new builder-style object to manufacture [`ListStreamConsumersOutput`](crate::operation::list_stream_consumers::ListStreamConsumersOutput).
    pub fn builder() -> crate::operation::list_stream_consumers::builders::ListStreamConsumersOutputBuilder {
        crate::operation::list_stream_consumers::builders::ListStreamConsumersOutputBuilder::default()
    }
}

/// A builder for [`ListStreamConsumersOutput`](crate::operation::list_stream_consumers::ListStreamConsumersOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListStreamConsumersOutputBuilder {
    pub(crate) consumers: ::std::option::Option<::std::vec::Vec<crate::types::Consumer>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListStreamConsumersOutputBuilder {
    /// Appends an item to `consumers`.
    ///
    /// To override the contents of this collection use [`set_consumers`](Self::set_consumers).
    ///
    /// <p>An array of JSON objects. Each object represents one registered consumer.</p>
    pub fn consumers(mut self, input: crate::types::Consumer) -> Self {
        let mut v = self.consumers.unwrap_or_default();
        v.push(input);
        self.consumers = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of JSON objects. Each object represents one registered consumer.</p>
    pub fn set_consumers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Consumer>>) -> Self {
        self.consumers = input;
        self
    }
    /// <p>An array of JSON objects. Each object represents one registered consumer.</p>
    pub fn get_consumers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Consumer>> {
        &self.consumers
    }
    /// <p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of registered consumers, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers. For more information about the use of this pagination token when calling the <code>ListStreamConsumers</code> operation, see <code>ListStreamConsumersInput$NextToken</code>.</p> <important>
    /// <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p>
    /// </important>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of registered consumers, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers. For more information about the use of this pagination token when calling the <code>ListStreamConsumers</code> operation, see <code>ListStreamConsumersInput$NextToken</code>.</p> <important>
    /// <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p>
    /// </important>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of registered consumers, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers. For more information about the use of this pagination token when calling the <code>ListStreamConsumers</code> operation, see <code>ListStreamConsumersInput$NextToken</code>.</p> <important>
    /// <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p>
    /// </important>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListStreamConsumersOutput`](crate::operation::list_stream_consumers::ListStreamConsumersOutput).
    pub fn build(self) -> crate::operation::list_stream_consumers::ListStreamConsumersOutput {
        crate::operation::list_stream_consumers::ListStreamConsumersOutput {
            consumers: self.consumers,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
