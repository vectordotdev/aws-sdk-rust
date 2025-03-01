// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteMetricFilter`](crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`log_group_name(impl ::std::convert::Into<String>)`](crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder::log_group_name) / [`set_log_group_name(Option<String>)`](crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder::set_log_group_name): <p>The name of the log group.</p>
    ///   - [`filter_name(impl ::std::convert::Into<String>)`](crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder::filter_name) / [`set_filter_name(Option<String>)`](crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder::set_filter_name): <p>The name of the metric filter.</p>
    /// - On success, responds with [`DeleteMetricFilterOutput`](crate::operation::delete_metric_filter::DeleteMetricFilterOutput)
    /// - On failure, responds with [`SdkError<DeleteMetricFilterError>`](crate::operation::delete_metric_filter::DeleteMetricFilterError)
    pub fn delete_metric_filter(
        &self,
    ) -> crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder {
        crate::operation::delete_metric_filter::builders::DeleteMetricFilterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
