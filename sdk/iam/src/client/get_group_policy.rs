// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetGroupPolicy`](crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_name(impl ::std::convert::Into<String>)`](crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder::set_group_name): <p>The name of the group the policy is associated with.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`policy_name(impl ::std::convert::Into<String>)`](crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder::set_policy_name): <p>The name of the policy document to get.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    /// - On success, responds with [`GetGroupPolicyOutput`](crate::operation::get_group_policy::GetGroupPolicyOutput) with field(s):
    ///   - [`group_name(Option<String>)`](crate::operation::get_group_policy::GetGroupPolicyOutput::group_name): <p>The group the policy is associated with.</p>
    ///   - [`policy_name(Option<String>)`](crate::operation::get_group_policy::GetGroupPolicyOutput::policy_name): <p>The name of the policy.</p>
    ///   - [`policy_document(Option<String>)`](crate::operation::get_group_policy::GetGroupPolicyOutput::policy_document): <p>The policy document.</p>  <p>IAM stores policies in JSON format. However, resources that were created using CloudFormation templates can be formatted in YAML. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// - On failure, responds with [`SdkError<GetGroupPolicyError>`](crate::operation::get_group_policy::GetGroupPolicyError)
    pub fn get_group_policy(
        &self,
    ) -> crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder {
        crate::operation::get_group_policy::builders::GetGroupPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
