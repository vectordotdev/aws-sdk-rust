// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCertificates`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_statuses(Vec<CertificateStatus>)`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::certificate_statuses) / [`set_certificate_statuses(Option<Vec<CertificateStatus>>)`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::set_certificate_statuses): <p>The status of the certificates for which to return information.</p>  <p>For example, specify <code>ISSUED</code> to return only certificates with an <code>ISSUED</code> status.</p>  <p>When omitted, the response includes all of your certificates in the Amazon Web Services Region where the request is made, regardless of their current status.</p>
    ///   - [`include_certificate_details(bool)`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::include_certificate_details) / [`set_include_certificate_details(Option<bool>)`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::set_include_certificate_details): <p>Indicates whether to include detailed information about the certificates in the response.</p>  <p>When omitted, the response includes only the certificate names, Amazon Resource Names (ARNs), domain names, and tags.</p>
    ///   - [`certificate_name(impl ::std::convert::Into<String>)`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::certificate_name) / [`set_certificate_name(Option<String>)`](crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::set_certificate_name): <p>The name for the certificate for which to return information.</p>  <p>When omitted, the response includes all of your certificates in the Amazon Web Services Region where the request is made.</p>
    /// - On success, responds with [`GetCertificatesOutput`](crate::operation::get_certificates::GetCertificatesOutput) with field(s):
    ///   - [`certificates(Option<Vec<CertificateSummary>>)`](crate::operation::get_certificates::GetCertificatesOutput::certificates): <p>An object that describes certificates.</p>
    /// - On failure, responds with [`SdkError<GetCertificatesError>`](crate::operation::get_certificates::GetCertificatesError)
    pub fn get_certificates(
        &self,
    ) -> crate::operation::get_certificates::builders::GetCertificatesFluentBuilder {
        crate::operation::get_certificates::builders::GetCertificatesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
