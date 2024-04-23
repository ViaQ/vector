// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMessageOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteMessageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteMessageOutput {
    /// Creates a new builder-style object to manufacture [`DeleteMessageOutput`](crate::operation::delete_message::DeleteMessageOutput).
    pub fn builder() -> crate::operation::delete_message::builders::DeleteMessageOutputBuilder {
        crate::operation::delete_message::builders::DeleteMessageOutputBuilder::default()
    }
}

/// A builder for [`DeleteMessageOutput`](crate::operation::delete_message::DeleteMessageOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteMessageOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteMessageOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteMessageOutput`](crate::operation::delete_message::DeleteMessageOutput).
    pub fn build(self) -> crate::operation::delete_message::DeleteMessageOutput {
        crate::operation::delete_message::DeleteMessageOutput {
            _request_id: self._request_id,
        }
    }
}
