// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDestination`](crate::operation::delete_destination::builders::DeleteDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_destination::builders::DeleteDestinationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_destination::builders::DeleteDestinationFluentBuilder::set_name): <p>The name of the resource to delete.</p>
    /// - On success, responds with [`DeleteDestinationOutput`](crate::operation::delete_destination::DeleteDestinationOutput)
    /// - On failure, responds with [`SdkError<DeleteDestinationError>`](crate::operation::delete_destination::DeleteDestinationError)
    pub fn delete_destination(
        &self,
    ) -> crate::operation::delete_destination::builders::DeleteDestinationFluentBuilder {
        crate::operation::delete_destination::builders::DeleteDestinationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
