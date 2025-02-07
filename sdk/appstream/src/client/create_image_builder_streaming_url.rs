// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateImageBuilderStreamingURL`](crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder::set_name): <p>The name of the image builder.</p>
    ///   - [`validity(i64)`](crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder::validity) / [`set_validity(Option<i64>)`](crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder::set_validity): <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 3600 seconds.</p>
    /// - On success, responds with [`CreateImageBuilderStreamingUrlOutput`](crate::operation::create_image_builder_streaming_url::CreateImageBuilderStreamingUrlOutput) with field(s):
    ///   - [`streaming_url(Option<String>)`](crate::operation::create_image_builder_streaming_url::CreateImageBuilderStreamingUrlOutput::streaming_url): <p>The URL to start the AppStream 2.0 streaming session.</p>
    ///   - [`expires(Option<DateTime>)`](crate::operation::create_image_builder_streaming_url::CreateImageBuilderStreamingUrlOutput::expires): <p>The elapsed time, in seconds after the Unix epoch, when this URL expires.</p>
    /// - On failure, responds with [`SdkError<CreateImageBuilderStreamingURLError>`](crate::operation::create_image_builder_streaming_url::CreateImageBuilderStreamingURLError)
    pub fn create_image_builder_streaming_url(&self) -> crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder{
        crate::operation::create_image_builder_streaming_url::builders::CreateImageBuilderStreamingURLFluentBuilder::new(self.handle.clone())
    }
}
