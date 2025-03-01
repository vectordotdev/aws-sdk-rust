// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableDomainAutoRenew`](crate::operation::enable_domain_auto_renew::builders::EnableDomainAutoRenewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::enable_domain_auto_renew::builders::EnableDomainAutoRenewFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::enable_domain_auto_renew::builders::EnableDomainAutoRenewFluentBuilder::set_domain_name): <p>The name of the domain that you want to enable automatic renewal for.</p>
    /// - On success, responds with [`EnableDomainAutoRenewOutput`](crate::operation::enable_domain_auto_renew::EnableDomainAutoRenewOutput)
    /// - On failure, responds with [`SdkError<EnableDomainAutoRenewError>`](crate::operation::enable_domain_auto_renew::EnableDomainAutoRenewError)
    pub fn enable_domain_auto_renew(
        &self,
    ) -> crate::operation::enable_domain_auto_renew::builders::EnableDomainAutoRenewFluentBuilder
    {
        crate::operation::enable_domain_auto_renew::builders::EnableDomainAutoRenewFluentBuilder::new(self.handle.clone())
    }
}
