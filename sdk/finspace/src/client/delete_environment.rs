// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEnvironment`](crate::operation::delete_environment::builders::DeleteEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`environment_id(impl ::std::convert::Into<String>)`](crate::operation::delete_environment::builders::DeleteEnvironmentFluentBuilder::environment_id) / [`set_environment_id(Option<String>)`](crate::operation::delete_environment::builders::DeleteEnvironmentFluentBuilder::set_environment_id): <p>The identifier for the FinSpace environment.</p>
    /// - On success, responds with [`DeleteEnvironmentOutput`](crate::operation::delete_environment::DeleteEnvironmentOutput)
    /// - On failure, responds with [`SdkError<DeleteEnvironmentError>`](crate::operation::delete_environment::DeleteEnvironmentError)
    pub fn delete_environment(
        &self,
    ) -> crate::operation::delete_environment::builders::DeleteEnvironmentFluentBuilder {
        crate::operation::delete_environment::builders::DeleteEnvironmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
