// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePolicy`](crate::operation::delete_policy::builders::DeletePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_policy::builders::DeletePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::delete_policy::builders::DeletePolicyFluentBuilder::set_resource_arn): <p>The Amazon Resource Number (ARN) of the private CA that will have its policy deleted. You can find the CA's ARN by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action. The ARN value must have the form <code>arn:aws:acm-pca:region:account:certificate-authority/01234567-89ab-cdef-0123-0123456789ab</code>. </p>
    /// - On success, responds with [`DeletePolicyOutput`](crate::operation::delete_policy::DeletePolicyOutput)
    /// - On failure, responds with [`SdkError<DeletePolicyError>`](crate::operation::delete_policy::DeletePolicyError)
    pub fn delete_policy(
        &self,
    ) -> crate::operation::delete_policy::builders::DeletePolicyFluentBuilder {
        crate::operation::delete_policy::builders::DeletePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
