// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVault`](crate::operation::create_vault::builders::CreateVaultFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::create_vault::builders::CreateVaultFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::create_vault::builders::CreateVaultFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    ///   - [`vault_name(impl ::std::convert::Into<String>)`](crate::operation::create_vault::builders::CreateVaultFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::create_vault::builders::CreateVaultFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    /// - On success, responds with [`CreateVaultOutput`](crate::operation::create_vault::CreateVaultOutput) with field(s):
    ///   - [`location(Option<String>)`](crate::operation::create_vault::CreateVaultOutput::location): <p>The URI of the vault that was created.</p>
    /// - On failure, responds with [`SdkError<CreateVaultError>`](crate::operation::create_vault::CreateVaultError)
    pub fn create_vault(
        &self,
    ) -> crate::operation::create_vault::builders::CreateVaultFluentBuilder {
        crate::operation::create_vault::builders::CreateVaultFluentBuilder::new(self.handle.clone())
    }
}
