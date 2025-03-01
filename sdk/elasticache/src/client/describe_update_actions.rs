// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeUpdateActions`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_update_name(impl ::std::convert::Into<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::service_update_name) / [`set_service_update_name(Option<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_service_update_name): <p>The unique ID of the service update</p>
    ///   - [`replication_group_ids(Vec<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::replication_group_ids) / [`set_replication_group_ids(Option<Vec<String>>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_replication_group_ids): <p>The replication group IDs</p>
    ///   - [`cache_cluster_ids(Vec<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::cache_cluster_ids) / [`set_cache_cluster_ids(Option<Vec<String>>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_cache_cluster_ids): <p>The cache cluster IDs</p>
    ///   - [`engine(impl ::std::convert::Into<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::engine) / [`set_engine(Option<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_engine): <p>The Elasticache engine to which the update applies. Either Redis or Memcached </p>
    ///   - [`service_update_status(Vec<ServiceUpdateStatus>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::service_update_status) / [`set_service_update_status(Option<Vec<ServiceUpdateStatus>>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_service_update_status): <p>The status of the service update</p>
    ///   - [`service_update_time_range(TimeRangeFilter)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::service_update_time_range) / [`set_service_update_time_range(Option<TimeRangeFilter>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_service_update_time_range): <p>The range of time specified to search for service updates that are in available status</p>
    ///   - [`update_action_status(Vec<UpdateActionStatus>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::update_action_status) / [`set_update_action_status(Option<Vec<UpdateActionStatus>>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_update_action_status): <p>The status of the update action.</p>
    ///   - [`show_node_level_update_status(bool)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::show_node_level_update_status) / [`set_show_node_level_update_status(Option<bool>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_show_node_level_update_status): <p>Dictates whether to include node level update status in the response </p>
    ///   - [`max_records(i32)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::max_records) / [`set_max_records(Option<i32>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_max_records): <p>The maximum number of records to include in the response</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::set_marker): <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    /// - On success, responds with [`DescribeUpdateActionsOutput`](crate::operation::describe_update_actions::DescribeUpdateActionsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::operation::describe_update_actions::DescribeUpdateActionsOutput::marker): <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    ///   - [`update_actions(Option<Vec<UpdateAction>>)`](crate::operation::describe_update_actions::DescribeUpdateActionsOutput::update_actions): <p>Returns a list of update actions</p>
    /// - On failure, responds with [`SdkError<DescribeUpdateActionsError>`](crate::operation::describe_update_actions::DescribeUpdateActionsError)
    pub fn describe_update_actions(
        &self,
    ) -> crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder
    {
        crate::operation::describe_update_actions::builders::DescribeUpdateActionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
