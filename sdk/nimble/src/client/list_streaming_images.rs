// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListStreamingImages`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`owner(impl ::std::convert::Into<String>)`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::owner) / [`set_owner(Option<String>)`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::set_owner): <p>Filter this request to streaming images with the given owner</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::set_studio_id): <p>The studio ID. </p>
    /// - On success, responds with [`ListStreamingImagesOutput`](crate::operation::list_streaming_images::ListStreamingImagesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_streaming_images::ListStreamingImagesOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`streaming_images(Option<Vec<StreamingImage>>)`](crate::operation::list_streaming_images::ListStreamingImagesOutput::streaming_images): <p>A collection of streaming images.</p>
    /// - On failure, responds with [`SdkError<ListStreamingImagesError>`](crate::operation::list_streaming_images::ListStreamingImagesError)
    pub fn list_streaming_images(
        &self,
    ) -> crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder {
        crate::operation::list_streaming_images::builders::ListStreamingImagesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
