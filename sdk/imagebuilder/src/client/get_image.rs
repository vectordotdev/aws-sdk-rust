// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetImage`](crate::operation::get_image::builders::GetImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`image_build_version_arn(impl ::std::convert::Into<String>)`](crate::operation::get_image::builders::GetImageFluentBuilder::image_build_version_arn) / [`set_image_build_version_arn(Option<String>)`](crate::operation::get_image::builders::GetImageFluentBuilder::set_image_build_version_arn): <p>The Amazon Resource Name (ARN) of the image that you want to get.</p>
    /// - On success, responds with [`GetImageOutput`](crate::operation::get_image::GetImageOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::get_image::GetImageOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`image(Option<Image>)`](crate::operation::get_image::GetImageOutput::image): <p>The image object.</p>
    /// - On failure, responds with [`SdkError<GetImageError>`](crate::operation::get_image::GetImageError)
    pub fn get_image(&self) -> crate::operation::get_image::builders::GetImageFluentBuilder {
        crate::operation::get_image::builders::GetImageFluentBuilder::new(self.handle.clone())
    }
}
