// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateStreamingImage`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::set_description): <p>A human-readable description of the streaming image.</p>
    ///   - [`ec2_image_id(impl ::std::convert::Into<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::ec2_image_id) / [`set_ec2_image_id(Option<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::set_ec2_image_id): <p>The ID of an EC2 machine image with which to create this streaming image.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::set_name): <p>A friendly name for a streaming image resource.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::set_studio_id): <p>The studio ID. </p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::set_tags): <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    /// - On success, responds with [`CreateStreamingImageOutput`](crate::operation::create_streaming_image::CreateStreamingImageOutput) with field(s):
    ///   - [`streaming_image(Option<StreamingImage>)`](crate::operation::create_streaming_image::CreateStreamingImageOutput::streaming_image): <p>The streaming image.</p>
    /// - On failure, responds with [`SdkError<CreateStreamingImageError>`](crate::operation::create_streaming_image::CreateStreamingImageError)
    pub fn create_streaming_image(
        &self,
    ) -> crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder {
        crate::operation::create_streaming_image::builders::CreateStreamingImageFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
