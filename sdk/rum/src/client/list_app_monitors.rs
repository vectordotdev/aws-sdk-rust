// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAppMonitors`](crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder::set_max_results): <p>The maximum number of results to return in one operation. The default is 50. The maximum that you can specify is 100.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder::set_next_token): <p>Use the token returned by the previous operation to request the next page of results.</p>
    /// - On success, responds with [`ListAppMonitorsOutput`](crate::operation::list_app_monitors::ListAppMonitorsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_app_monitors::ListAppMonitorsOutput::next_token): <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
    ///   - [`app_monitor_summaries(Option<Vec<AppMonitorSummary>>)`](crate::operation::list_app_monitors::ListAppMonitorsOutput::app_monitor_summaries): <p>An array of structures that contain information about the returned app monitors.</p>
    /// - On failure, responds with [`SdkError<ListAppMonitorsError>`](crate::operation::list_app_monitors::ListAppMonitorsError)
    pub fn list_app_monitors(
        &self,
    ) -> crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder {
        crate::operation::list_app_monitors::builders::ListAppMonitorsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
