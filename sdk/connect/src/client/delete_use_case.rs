// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteUseCase`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`integration_association_id(impl ::std::convert::Into<String>)`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::integration_association_id) / [`set_integration_association_id(Option<String>)`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::set_integration_association_id): <p>The identifier for the integration association.</p>
    ///   - [`use_case_id(impl ::std::convert::Into<String>)`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::use_case_id) / [`set_use_case_id(Option<String>)`](crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::set_use_case_id): <p>The identifier for the use case.</p>
    /// - On success, responds with [`DeleteUseCaseOutput`](crate::operation::delete_use_case::DeleteUseCaseOutput)
    /// - On failure, responds with [`SdkError<DeleteUseCaseError>`](crate::operation::delete_use_case::DeleteUseCaseError)
    pub fn delete_use_case(
        &self,
    ) -> crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder {
        crate::operation::delete_use_case::builders::DeleteUseCaseFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
