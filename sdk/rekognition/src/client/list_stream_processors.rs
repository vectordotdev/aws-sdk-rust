// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListStreamProcessors`](crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder::set_next_token): <p>If the previous response was incomplete (because there are more stream processors to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of stream processors. </p>
    ///   - [`max_results(i32)`](crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder::set_max_results): <p>Maximum number of stream processors you want Amazon Rekognition Video to return in the response. The default is 1000. </p>
    /// - On success, responds with [`ListStreamProcessorsOutput`](crate::operation::list_stream_processors::ListStreamProcessorsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_stream_processors::ListStreamProcessorsOutput::next_token): <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of stream processors. </p>
    ///   - [`stream_processors(Option<Vec<StreamProcessor>>)`](crate::operation::list_stream_processors::ListStreamProcessorsOutput::stream_processors): <p>List of stream processors that you have created.</p>
    /// - On failure, responds with [`SdkError<ListStreamProcessorsError>`](crate::operation::list_stream_processors::ListStreamProcessorsError)
    pub fn list_stream_processors(
        &self,
    ) -> crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder {
        crate::operation::list_stream_processors::builders::ListStreamProcessorsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
