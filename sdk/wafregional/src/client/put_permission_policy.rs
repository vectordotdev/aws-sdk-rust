// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutPermissionPolicy`](crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the RuleGroup to which you want to attach the policy.</p>
    ///   - [`policy(impl ::std::convert::Into<String>)`](crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder::set_policy): <p>The policy to attach to the specified RuleGroup.</p>
    /// - On success, responds with [`PutPermissionPolicyOutput`](crate::operation::put_permission_policy::PutPermissionPolicyOutput)
    /// - On failure, responds with [`SdkError<PutPermissionPolicyError>`](crate::operation::put_permission_policy::PutPermissionPolicyError)
    pub fn put_permission_policy(
        &self,
    ) -> crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder {
        crate::operation::put_permission_policy::builders::PutPermissionPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
