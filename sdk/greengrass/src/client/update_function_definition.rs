// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateFunctionDefinition`](crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_definition_id(impl ::std::convert::Into<String>)`](crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder::function_definition_id) / [`set_function_definition_id(Option<String>)`](crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder::set_function_definition_id): The ID of the Lambda function definition.
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder::set_name): The name of the definition.
    /// - On success, responds with [`UpdateFunctionDefinitionOutput`](crate::operation::update_function_definition::UpdateFunctionDefinitionOutput)
    /// - On failure, responds with [`SdkError<UpdateFunctionDefinitionError>`](crate::operation::update_function_definition::UpdateFunctionDefinitionError)
    pub fn update_function_definition(
        &self,
    ) -> crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder
    {
        crate::operation::update_function_definition::builders::UpdateFunctionDefinitionFluentBuilder::new(self.handle.clone())
    }
}
