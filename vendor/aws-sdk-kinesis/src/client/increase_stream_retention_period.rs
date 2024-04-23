// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`IncreaseStreamRetentionPeriod`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::set_stream_name):<br>required: **false**<br><p>The name of the stream to modify.</p><br>
    ///   - [`retention_period_hours(i32)`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::retention_period_hours) / [`set_retention_period_hours(Option<i32>)`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::set_retention_period_hours):<br>required: **true**<br><p>The new retention period of the stream, in hours. Must be more than the current retention period.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::set_stream_arn):<br>required: **false**<br><p>The ARN of the stream.</p><br>
    /// - On success, responds with [`IncreaseStreamRetentionPeriodOutput`](crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodOutput)
    /// - On failure, responds with [`SdkError<IncreaseStreamRetentionPeriodError>`](crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodError)
    pub fn increase_stream_retention_period(
        &self,
    ) -> crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder {
        crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodFluentBuilder::new(self.handle.clone())
    }
}
