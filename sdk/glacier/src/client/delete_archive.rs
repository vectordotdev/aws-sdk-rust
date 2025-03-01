// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteArchive`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    ///   - [`vault_name(impl ::std::convert::Into<String>)`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    ///   - [`archive_id(impl ::std::convert::Into<String>)`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::archive_id) / [`set_archive_id(Option<String>)`](crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::set_archive_id): <p>The ID of the archive to delete.</p>
    /// - On success, responds with [`DeleteArchiveOutput`](crate::operation::delete_archive::DeleteArchiveOutput)
    /// - On failure, responds with [`SdkError<DeleteArchiveError>`](crate::operation::delete_archive::DeleteArchiveError)
    pub fn delete_archive(
        &self,
    ) -> crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder {
        crate::operation::delete_archive::builders::DeleteArchiveFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
