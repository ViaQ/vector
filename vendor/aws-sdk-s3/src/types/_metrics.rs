// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A container specifying replication metrics-related settings enabling replication metrics and events.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Metrics {
    /// <p> Specifies whether the replication metrics are enabled. </p>
    pub status: crate::types::MetricsStatus,
    /// <p> A container specifying the time threshold for emitting the <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub event_threshold: ::std::option::Option<crate::types::ReplicationTimeValue>,
}
impl Metrics {
    /// <p> Specifies whether the replication metrics are enabled. </p>
    pub fn status(&self) -> &crate::types::MetricsStatus {
        &self.status
    }
    /// <p> A container specifying the time threshold for emitting the <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub fn event_threshold(&self) -> ::std::option::Option<&crate::types::ReplicationTimeValue> {
        self.event_threshold.as_ref()
    }
}
impl Metrics {
    /// Creates a new builder-style object to manufacture [`Metrics`](crate::types::Metrics).
    pub fn builder() -> crate::types::builders::MetricsBuilder {
        crate::types::builders::MetricsBuilder::default()
    }
}

/// A builder for [`Metrics`](crate::types::Metrics).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MetricsBuilder {
    pub(crate) status: ::std::option::Option<crate::types::MetricsStatus>,
    pub(crate) event_threshold: ::std::option::Option<crate::types::ReplicationTimeValue>,
}
impl MetricsBuilder {
    /// <p> Specifies whether the replication metrics are enabled. </p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::MetricsStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p> Specifies whether the replication metrics are enabled. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::MetricsStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p> Specifies whether the replication metrics are enabled. </p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::MetricsStatus> {
        &self.status
    }
    /// <p> A container specifying the time threshold for emitting the <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub fn event_threshold(mut self, input: crate::types::ReplicationTimeValue) -> Self {
        self.event_threshold = ::std::option::Option::Some(input);
        self
    }
    /// <p> A container specifying the time threshold for emitting the <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub fn set_event_threshold(mut self, input: ::std::option::Option<crate::types::ReplicationTimeValue>) -> Self {
        self.event_threshold = input;
        self
    }
    /// <p> A container specifying the time threshold for emitting the <code>s3:Replication:OperationMissedThreshold</code> event. </p>
    pub fn get_event_threshold(&self) -> &::std::option::Option<crate::types::ReplicationTimeValue> {
        &self.event_threshold
    }
    /// Consumes the builder and constructs a [`Metrics`](crate::types::Metrics).
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](crate::types::builders::MetricsBuilder::status)
    pub fn build(self) -> ::std::result::Result<crate::types::Metrics, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Metrics {
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building Metrics",
                )
            })?,
            event_threshold: self.event_threshold,
        })
    }
}
