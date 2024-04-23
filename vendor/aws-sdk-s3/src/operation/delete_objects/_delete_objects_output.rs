// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteObjectsOutput {
    /// <p>Container element for a successful delete. It identifies the object that was successfully deleted.</p>
    pub deleted: ::std::option::Option<::std::vec::Vec<crate::types::DeletedObject>>,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub request_charged: ::std::option::Option<crate::types::RequestCharged>,
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to delete and the error it encountered.</p>
    pub errors: ::std::option::Option<::std::vec::Vec<crate::types::Error>>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl DeleteObjectsOutput {
    /// <p>Container element for a successful delete. It identifies the object that was successfully deleted.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.deleted.is_none()`.
    pub fn deleted(&self) -> &[crate::types::DeletedObject] {
        self.deleted.as_deref().unwrap_or_default()
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_charged(&self) -> ::std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to delete and the error it encountered.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.errors.is_none()`.
    pub fn errors(&self) -> &[crate::types::Error] {
        self.errors.as_deref().unwrap_or_default()
    }
}
impl crate::s3_request_id::RequestIdExt for DeleteObjectsOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteObjectsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteObjectsOutput {
    /// Creates a new builder-style object to manufacture [`DeleteObjectsOutput`](crate::operation::delete_objects::DeleteObjectsOutput).
    pub fn builder() -> crate::operation::delete_objects::builders::DeleteObjectsOutputBuilder {
        crate::operation::delete_objects::builders::DeleteObjectsOutputBuilder::default()
    }
}

/// A builder for [`DeleteObjectsOutput`](crate::operation::delete_objects::DeleteObjectsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteObjectsOutputBuilder {
    pub(crate) deleted: ::std::option::Option<::std::vec::Vec<crate::types::DeletedObject>>,
    pub(crate) request_charged: ::std::option::Option<crate::types::RequestCharged>,
    pub(crate) errors: ::std::option::Option<::std::vec::Vec<crate::types::Error>>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl DeleteObjectsOutputBuilder {
    /// Appends an item to `deleted`.
    ///
    /// To override the contents of this collection use [`set_deleted`](Self::set_deleted).
    ///
    /// <p>Container element for a successful delete. It identifies the object that was successfully deleted.</p>
    pub fn deleted(mut self, input: crate::types::DeletedObject) -> Self {
        let mut v = self.deleted.unwrap_or_default();
        v.push(input);
        self.deleted = ::std::option::Option::Some(v);
        self
    }
    /// <p>Container element for a successful delete. It identifies the object that was successfully deleted.</p>
    pub fn set_deleted(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DeletedObject>>) -> Self {
        self.deleted = input;
        self
    }
    /// <p>Container element for a successful delete. It identifies the object that was successfully deleted.</p>
    pub fn get_deleted(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DeletedObject>> {
        &self.deleted
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = ::std::option::Option::Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_request_charged(mut self, input: ::std::option::Option<crate::types::RequestCharged>) -> Self {
        self.request_charged = input;
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_request_charged(&self) -> &::std::option::Option<crate::types::RequestCharged> {
        &self.request_charged
    }
    /// Appends an item to `errors`.
    ///
    /// To override the contents of this collection use [`set_errors`](Self::set_errors).
    ///
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to delete and the error it encountered.</p>
    pub fn errors(mut self, input: crate::types::Error) -> Self {
        let mut v = self.errors.unwrap_or_default();
        v.push(input);
        self.errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to delete and the error it encountered.</p>
    pub fn set_errors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Error>>) -> Self {
        self.errors = input;
        self
    }
    /// <p>Container for a failed delete action that describes the object that Amazon S3 attempted to delete and the error it encountered.</p>
    pub fn get_errors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Error>> {
        &self.errors
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteObjectsOutput`](crate::operation::delete_objects::DeleteObjectsOutput).
    pub fn build(self) -> crate::operation::delete_objects::DeleteObjectsOutput {
        crate::operation::delete_objects::DeleteObjectsOutput {
            deleted: self.deleted,
            request_charged: self.request_charged,
            errors: self.errors,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
