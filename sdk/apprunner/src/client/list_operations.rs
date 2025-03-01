// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListOperations`](crate::operation::list_operations::builders::ListOperationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_arn(impl ::std::convert::Into<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::service_arn) / [`set_service_arn(Option<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_service_arn): <p>The Amazon Resource Name (ARN) of the App Runner service that you want a list of operations for.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_next_token): <p>A token from a previous result page. It's used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones specified in the initial request.</p>  <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
    ///   - [`max_results(i32)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_operations::builders::ListOperationsFluentBuilder::set_max_results): <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>  <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
    /// - On success, responds with [`ListOperationsOutput`](crate::operation::list_operations::ListOperationsOutput) with field(s):
    ///   - [`operation_summary_list(Option<Vec<OperationSummary>>)`](crate::operation::list_operations::ListOperationsOutput::operation_summary_list): <p>A list of operation summary information records. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_operations::ListOperationsOutput::next_token): <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
    /// - On failure, responds with [`SdkError<ListOperationsError>`](crate::operation::list_operations::ListOperationsError)
    pub fn list_operations(
        &self,
    ) -> crate::operation::list_operations::builders::ListOperationsFluentBuilder {
        crate::operation::list_operations::builders::ListOperationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
