// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRunGroups`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::set_name): <p>The run groups' name.</p>
    ///   - [`starting_token(impl ::std::convert::Into<String>)`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::starting_token) / [`set_starting_token(Option<String>)`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::set_starting_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::set_max_results): <p>The maximum number of run groups to return in one page of results.</p>
    /// - On success, responds with [`ListRunGroupsOutput`](crate::operation::list_run_groups::ListRunGroupsOutput) with field(s):
    ///   - [`items(Option<Vec<RunGroupListItem>>)`](crate::operation::list_run_groups::ListRunGroupsOutput::items): <p>A list of groups.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_run_groups::ListRunGroupsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListRunGroupsError>`](crate::operation::list_run_groups::ListRunGroupsError)
    pub fn list_run_groups(
        &self,
    ) -> crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder {
        crate::operation::list_run_groups::builders::ListRunGroupsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
