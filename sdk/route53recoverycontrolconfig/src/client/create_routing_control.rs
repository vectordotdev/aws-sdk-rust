// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRoutingControl`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::set_client_token): <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p>
    ///   - [`cluster_arn(impl ::std::convert::Into<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::set_cluster_arn): <p>The Amazon Resource Name (ARN) of the cluster that includes the routing control.</p>
    ///   - [`control_panel_arn(impl ::std::convert::Into<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::control_panel_arn) / [`set_control_panel_arn(Option<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::set_control_panel_arn): <p>The Amazon Resource Name (ARN) of the control panel that includes the routing control.</p>
    ///   - [`routing_control_name(impl ::std::convert::Into<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::routing_control_name) / [`set_routing_control_name(Option<String>)`](crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::set_routing_control_name): <p>The name of the routing control.</p>
    /// - On success, responds with [`CreateRoutingControlOutput`](crate::operation::create_routing_control::CreateRoutingControlOutput) with field(s):
    ///   - [`routing_control(Option<RoutingControl>)`](crate::operation::create_routing_control::CreateRoutingControlOutput::routing_control): <p>The routing control that is created.</p>
    /// - On failure, responds with [`SdkError<CreateRoutingControlError>`](crate::operation::create_routing_control::CreateRoutingControlError)
    pub fn create_routing_control(
        &self,
    ) -> crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder {
        crate::operation::create_routing_control::builders::CreateRoutingControlFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
