// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportCertificate`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_identifier(impl ::std::convert::Into<String>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::certificate_identifier) / [`set_certificate_identifier(Option<String>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::set_certificate_identifier): <p>A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    ///   - [`certificate_pem(impl ::std::convert::Into<String>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::certificate_pem) / [`set_certificate_pem(Option<String>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::set_certificate_pem): <p>The contents of a <code>.pem</code> file, which contains an X.509 certificate.</p>
    ///   - [`certificate_wallet(Blob)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::certificate_wallet) / [`set_certificate_wallet(Option<Blob>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::set_certificate_wallet): <p>The location of an imported Oracle Wallet certificate for use with SSL. Provide the name of a <code>.sso</code> file using the <code>fileb://</code> prefix. You can't provide the certificate inline.</p>  <p>Example: <code>filebase64("${path.root}/rds-ca-2019-root.sso")</code> </p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::set_tags): <p>The tags associated with the certificate.</p>
    /// - On success, responds with [`ImportCertificateOutput`](crate::operation::import_certificate::ImportCertificateOutput) with field(s):
    ///   - [`certificate(Option<Certificate>)`](crate::operation::import_certificate::ImportCertificateOutput::certificate): <p>The certificate to be uploaded.</p>
    /// - On failure, responds with [`SdkError<ImportCertificateError>`](crate::operation::import_certificate::ImportCertificateError)
    pub fn import_certificate(
        &self,
    ) -> crate::operation::import_certificate::builders::ImportCertificateFluentBuilder {
        crate::operation::import_certificate::builders::ImportCertificateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
