// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSyncJobs`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl ::std::convert::Into<String>)`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::set_workspace_id): <p>The ID of the workspace that contains the sync job.</p>
    ///   - [`max_results(i32)`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::set_max_results): <p>The maximum number of results to return at one time. The default is 50.</p>  <p>Valid Range: Minimum value of 0. Maximum value of 200.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::set_next_token): <p>The string that specifies the next page of results.</p>
    /// - On success, responds with [`ListSyncJobsOutput`](crate::operation::list_sync_jobs::ListSyncJobsOutput) with field(s):
    ///   - [`sync_job_summaries(Option<Vec<SyncJobSummary>>)`](crate::operation::list_sync_jobs::ListSyncJobsOutput::sync_job_summaries): <p>The listed SyncJob summaries.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_sync_jobs::ListSyncJobsOutput::next_token): <p>The string that specifies the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListSyncJobsError>`](crate::operation::list_sync_jobs::ListSyncJobsError)
    pub fn list_sync_jobs(
        &self,
    ) -> crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder {
        crate::operation::list_sync_jobs::builders::ListSyncJobsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
