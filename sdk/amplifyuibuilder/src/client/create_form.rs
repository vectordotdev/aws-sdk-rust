// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateForm`](crate::operation::create_form::builders::CreateFormFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::set_app_id): <p>The unique ID of the Amplify app to associate with the form.</p>
    ///   - [`environment_name(impl ::std::convert::Into<String>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::environment_name) / [`set_environment_name(Option<String>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::set_environment_name): <p>The name of the backend environment that is a part of the Amplify app.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::set_client_token): <p>The unique client token.</p>
    ///   - [`form_to_create(CreateFormData)`](crate::operation::create_form::builders::CreateFormFluentBuilder::form_to_create) / [`set_form_to_create(Option<CreateFormData>)`](crate::operation::create_form::builders::CreateFormFluentBuilder::set_form_to_create): <p>Represents the configuration of the form to create.</p>
    /// - On success, responds with [`CreateFormOutput`](crate::operation::create_form::CreateFormOutput) with field(s):
    ///   - [`entity(Option<Form>)`](crate::operation::create_form::CreateFormOutput::entity): <p>Describes the configuration of the new form.</p>
    /// - On failure, responds with [`SdkError<CreateFormError>`](crate::operation::create_form::CreateFormError)
    pub fn create_form(&self) -> crate::operation::create_form::builders::CreateFormFluentBuilder {
        crate::operation::create_form::builders::CreateFormFluentBuilder::new(self.handle.clone())
    }
}
