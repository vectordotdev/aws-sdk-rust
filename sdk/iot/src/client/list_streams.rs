// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListStreams`](crate::operation::list_streams::builders::ListStreamsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::set_max_results): <p>The maximum number of results to return at a time.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::set_next_token): <p>A token used to get the next set of results.</p>
    ///   - [`ascending_order(bool)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::ascending_order) / [`set_ascending_order(Option<bool>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::set_ascending_order): <p>Set to true to return the list of streams in ascending order.</p>
    /// - On success, responds with [`ListStreamsOutput`](crate::operation::list_streams::ListStreamsOutput) with field(s):
    ///   - [`streams(Option<Vec<StreamSummary>>)`](crate::operation::list_streams::ListStreamsOutput::streams): <p>A list of streams.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_streams::ListStreamsOutput::next_token): <p>A token used to get the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListStreamsError>`](crate::operation::list_streams::ListStreamsError)
    pub fn list_streams(
        &self,
    ) -> crate::operation::list_streams::builders::ListStreamsFluentBuilder {
        crate::operation::list_streams::builders::ListStreamsFluentBuilder::new(self.handle.clone())
    }
}
