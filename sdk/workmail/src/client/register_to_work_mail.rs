// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterToWorkMail`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl ::std::convert::Into<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::set_organization_id): <p>The identifier for the organization under which the user, group, or resource exists.</p>
    ///   - [`entity_id(impl ::std::convert::Into<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::entity_id) / [`set_entity_id(Option<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::set_entity_id): <p>The identifier for the user, group, or resource to be updated.</p>
    ///   - [`email(impl ::std::convert::Into<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::email) / [`set_email(Option<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::set_email): <p>The email for the user, group, or resource to be updated.</p>
    /// - On success, responds with [`RegisterToWorkMailOutput`](crate::operation::register_to_work_mail::RegisterToWorkMailOutput)
    /// - On failure, responds with [`SdkError<RegisterToWorkMailError>`](crate::operation::register_to_work_mail::RegisterToWorkMailError)
    pub fn register_to_work_mail(
        &self,
    ) -> crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder {
        crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
