// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRestApi`](crate::operation::delete_rest_api::builders::DeleteRestApiFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl ::std::convert::Into<String>)`](crate::operation::delete_rest_api::builders::DeleteRestApiFluentBuilder::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::operation::delete_rest_api::builders::DeleteRestApiFluentBuilder::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    /// - On success, responds with [`DeleteRestApiOutput`](crate::operation::delete_rest_api::DeleteRestApiOutput)
    /// - On failure, responds with [`SdkError<DeleteRestApiError>`](crate::operation::delete_rest_api::DeleteRestApiError)
    pub fn delete_rest_api(
        &self,
    ) -> crate::operation::delete_rest_api::builders::DeleteRestApiFluentBuilder {
        crate::operation::delete_rest_api::builders::DeleteRestApiFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
