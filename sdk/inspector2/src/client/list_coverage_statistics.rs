// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCoverageStatistics`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filter_criteria(CoverageFilterCriteria)`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::filter_criteria) / [`set_filter_criteria(Option<CoverageFilterCriteria>)`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::set_filter_criteria): <p>An object that contains details on the filters to apply to the coverage data for your environment.</p>
    ///   - [`group_by(GroupKey)`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::group_by) / [`set_group_by(Option<GroupKey>)`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::set_group_by): <p>The value to group the results by.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::set_next_token): <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    /// - On success, responds with [`ListCoverageStatisticsOutput`](crate::operation::list_coverage_statistics::ListCoverageStatisticsOutput) with field(s):
    ///   - [`counts_by_group(Option<Vec<Counts>>)`](crate::operation::list_coverage_statistics::ListCoverageStatisticsOutput::counts_by_group): <p>An array with the number for each group.</p>
    ///   - [`total_counts(Option<i64>)`](crate::operation::list_coverage_statistics::ListCoverageStatisticsOutput::total_counts): <p>The total number for all groups.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_coverage_statistics::ListCoverageStatisticsOutput::next_token): <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    /// - On failure, responds with [`SdkError<ListCoverageStatisticsError>`](crate::operation::list_coverage_statistics::ListCoverageStatisticsError)
    pub fn list_coverage_statistics(
        &self,
    ) -> crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder
    {
        crate::operation::list_coverage_statistics::builders::ListCoverageStatisticsFluentBuilder::new(self.handle.clone())
    }
}
