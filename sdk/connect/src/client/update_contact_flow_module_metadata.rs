// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateContactFlowModuleMetadata`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`contact_flow_module_id(impl ::std::convert::Into<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::contact_flow_module_id) / [`set_contact_flow_module_id(Option<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::set_contact_flow_module_id): <p>The identifier of the flow module.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::set_name): <p>The name of the flow module.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::set_description): <p>The description of the flow module.</p>
    ///   - [`state(ContactFlowModuleState)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::state) / [`set_state(Option<ContactFlowModuleState>)`](crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::set_state): <p>The state of flow module.</p>
    /// - On success, responds with [`UpdateContactFlowModuleMetadataOutput`](crate::operation::update_contact_flow_module_metadata::UpdateContactFlowModuleMetadataOutput)
    /// - On failure, responds with [`SdkError<UpdateContactFlowModuleMetadataError>`](crate::operation::update_contact_flow_module_metadata::UpdateContactFlowModuleMetadataError)
    pub fn update_contact_flow_module_metadata(&self) -> crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder{
        crate::operation::update_contact_flow_module_metadata::builders::UpdateContactFlowModuleMetadataFluentBuilder::new(self.handle.clone())
    }
}
