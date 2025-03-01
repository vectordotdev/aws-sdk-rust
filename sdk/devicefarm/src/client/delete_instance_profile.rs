// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteInstanceProfile`](crate::operation::delete_instance_profile::builders::DeleteInstanceProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::delete_instance_profile::builders::DeleteInstanceProfileFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::delete_instance_profile::builders::DeleteInstanceProfileFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the instance profile you are requesting to delete.</p>
    /// - On success, responds with [`DeleteInstanceProfileOutput`](crate::operation::delete_instance_profile::DeleteInstanceProfileOutput)
    /// - On failure, responds with [`SdkError<DeleteInstanceProfileError>`](crate::operation::delete_instance_profile::DeleteInstanceProfileError)
    pub fn delete_instance_profile(
        &self,
    ) -> crate::operation::delete_instance_profile::builders::DeleteInstanceProfileFluentBuilder
    {
        crate::operation::delete_instance_profile::builders::DeleteInstanceProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
