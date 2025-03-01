// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWorkerFleets`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`site(impl ::std::convert::Into<String>)`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::site) / [`set_site(Option<String>)`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::set_site): Site ARN.
    ///   - [`max_results(i32)`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::set_max_results): Maximum number of results to retrieve in a single ListWorkerFleets call.
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::set_next_token): Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    /// - On success, responds with [`ListWorkerFleetsOutput`](crate::operation::list_worker_fleets::ListWorkerFleetsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_worker_fleets::ListWorkerFleetsOutput::next_token): Pagination token returned when another page of data exists. Provide it in your next call to the API to receive the next page.
    ///   - [`worker_fleets(Option<Vec<WorkerFleet>>)`](crate::operation::list_worker_fleets::ListWorkerFleetsOutput::worker_fleets): List of worker fleets.
    /// - On failure, responds with [`SdkError<ListWorkerFleetsError>`](crate::operation::list_worker_fleets::ListWorkerFleetsError)
    pub fn list_worker_fleets(
        &self,
    ) -> crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder {
        crate::operation::list_worker_fleets::builders::ListWorkerFleetsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
