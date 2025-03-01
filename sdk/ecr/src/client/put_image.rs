// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutImage`](crate::operation::put_image::builders::PutImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`registry_id(impl ::std::convert::Into<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::registry_id) / [`set_registry_id(Option<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::set_registry_id): <p>The Amazon Web Services account ID associated with the registry that contains the repository in which to put the image. If you do not specify a registry, the default registry is assumed.</p>
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::set_repository_name): <p>The name of the repository in which to put the image.</p>
    ///   - [`image_manifest(impl ::std::convert::Into<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::image_manifest) / [`set_image_manifest(Option<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::set_image_manifest): <p>The image manifest corresponding to the image to be uploaded.</p>
    ///   - [`image_manifest_media_type(impl ::std::convert::Into<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::image_manifest_media_type) / [`set_image_manifest_media_type(Option<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::set_image_manifest_media_type): <p>The media type of the image manifest. If you push an image manifest that does not contain the <code>mediaType</code> field, you must specify the <code>imageManifestMediaType</code> in the request.</p>
    ///   - [`image_tag(impl ::std::convert::Into<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::image_tag) / [`set_image_tag(Option<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::set_image_tag): <p>The tag to associate with the image. This parameter is required for images that use the Docker Image Manifest V2 Schema 2 or Open Container Initiative (OCI) formats.</p>
    ///   - [`image_digest(impl ::std::convert::Into<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::image_digest) / [`set_image_digest(Option<String>)`](crate::operation::put_image::builders::PutImageFluentBuilder::set_image_digest): <p>The image digest of the image manifest corresponding to the image.</p>
    /// - On success, responds with [`PutImageOutput`](crate::operation::put_image::PutImageOutput) with field(s):
    ///   - [`image(Option<Image>)`](crate::operation::put_image::PutImageOutput::image): <p>Details of the image uploaded.</p>
    /// - On failure, responds with [`SdkError<PutImageError>`](crate::operation::put_image::PutImageError)
    pub fn put_image(&self) -> crate::operation::put_image::builders::PutImageFluentBuilder {
        crate::operation::put_image::builders::PutImageFluentBuilder::new(self.handle.clone())
    }
}
