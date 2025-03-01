// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDataIntegrations`](crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder::set_max_results): <p>The maximum number of results to return per page.</p>
    /// - On success, responds with [`ListDataIntegrationsOutput`](crate::operation::list_data_integrations::ListDataIntegrationsOutput) with field(s):
    ///   - [`data_integrations(Option<Vec<DataIntegrationSummary>>)`](crate::operation::list_data_integrations::ListDataIntegrationsOutput::data_integrations): <p>The DataIntegrations associated with this account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_data_integrations::ListDataIntegrationsOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListDataIntegrationsError>`](crate::operation::list_data_integrations::ListDataIntegrationsError)
    pub fn list_data_integrations(
        &self,
    ) -> crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder {
        crate::operation::list_data_integrations::builders::ListDataIntegrationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
