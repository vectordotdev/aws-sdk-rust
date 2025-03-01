// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDomain`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl ::std::convert::Into<String>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::set_domain_id): <p>The identifier of the domain to be updated.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::set_name): <p>The name of the domain.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::set_description): <p>A brief description about this domain.</p>
    ///   - [`server_side_encryption_configuration(ServerSideEncryptionConfiguration)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::server_side_encryption_configuration) / [`set_server_side_encryption_configuration(Option<ServerSideEncryptionConfiguration>)`](crate::operation::update_domain::builders::UpdateDomainFluentBuilder::set_server_side_encryption_configuration): <p>The configuration, containing the KMS key identifier, to be used by Voice ID for the server-side encryption of your data. Changing the domain's associated KMS key immediately triggers an asynchronous process to remove dependency on the old KMS key, such that the domain's data can only be accessed using the new KMS key. The domain's <code>ServerSideEncryptionUpdateDetails</code> contains the details for this process.</p>
    /// - On success, responds with [`UpdateDomainOutput`](crate::operation::update_domain::UpdateDomainOutput) with field(s):
    ///   - [`domain(Option<Domain>)`](crate::operation::update_domain::UpdateDomainOutput::domain): <p>Details about the updated domain</p>
    /// - On failure, responds with [`SdkError<UpdateDomainError>`](crate::operation::update_domain::UpdateDomainError)
    pub fn update_domain(
        &self,
    ) -> crate::operation::update_domain::builders::UpdateDomainFluentBuilder {
        crate::operation::update_domain::builders::UpdateDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
