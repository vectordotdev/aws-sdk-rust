// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForVault`](crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    ///   - [`vault_name(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    /// - On success, responds with [`ListTagsForVaultOutput`](crate::operation::list_tags_for_vault::ListTagsForVaultOutput) with field(s):
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::list_tags_for_vault::ListTagsForVaultOutput::tags): <p>The tags attached to the vault. Each tag is composed of a key and a value.</p>
    /// - On failure, responds with [`SdkError<ListTagsForVaultError>`](crate::operation::list_tags_for_vault::ListTagsForVaultError)
    pub fn list_tags_for_vault(
        &self,
    ) -> crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder {
        crate::operation::list_tags_for_vault::builders::ListTagsForVaultFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
