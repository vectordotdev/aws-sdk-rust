// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateListener`](crate::operation::create_listener::builders::CreateListenerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`accelerator_arn(impl ::std::convert::Into<String>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::accelerator_arn) / [`set_accelerator_arn(Option<String>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::set_accelerator_arn): <p>The Amazon Resource Name (ARN) of your accelerator.</p>
    ///   - [`port_ranges(Vec<PortRange>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::port_ranges) / [`set_port_ranges(Option<Vec<PortRange>>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::set_port_ranges): <p>The list of port ranges to support for connections from clients to your accelerator.</p>
    ///   - [`protocol(Protocol)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::protocol) / [`set_protocol(Option<Protocol>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::set_protocol): <p>The protocol for connections from clients to your accelerator.</p>
    ///   - [`client_affinity(ClientAffinity)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::client_affinity) / [`set_client_affinity(Option<ClientAffinity>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::set_client_affinity): <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p>  <p>Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p>  <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p>  <p>The default value is <code>NONE</code>.</p>
    ///   - [`idempotency_token(impl ::std::convert::Into<String>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::operation::create_listener::builders::CreateListenerFluentBuilder::set_idempotency_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    /// - On success, responds with [`CreateListenerOutput`](crate::operation::create_listener::CreateListenerOutput) with field(s):
    ///   - [`listener(Option<Listener>)`](crate::operation::create_listener::CreateListenerOutput::listener): <p>The listener that you've created.</p>
    /// - On failure, responds with [`SdkError<CreateListenerError>`](crate::operation::create_listener::CreateListenerError)
    pub fn create_listener(
        &self,
    ) -> crate::operation::create_listener::builders::CreateListenerFluentBuilder {
        crate::operation::create_listener::builders::CreateListenerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
