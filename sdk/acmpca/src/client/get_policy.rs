// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPolicy`](crate::operation::get_policy::builders::GetPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::get_policy::builders::GetPolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::get_policy::builders::GetPolicyFluentBuilder::set_resource_arn): <p>The Amazon Resource Number (ARN) of the private CA that will have its policy retrieved. You can find the CA's ARN by calling the ListCertificateAuthorities action. </p>
    /// - On success, responds with [`GetPolicyOutput`](crate::operation::get_policy::GetPolicyOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::operation::get_policy::GetPolicyOutput::policy): <p>The policy attached to the private CA as a JSON document.</p>
    /// - On failure, responds with [`SdkError<GetPolicyError>`](crate::operation::get_policy::GetPolicyError)
    pub fn get_policy(&self) -> crate::operation::get_policy::builders::GetPolicyFluentBuilder {
        crate::operation::get_policy::builders::GetPolicyFluentBuilder::new(self.handle.clone())
    }
}
