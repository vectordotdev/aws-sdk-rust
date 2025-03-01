// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWirelessGateway`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::set_name): <p>The name of the new resource.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::set_description): <p>The description of the new resource.</p>
    ///   - [`lo_ra_wan(LoRaWanGateway)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::lo_ra_wan) / [`set_lo_ra_wan(Option<LoRaWanGateway>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::set_lo_ra_wan): <p>The gateway configuration information to use to create the wireless gateway.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::set_tags): <p>The tags to attach to the new wireless gateway. Tags are metadata that you can use to manage a resource.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::set_client_request_token): <p>Each resource must have a unique client request token. If you try to create a new resource with the same token as a resource that already exists, an exception occurs. If you omit this value, AWS SDKs will automatically generate a unique client request. </p>
    /// - On success, responds with [`CreateWirelessGatewayOutput`](crate::operation::create_wireless_gateway::CreateWirelessGatewayOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_wireless_gateway::CreateWirelessGatewayOutput::arn): <p>The Amazon Resource Name of the new resource.</p>
    ///   - [`id(Option<String>)`](crate::operation::create_wireless_gateway::CreateWirelessGatewayOutput::id): <p>The ID of the new wireless gateway.</p>
    /// - On failure, responds with [`SdkError<CreateWirelessGatewayError>`](crate::operation::create_wireless_gateway::CreateWirelessGatewayError)
    pub fn create_wireless_gateway(
        &self,
    ) -> crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder
    {
        crate::operation::create_wireless_gateway::builders::CreateWirelessGatewayFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
