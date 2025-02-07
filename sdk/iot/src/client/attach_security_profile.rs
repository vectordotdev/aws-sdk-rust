// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AttachSecurityProfile`](crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`security_profile_name(impl ::std::convert::Into<String>)`](crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder::security_profile_name) / [`set_security_profile_name(Option<String>)`](crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder::set_security_profile_name): <p>The security profile that is attached.</p>
    ///   - [`security_profile_target_arn(impl ::std::convert::Into<String>)`](crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder::security_profile_target_arn) / [`set_security_profile_target_arn(Option<String>)`](crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder::set_security_profile_target_arn): <p>The ARN of the target (thing group) to which the security profile is attached.</p>
    /// - On success, responds with [`AttachSecurityProfileOutput`](crate::operation::attach_security_profile::AttachSecurityProfileOutput)
    /// - On failure, responds with [`SdkError<AttachSecurityProfileError>`](crate::operation::attach_security_profile::AttachSecurityProfileError)
    pub fn attach_security_profile(
        &self,
    ) -> crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder
    {
        crate::operation::attach_security_profile::builders::AttachSecurityProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
