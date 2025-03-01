// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBuild`](crate::operation::delete_build::builders::DeleteBuildFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`build_id(impl ::std::convert::Into<String>)`](crate::operation::delete_build::builders::DeleteBuildFluentBuilder::build_id) / [`set_build_id(Option<String>)`](crate::operation::delete_build::builders::DeleteBuildFluentBuilder::set_build_id): <p>A unique identifier for the build to delete. You can use either the build ID or ARN value. </p>
    /// - On success, responds with [`DeleteBuildOutput`](crate::operation::delete_build::DeleteBuildOutput)
    /// - On failure, responds with [`SdkError<DeleteBuildError>`](crate::operation::delete_build::DeleteBuildError)
    pub fn delete_build(
        &self,
    ) -> crate::operation::delete_build::builders::DeleteBuildFluentBuilder {
        crate::operation::delete_build::builders::DeleteBuildFluentBuilder::new(self.handle.clone())
    }
}
