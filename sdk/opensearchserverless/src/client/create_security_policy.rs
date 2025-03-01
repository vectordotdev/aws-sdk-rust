// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSecurityPolicy`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`r#type(SecurityPolicyType)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::type) / [`set_type(Option<SecurityPolicyType>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::set_type): <p>The type of security policy.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::set_name): <p>The name of the policy.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::set_description): <p>A description of the policy. Typically used to store information about the permissions defined in the policy.</p>
    ///   - [`policy(impl ::std::convert::Into<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::set_policy): <p>The JSON policy document to use as the content for the new policy.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    /// - On success, responds with [`CreateSecurityPolicyOutput`](crate::operation::create_security_policy::CreateSecurityPolicyOutput) with field(s):
    ///   - [`security_policy_detail(Option<SecurityPolicyDetail>)`](crate::operation::create_security_policy::CreateSecurityPolicyOutput::security_policy_detail): <p>Details about the created security policy.</p>
    /// - On failure, responds with [`SdkError<CreateSecurityPolicyError>`](crate::operation::create_security_policy::CreateSecurityPolicyError)
    pub fn create_security_policy(
        &self,
    ) -> crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder {
        crate::operation::create_security_policy::builders::CreateSecurityPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
