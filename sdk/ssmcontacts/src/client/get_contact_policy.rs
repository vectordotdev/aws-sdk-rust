// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetContactPolicy`](crate::operation::get_contact_policy::builders::GetContactPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contact_arn(impl ::std::convert::Into<String>)`](crate::operation::get_contact_policy::builders::GetContactPolicyFluentBuilder::contact_arn) / [`set_contact_arn(Option<String>)`](crate::operation::get_contact_policy::builders::GetContactPolicyFluentBuilder::set_contact_arn): <p>The Amazon Resource Name (ARN) of the contact or escalation plan.</p>
    /// - On success, responds with [`GetContactPolicyOutput`](crate::operation::get_contact_policy::GetContactPolicyOutput) with field(s):
    ///   - [`contact_arn(Option<String>)`](crate::operation::get_contact_policy::GetContactPolicyOutput::contact_arn): <p>The ARN of the contact or escalation plan.</p>
    ///   - [`policy(Option<String>)`](crate::operation::get_contact_policy::GetContactPolicyOutput::policy): <p>Details about the resource policy attached to the contact or escalation plan.</p>
    /// - On failure, responds with [`SdkError<GetContactPolicyError>`](crate::operation::get_contact_policy::GetContactPolicyError)
    pub fn get_contact_policy(
        &self,
    ) -> crate::operation::get_contact_policy::builders::GetContactPolicyFluentBuilder {
        crate::operation::get_contact_policy::builders::GetContactPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
