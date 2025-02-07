// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCoreNetwork`](crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`core_network_id(impl ::std::convert::Into<String>)`](crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder::core_network_id) / [`set_core_network_id(Option<String>)`](crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder::set_core_network_id): <p>The ID of a core network.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder::set_description): <p>The description of the update.</p>
    /// - On success, responds with [`UpdateCoreNetworkOutput`](crate::operation::update_core_network::UpdateCoreNetworkOutput) with field(s):
    ///   - [`core_network(Option<CoreNetwork>)`](crate::operation::update_core_network::UpdateCoreNetworkOutput::core_network): <p>Returns information about a core network update.</p>
    /// - On failure, responds with [`SdkError<UpdateCoreNetworkError>`](crate::operation::update_core_network::UpdateCoreNetworkError)
    pub fn update_core_network(
        &self,
    ) -> crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder {
        crate::operation::update_core_network::builders::UpdateCoreNetworkFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
