// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBuildBatch`](crate::operation::delete_build_batch::builders::DeleteBuildBatchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::delete_build_batch::builders::DeleteBuildBatchFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_build_batch::builders::DeleteBuildBatchFluentBuilder::set_id): <p>The identifier of the batch build to delete.</p>
    /// - On success, responds with [`DeleteBuildBatchOutput`](crate::operation::delete_build_batch::DeleteBuildBatchOutput) with field(s):
    ///   - [`status_code(Option<String>)`](crate::operation::delete_build_batch::DeleteBuildBatchOutput::status_code): <p>The status code.</p>
    ///   - [`builds_deleted(Option<Vec<String>>)`](crate::operation::delete_build_batch::DeleteBuildBatchOutput::builds_deleted): <p>An array of strings that contain the identifiers of the builds that were deleted.</p>
    ///   - [`builds_not_deleted(Option<Vec<BuildNotDeleted>>)`](crate::operation::delete_build_batch::DeleteBuildBatchOutput::builds_not_deleted): <p>An array of <code>BuildNotDeleted</code> objects that specify the builds that could not be deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteBuildBatchError>`](crate::operation::delete_build_batch::DeleteBuildBatchError)
    pub fn delete_build_batch(
        &self,
    ) -> crate::operation::delete_build_batch::builders::DeleteBuildBatchFluentBuilder {
        crate::operation::delete_build_batch::builders::DeleteBuildBatchFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
