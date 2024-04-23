// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDomainNames`](crate::operation::list_domain_names::builders::ListDomainNamesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`engine_type(EngineType)`](crate::operation::list_domain_names::builders::ListDomainNamesFluentBuilder::engine_type) / [`set_engine_type(Option<EngineType>)`](crate::operation::list_domain_names::builders::ListDomainNamesFluentBuilder::set_engine_type):<br>required: **false**<br><p> Optional parameter to filter the output by domain engine type. Acceptable values are 'Elasticsearch' and 'OpenSearch'. </p><br>
    /// - On success, responds with [`ListDomainNamesOutput`](crate::operation::list_domain_names::ListDomainNamesOutput) with field(s):
    ///   - [`domain_names(Option<Vec::<DomainInfo>>)`](crate::operation::list_domain_names::ListDomainNamesOutput::domain_names): <p>List of domain names and respective engine types.</p>
    /// - On failure, responds with [`SdkError<ListDomainNamesError>`](crate::operation::list_domain_names::ListDomainNamesError)
    pub fn list_domain_names(&self) -> crate::operation::list_domain_names::builders::ListDomainNamesFluentBuilder {
        crate::operation::list_domain_names::builders::ListDomainNamesFluentBuilder::new(self.handle.clone())
    }
}
