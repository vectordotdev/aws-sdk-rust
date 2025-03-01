// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPublishedSchemaArns`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_arn(impl ::std::convert::Into<String>)`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::schema_arn) / [`set_schema_arn(Option<String>)`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::set_schema_arn): <p>The response for <code>ListPublishedSchemaArns</code> when this parameter is used will list all minor version ARNs for a major version.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::set_next_token): <p>The pagination token.</p>
    ///   - [`max_results(i32)`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::set_max_results): <p>The maximum number of results to retrieve.</p>
    /// - On success, responds with [`ListPublishedSchemaArnsOutput`](crate::operation::list_published_schema_arns::ListPublishedSchemaArnsOutput) with field(s):
    ///   - [`schema_arns(Option<Vec<String>>)`](crate::operation::list_published_schema_arns::ListPublishedSchemaArnsOutput::schema_arns): <p>The ARNs of published schemas.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_published_schema_arns::ListPublishedSchemaArnsOutput::next_token): <p>The pagination token.</p>
    /// - On failure, responds with [`SdkError<ListPublishedSchemaArnsError>`](crate::operation::list_published_schema_arns::ListPublishedSchemaArnsError)
    pub fn list_published_schema_arns(
        &self,
    ) -> crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder
    {
        crate::operation::list_published_schema_arns::builders::ListPublishedSchemaArnsFluentBuilder::new(self.handle.clone())
    }
}
