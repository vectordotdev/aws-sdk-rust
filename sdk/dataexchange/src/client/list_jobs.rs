// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListJobs`](crate::operation::list_jobs::builders::ListJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`data_set_id(impl ::std::convert::Into<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::data_set_id) / [`set_data_set_id(Option<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_data_set_id): <p>The unique identifier for a data set.</p>
    ///   - [`max_results(i32)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_max_results): <p>The maximum number of results returned by a single call.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_next_token): <p>The token value retrieved from a previous call to access the next page of results.</p>
    ///   - [`revision_id(impl ::std::convert::Into<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_revision_id): <p>The unique identifier for a revision.</p>
    /// - On success, responds with [`ListJobsOutput`](crate::operation::list_jobs::ListJobsOutput) with field(s):
    ///   - [`jobs(Option<Vec<JobEntry>>)`](crate::operation::list_jobs::ListJobsOutput::jobs): <p>The jobs listed by the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_jobs::ListJobsOutput::next_token): <p>The token value retrieved from a previous call to access the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListJobsError>`](crate::operation::list_jobs::ListJobsError)
    pub fn list_jobs(&self) -> crate::operation::list_jobs::builders::ListJobsFluentBuilder {
        crate::operation::list_jobs::builders::ListJobsFluentBuilder::new(self.handle.clone())
    }
}
