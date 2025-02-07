// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEffectivePolicies`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`principal(impl ::std::convert::Into<String>)`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::principal) / [`set_principal(Option<String>)`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::set_principal): <p>The principal. Valid principals are CertificateArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:cert/<i>certificateId</i>), thingGroupArn (arn:aws:iot:<i>region</i>:<i>accountId</i>:thinggroup/<i>groupName</i>) and CognitoId (<i>region</i>:<i>id</i>).</p>
    ///   - [`cognito_identity_pool_id(impl ::std::convert::Into<String>)`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::cognito_identity_pool_id) / [`set_cognito_identity_pool_id(Option<String>)`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::set_cognito_identity_pool_id): <p>The Cognito identity pool ID.</p>
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::set_thing_name): <p>The thing name.</p>
    /// - On success, responds with [`GetEffectivePoliciesOutput`](crate::operation::get_effective_policies::GetEffectivePoliciesOutput) with field(s):
    ///   - [`effective_policies(Option<Vec<EffectivePolicy>>)`](crate::operation::get_effective_policies::GetEffectivePoliciesOutput::effective_policies): <p>The effective policies.</p>
    /// - On failure, responds with [`SdkError<GetEffectivePoliciesError>`](crate::operation::get_effective_policies::GetEffectivePoliciesError)
    pub fn get_effective_policies(
        &self,
    ) -> crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder {
        crate::operation::get_effective_policies::builders::GetEffectivePoliciesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
