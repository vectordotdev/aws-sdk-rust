// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetComponent`](crate::operation::get_component::builders::GetComponentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::get_component::builders::GetComponentFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::get_component::builders::GetComponentFluentBuilder::set_application_id): <p>The ID of the application.</p>
    ///   - [`component_id(impl ::std::convert::Into<String>)`](crate::operation::get_component::builders::GetComponentFluentBuilder::component_id) / [`set_component_id(Option<String>)`](crate::operation::get_component::builders::GetComponentFluentBuilder::set_component_id): <p>The ID of the component.</p>
    /// - On success, responds with [`GetComponentOutput`](crate::operation::get_component::GetComponentOutput) with field(s):
    ///   - [`component(Option<Component>)`](crate::operation::get_component::GetComponentOutput::component): <p>The component of an application registered with AWS Systems Manager for SAP.</p>
    /// - On failure, responds with [`SdkError<GetComponentError>`](crate::operation::get_component::GetComponentError)
    pub fn get_component(
        &self,
    ) -> crate::operation::get_component::builders::GetComponentFluentBuilder {
        crate::operation::get_component::builders::GetComponentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
