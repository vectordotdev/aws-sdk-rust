// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RestoreAddressToClassic`](crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`public_ip(impl ::std::convert::Into<String>)`](crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder::public_ip) / [`set_public_ip(Option<String>)`](crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder::set_public_ip): <p>The Elastic IP address.</p>
    /// - On success, responds with [`RestoreAddressToClassicOutput`](crate::operation::restore_address_to_classic::RestoreAddressToClassicOutput) with field(s):
    ///   - [`public_ip(Option<String>)`](crate::operation::restore_address_to_classic::RestoreAddressToClassicOutput::public_ip): <p>The Elastic IP address.</p>
    ///   - [`status(Option<Status>)`](crate::operation::restore_address_to_classic::RestoreAddressToClassicOutput::status): <p>The move status for the IP address.</p>
    /// - On failure, responds with [`SdkError<RestoreAddressToClassicError>`](crate::operation::restore_address_to_classic::RestoreAddressToClassicError)
    pub fn restore_address_to_classic(
        &self,
    ) -> crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder
    {
        crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicFluentBuilder::new(self.handle.clone())
    }
}
