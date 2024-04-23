// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetQueryResults`](crate::operation::get_query_results::builders::GetQueryResultsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`query_id(impl Into<String>)`](crate::operation::get_query_results::builders::GetQueryResultsFluentBuilder::query_id) / [`set_query_id(Option<String>)`](crate::operation::get_query_results::builders::GetQueryResultsFluentBuilder::set_query_id):<br>required: **true**<br><p>The ID number of the query.</p><br>
    /// - On success, responds with [`GetQueryResultsOutput`](crate::operation::get_query_results::GetQueryResultsOutput) with field(s):
    ///   - [`results(Option<Vec::<Vec::<ResultField>>>)`](crate::operation::get_query_results::GetQueryResultsOutput::results): <p>The log events that matched the query criteria during the most recent time it ran.</p>  <p>The <code>results</code> value is an array of arrays. Each log event is one object in the top-level array. Each of these log event objects is an array of <code>field</code>/<code>value</code> pairs.</p>
    ///   - [`statistics(Option<QueryStatistics>)`](crate::operation::get_query_results::GetQueryResultsOutput::statistics): <p>Includes the number of log events scanned by the query, the number of log events that matched the query criteria, and the total number of bytes in the scanned log events. These values reflect the full raw results of the query.</p>
    ///   - [`status(Option<QueryStatus>)`](crate::operation::get_query_results::GetQueryResultsOutput::status): <p>The status of the most recent running of the query. Possible values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, <code>Scheduled</code>, <code>Timeout</code>, and <code>Unknown</code>.</p>  <p>Queries time out after 60 minutes of runtime. To avoid having your queries time out, reduce the time range being searched or partition your query into a number of queries.</p>
    ///   - [`encryption_key(Option<String>)`](crate::operation::get_query_results::GetQueryResultsOutput::encryption_key): <p>If you associated an KMS key with the CloudWatch Logs Insights query results in this account, this field displays the ARN of the key that's used to encrypt the query results when <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_StartQuery.html">StartQuery</a> stores them.</p>
    /// - On failure, responds with [`SdkError<GetQueryResultsError>`](crate::operation::get_query_results::GetQueryResultsError)
    pub fn get_query_results(&self) -> crate::operation::get_query_results::builders::GetQueryResultsFluentBuilder {
        crate::operation::get_query_results::builders::GetQueryResultsFluentBuilder::new(self.handle.clone())
    }
}
