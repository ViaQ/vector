// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDashboardsOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteDashboardsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteDashboardsOutput {
    /// Creates a new builder-style object to manufacture [`DeleteDashboardsOutput`](crate::operation::delete_dashboards::DeleteDashboardsOutput).
    pub fn builder() -> crate::operation::delete_dashboards::builders::DeleteDashboardsOutputBuilder {
        crate::operation::delete_dashboards::builders::DeleteDashboardsOutputBuilder::default()
    }
}

/// A builder for [`DeleteDashboardsOutput`](crate::operation::delete_dashboards::DeleteDashboardsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteDashboardsOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteDashboardsOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDashboardsOutput`](crate::operation::delete_dashboards::DeleteDashboardsOutput).
    pub fn build(self) -> crate::operation::delete_dashboards::DeleteDashboardsOutput {
        crate::operation::delete_dashboards::DeleteDashboardsOutput {
            _request_id: self._request_id,
        }
    }
}
