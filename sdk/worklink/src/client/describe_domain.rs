// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDomain`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::set_fleet_arn): <p>The ARN of the fleet.</p>
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::set_domain_name): <p>The name of the domain.</p>
    /// - On success, responds with [`DescribeDomainOutput`](crate::operation::describe_domain::DescribeDomainOutput) with field(s):
    ///   - [`domain_name(Option<String>)`](crate::operation::describe_domain::DescribeDomainOutput::domain_name): <p>The name of the domain.</p>
    ///   - [`display_name(Option<String>)`](crate::operation::describe_domain::DescribeDomainOutput::display_name): <p>The name to display.</p>
    ///   - [`created_time(Option<DateTime>)`](crate::operation::describe_domain::DescribeDomainOutput::created_time): <p>The time that the domain was added.</p>
    ///   - [`domain_status(Option<DomainStatus>)`](crate::operation::describe_domain::DescribeDomainOutput::domain_status): <p>The current state for the domain.</p>
    ///   - [`acm_certificate_arn(Option<String>)`](crate::operation::describe_domain::DescribeDomainOutput::acm_certificate_arn): <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    /// - On failure, responds with [`SdkError<DescribeDomainError>`](crate::operation::describe_domain::DescribeDomainError)
    #[deprecated(
        note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK."
    )]
    pub fn describe_domain(
        &self,
    ) -> crate::operation::describe_domain::builders::DescribeDomainFluentBuilder {
        crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
