// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCodeSigningConfig`](crate::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`code_signing_config_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigFluentBuilder::code_signing_config_arn) / [`set_code_signing_config_arn(Option<String>)`](crate::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigFluentBuilder::set_code_signing_config_arn): <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    /// - On success, responds with [`DeleteCodeSigningConfigOutput`](crate::operation::delete_code_signing_config::DeleteCodeSigningConfigOutput)
    /// - On failure, responds with [`SdkError<DeleteCodeSigningConfigError>`](crate::operation::delete_code_signing_config::DeleteCodeSigningConfigError)
    pub fn delete_code_signing_config(
        &self,
    ) -> crate::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigFluentBuilder
    {
        crate::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigFluentBuilder::new(self.handle.clone())
    }
}
