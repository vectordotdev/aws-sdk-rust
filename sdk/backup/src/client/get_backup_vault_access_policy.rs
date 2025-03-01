// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBackupVaultAccessPolicy`](crate::operation::get_backup_vault_access_policy::builders::GetBackupVaultAccessPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_vault_name(impl ::std::convert::Into<String>)`](crate::operation::get_backup_vault_access_policy::builders::GetBackupVaultAccessPolicyFluentBuilder::backup_vault_name) / [`set_backup_vault_name(Option<String>)`](crate::operation::get_backup_vault_access_policy::builders::GetBackupVaultAccessPolicyFluentBuilder::set_backup_vault_name): <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    /// - On success, responds with [`GetBackupVaultAccessPolicyOutput`](crate::operation::get_backup_vault_access_policy::GetBackupVaultAccessPolicyOutput) with field(s):
    ///   - [`backup_vault_name(Option<String>)`](crate::operation::get_backup_vault_access_policy::GetBackupVaultAccessPolicyOutput::backup_vault_name): <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    ///   - [`backup_vault_arn(Option<String>)`](crate::operation::get_backup_vault_access_policy::GetBackupVaultAccessPolicyOutput::backup_vault_arn): <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    ///   - [`policy(Option<String>)`](crate::operation::get_backup_vault_access_policy::GetBackupVaultAccessPolicyOutput::policy): <p>The backup vault access policy document in JSON format.</p>
    /// - On failure, responds with [`SdkError<GetBackupVaultAccessPolicyError>`](crate::operation::get_backup_vault_access_policy::GetBackupVaultAccessPolicyError)
    pub fn get_backup_vault_access_policy(&self) -> crate::operation::get_backup_vault_access_policy::builders::GetBackupVaultAccessPolicyFluentBuilder{
        crate::operation::get_backup_vault_access_policy::builders::GetBackupVaultAccessPolicyFluentBuilder::new(self.handle.clone())
    }
}
