// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagCertificateAuthority`](crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_authority_arn(impl ::std::convert::Into<String>)`](crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder::certificate_authority_arn) / [`set_certificate_authority_arn(Option<String>)`](crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder::set_certificate_authority_arn): <p>The Amazon Resource Name (ARN) that was returned when you called <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html">CreateCertificateAuthority</a>. This must be of the form: </p>  <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder::set_tags): <p>List of tags to be associated with the CA.</p>
    /// - On success, responds with [`TagCertificateAuthorityOutput`](crate::operation::tag_certificate_authority::TagCertificateAuthorityOutput)
    /// - On failure, responds with [`SdkError<TagCertificateAuthorityError>`](crate::operation::tag_certificate_authority::TagCertificateAuthorityError)
    pub fn tag_certificate_authority(
        &self,
    ) -> crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder
    {
        crate::operation::tag_certificate_authority::builders::TagCertificateAuthorityFluentBuilder::new(self.handle.clone())
    }
}
