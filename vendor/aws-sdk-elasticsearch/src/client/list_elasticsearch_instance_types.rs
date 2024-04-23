// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListElasticsearchInstanceTypes`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`elasticsearch_version(impl Into<String>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::elasticsearch_version) / [`set_elasticsearch_version(Option<String>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::set_elasticsearch_version):<br>required: **true**<br><p>Version of Elasticsearch for which list of supported elasticsearch instance types are needed. </p><br>
    ///   - [`domain_name(impl Into<String>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::set_domain_name):<br>required: **false**<br><p>DomainName represents the name of the Domain that we are trying to modify. This should be present only if we are querying for list of available Elasticsearch instance types when modifying existing domain. </p><br>
    ///   - [`max_results(i32)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::set_max_results):<br>required: **false**<br><p> Set this value to limit the number of results returned. Value provided must be greater than 30 else it wont be honored. </p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::set_next_token):<br>required: **false**<br><p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination. </p><br>
    /// - On success, responds with [`ListElasticsearchInstanceTypesOutput`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput) with field(s):
    ///   - [`elasticsearch_instance_types(Option<Vec::<EsPartitionInstanceType>>)`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput::elasticsearch_instance_types): <p> List of instance types supported by Amazon Elasticsearch service for given <code> <code>ElasticsearchVersion</code> </code> </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput::next_token): <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    /// - On failure, responds with [`SdkError<ListElasticsearchInstanceTypesError>`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesError)
    pub fn list_elasticsearch_instance_types(
        &self,
    ) -> crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder {
        crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesFluentBuilder::new(self.handle.clone())
    }
}
