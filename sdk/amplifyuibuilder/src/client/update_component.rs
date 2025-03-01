// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateComponent`](crate::operation::update_component::builders::UpdateComponentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::set_app_id): <p>The unique ID for the Amplify app.</p>
    ///   - [`environment_name(impl ::std::convert::Into<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::environment_name) / [`set_environment_name(Option<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::set_environment_name): <p>The name of the backend environment that is part of the Amplify app.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::set_id): <p>The unique ID for the component.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::set_client_token): <p>The unique client token.</p>
    ///   - [`updated_component(UpdateComponentData)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::updated_component) / [`set_updated_component(Option<UpdateComponentData>)`](crate::operation::update_component::builders::UpdateComponentFluentBuilder::set_updated_component): <p>The configuration of the updated component.</p>
    /// - On success, responds with [`UpdateComponentOutput`](crate::operation::update_component::UpdateComponentOutput) with field(s):
    ///   - [`entity(Option<Component>)`](crate::operation::update_component::UpdateComponentOutput::entity): <p>Describes the configuration of the updated component.</p>
    /// - On failure, responds with [`SdkError<UpdateComponentError>`](crate::operation::update_component::UpdateComponentError)
    pub fn update_component(
        &self,
    ) -> crate::operation::update_component::builders::UpdateComponentFluentBuilder {
        crate::operation::update_component::builders::UpdateComponentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
