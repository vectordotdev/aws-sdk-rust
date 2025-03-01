// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWirelessGatewayTaskDefinitions`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::set_max_results): <p>The maximum number of results to return in this operation.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`task_definition_type(WirelessGatewayTaskDefinitionType)`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::task_definition_type) / [`set_task_definition_type(Option<WirelessGatewayTaskDefinitionType>)`](crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::set_task_definition_type): <p>A filter to list only the wireless gateway task definitions that use this task definition type.</p>
    /// - On success, responds with [`ListWirelessGatewayTaskDefinitionsOutput`](crate::operation::list_wireless_gateway_task_definitions::ListWirelessGatewayTaskDefinitionsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_wireless_gateway_task_definitions::ListWirelessGatewayTaskDefinitionsOutput::next_token): <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    ///   - [`task_definitions(Option<Vec<UpdateWirelessGatewayTaskEntry>>)`](crate::operation::list_wireless_gateway_task_definitions::ListWirelessGatewayTaskDefinitionsOutput::task_definitions): <p>The list of task definitions.</p>
    /// - On failure, responds with [`SdkError<ListWirelessGatewayTaskDefinitionsError>`](crate::operation::list_wireless_gateway_task_definitions::ListWirelessGatewayTaskDefinitionsError)
    pub fn list_wireless_gateway_task_definitions(&self) -> crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder{
        crate::operation::list_wireless_gateway_task_definitions::builders::ListWirelessGatewayTaskDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
