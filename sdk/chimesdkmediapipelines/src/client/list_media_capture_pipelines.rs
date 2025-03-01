// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMediaCapturePipelines`](crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder::set_next_token): <p>The token used to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call. Valid Range: 1 - 99.</p>
    /// - On success, responds with [`ListMediaCapturePipelinesOutput`](crate::operation::list_media_capture_pipelines::ListMediaCapturePipelinesOutput) with field(s):
    ///   - [`media_capture_pipelines(Option<Vec<MediaCapturePipelineSummary>>)`](crate::operation::list_media_capture_pipelines::ListMediaCapturePipelinesOutput::media_capture_pipelines): <p>The media pipeline objects in the list.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_media_capture_pipelines::ListMediaCapturePipelinesOutput::next_token): <p>The token used to retrieve the next page of results. </p>
    /// - On failure, responds with [`SdkError<ListMediaCapturePipelinesError>`](crate::operation::list_media_capture_pipelines::ListMediaCapturePipelinesError)
    pub fn list_media_capture_pipelines(&self) -> crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder{
        crate::operation::list_media_capture_pipelines::builders::ListMediaCapturePipelinesFluentBuilder::new(self.handle.clone())
    }
}
