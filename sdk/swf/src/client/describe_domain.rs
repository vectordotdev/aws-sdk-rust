// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDomain`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::set_name): <p>The name of the domain to describe.</p>
    /// - On success, responds with [`DescribeDomainOutput`](crate::operation::describe_domain::DescribeDomainOutput) with field(s):
    ///   - [`domain_info(Option<DomainInfo>)`](crate::operation::describe_domain::DescribeDomainOutput::domain_info): <p>The basic information about a domain, such as its name, status, and description.</p>
    ///   - [`configuration(Option<DomainConfiguration>)`](crate::operation::describe_domain::DescribeDomainOutput::configuration): <p>The domain configuration. Currently, this includes only the domain's retention period.</p>
    /// - On failure, responds with [`SdkError<DescribeDomainError>`](crate::operation::describe_domain::DescribeDomainError)
    pub fn describe_domain(
        &self,
    ) -> crate::operation::describe_domain::builders::DescribeDomainFluentBuilder {
        crate::operation::describe_domain::builders::DescribeDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
