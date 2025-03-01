// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListConnectorDefinitions`](crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(impl ::std::convert::Into<String>)`](crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<String>)`](crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder::set_max_results): The maximum number of results to be returned per request.
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder::set_next_token): The token for the next set of results, or ''null'' if there are no additional results.
    /// - On success, responds with [`ListConnectorDefinitionsOutput`](crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput) with field(s):
    ///   - [`definitions(Option<Vec<DefinitionInformation>>)`](crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput::definitions): Information about a definition.
    ///   - [`next_token(Option<String>)`](crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput::next_token): The token for the next set of results, or ''null'' if there are no additional results.
    /// - On failure, responds with [`SdkError<ListConnectorDefinitionsError>`](crate::operation::list_connector_definitions::ListConnectorDefinitionsError)
    pub fn list_connector_definitions(
        &self,
    ) -> crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder
    {
        crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
