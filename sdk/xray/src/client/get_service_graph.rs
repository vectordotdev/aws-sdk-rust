// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetServiceGraph`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`start_time(DateTime)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::set_start_time): <p>The start of the time frame for which to generate a graph.</p>
    ///   - [`end_time(DateTime)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::set_end_time): <p>The end of the timeframe for which to generate a graph.</p>
    ///   - [`group_name(impl ::std::convert::Into<String>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::set_group_name): <p>The name of a group based on which you want to generate a graph.</p>
    ///   - [`group_arn(impl ::std::convert::Into<String>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::group_arn) / [`set_group_arn(Option<String>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::set_group_arn): <p>The Amazon Resource Name (ARN) of a group based on which you want to generate a graph.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::set_next_token): <p>Pagination token.</p>
    /// - On success, responds with [`GetServiceGraphOutput`](crate::operation::get_service_graph::GetServiceGraphOutput) with field(s):
    ///   - [`start_time(Option<DateTime>)`](crate::operation::get_service_graph::GetServiceGraphOutput::start_time): <p>The start of the time frame for which the graph was generated.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::get_service_graph::GetServiceGraphOutput::end_time): <p>The end of the time frame for which the graph was generated.</p>
    ///   - [`services(Option<Vec<Service>>)`](crate::operation::get_service_graph::GetServiceGraphOutput::services): <p>The services that have processed a traced request during the specified time frame.</p>
    ///   - [`contains_old_group_versions(bool)`](crate::operation::get_service_graph::GetServiceGraphOutput::contains_old_group_versions): <p>A flag indicating whether the group's filter expression has been consistent, or if the returned service graph may show traces from an older version of the group's filter expression.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_service_graph::GetServiceGraphOutput::next_token): <p>Pagination token.</p>
    /// - On failure, responds with [`SdkError<GetServiceGraphError>`](crate::operation::get_service_graph::GetServiceGraphError)
    pub fn get_service_graph(
        &self,
    ) -> crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder {
        crate::operation::get_service_graph::builders::GetServiceGraphFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
