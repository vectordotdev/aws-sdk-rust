// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateWirelessGatewayFromCertificate`](crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateFluentBuilder::set_id): <p>The ID of the resource to update.</p>
    /// - On success, responds with [`DisassociateWirelessGatewayFromCertificateOutput`](crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateOutput)
    /// - On failure, responds with [`SdkError<DisassociateWirelessGatewayFromCertificateError>`](crate::operation::disassociate_wireless_gateway_from_certificate::DisassociateWirelessGatewayFromCertificateError)
    pub fn disassociate_wireless_gateway_from_certificate(&self) -> crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateFluentBuilder{
        crate::operation::disassociate_wireless_gateway_from_certificate::builders::DisassociateWirelessGatewayFromCertificateFluentBuilder::new(self.handle.clone())
    }
}
