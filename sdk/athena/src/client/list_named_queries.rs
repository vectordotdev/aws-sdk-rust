// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListNamedQueries`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::set_next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    ///   - [`max_results(i32)`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::set_max_results): <p>The maximum number of queries to return in this request.</p>
    ///   - [`work_group(impl ::std::convert::Into<String>)`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::work_group) / [`set_work_group(Option<String>)`](crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::set_work_group): <p>The name of the workgroup from which the named queries are being returned. If a workgroup is not specified, the saved queries for the primary workgroup are returned.</p>
    /// - On success, responds with [`ListNamedQueriesOutput`](crate::operation::list_named_queries::ListNamedQueriesOutput) with field(s):
    ///   - [`named_query_ids(Option<Vec<String>>)`](crate::operation::list_named_queries::ListNamedQueriesOutput::named_query_ids): <p>The list of unique query IDs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_named_queries::ListNamedQueriesOutput::next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    /// - On failure, responds with [`SdkError<ListNamedQueriesError>`](crate::operation::list_named_queries::ListNamedQueriesError)
    pub fn list_named_queries(
        &self,
    ) -> crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder {
        crate::operation::list_named_queries::builders::ListNamedQueriesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
