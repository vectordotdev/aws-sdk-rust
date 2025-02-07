// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopQuery`](crate::operation::stop_query::builders::StopQueryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`query_id(impl ::std::convert::Into<String>)`](crate::operation::stop_query::builders::StopQueryFluentBuilder::query_id) / [`set_query_id(Option<String>)`](crate::operation::stop_query::builders::StopQueryFluentBuilder::set_query_id): <p>The ID number of the query to stop. To find this ID number, use <code>DescribeQueries</code>.</p>
    /// - On success, responds with [`StopQueryOutput`](crate::operation::stop_query::StopQueryOutput) with field(s):
    ///   - [`success(bool)`](crate::operation::stop_query::StopQueryOutput::success): <p>This is true if the query was stopped by the <code>StopQuery</code> operation.</p>
    /// - On failure, responds with [`SdkError<StopQueryError>`](crate::operation::stop_query::StopQueryError)
    pub fn stop_query(&self) -> crate::operation::stop_query::builders::StopQueryFluentBuilder {
        crate::operation::stop_query::builders::StopQueryFluentBuilder::new(self.handle.clone())
    }
}
