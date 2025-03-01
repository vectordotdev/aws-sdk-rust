// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRepository`](crate::operation::delete_repository::builders::DeleteRepositoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::delete_repository::builders::DeleteRepositoryFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::delete_repository::builders::DeleteRepositoryFluentBuilder::set_repository_name): <p>The name of the repository to delete.</p>
    /// - On success, responds with [`DeleteRepositoryOutput`](crate::operation::delete_repository::DeleteRepositoryOutput) with field(s):
    ///   - [`repository_id(Option<String>)`](crate::operation::delete_repository::DeleteRepositoryOutput::repository_id): <p>The ID of the repository that was deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteRepositoryError>`](crate::operation::delete_repository::DeleteRepositoryError)
    pub fn delete_repository(
        &self,
    ) -> crate::operation::delete_repository::builders::DeleteRepositoryFluentBuilder {
        crate::operation::delete_repository::builders::DeleteRepositoryFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
