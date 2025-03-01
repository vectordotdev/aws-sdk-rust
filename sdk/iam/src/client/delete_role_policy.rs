// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRolePolicy`](crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_name(impl ::std::convert::Into<String>)`](crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder::set_role_name): <p>The name (friendly name, not ARN) identifying the role that the policy is embedded in.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`policy_name(impl ::std::convert::Into<String>)`](crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder::set_policy_name): <p>The name of the inline policy to delete from the specified IAM role.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    /// - On success, responds with [`DeleteRolePolicyOutput`](crate::operation::delete_role_policy::DeleteRolePolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteRolePolicyError>`](crate::operation::delete_role_policy::DeleteRolePolicyError)
    pub fn delete_role_policy(
        &self,
    ) -> crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder {
        crate::operation::delete_role_policy::builders::DeleteRolePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
