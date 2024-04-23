// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTags`](crate::operation::list_tags::builders::ListTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::set_arn):<br>required: **true**<br><p> Specify the <code>ARN</code> for the Elasticsearch domain to which the tags are attached that you want to view.</p><br>
    /// - On success, responds with [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput) with field(s):
    ///   - [`tag_list(Option<Vec::<Tag>>)`](crate::operation::list_tags::ListTagsOutput::tag_list): <p> List of <code>Tag</code> for the requested Elasticsearch domain.</p>
    /// - On failure, responds with [`SdkError<ListTagsError>`](crate::operation::list_tags::ListTagsError)
    pub fn list_tags(&self) -> crate::operation::list_tags::builders::ListTagsFluentBuilder {
        crate::operation::list_tags::builders::ListTagsFluentBuilder::new(self.handle.clone())
    }
}
