// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListBackupPlanTemplates`](crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder::set_next_token): <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    ///   - [`max_results(i32)`](crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder::set_max_results): <p>The maximum number of items to be returned.</p>
    /// - On success, responds with [`ListBackupPlanTemplatesOutput`](crate::operation::list_backup_plan_templates::ListBackupPlanTemplatesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_backup_plan_templates::ListBackupPlanTemplatesOutput::next_token): <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    ///   - [`backup_plan_templates_list(Option<Vec<BackupPlanTemplatesListMember>>)`](crate::operation::list_backup_plan_templates::ListBackupPlanTemplatesOutput::backup_plan_templates_list): <p>An array of template list items containing metadata about your saved templates.</p>
    /// - On failure, responds with [`SdkError<ListBackupPlanTemplatesError>`](crate::operation::list_backup_plan_templates::ListBackupPlanTemplatesError)
    pub fn list_backup_plan_templates(
        &self,
    ) -> crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder
    {
        crate::operation::list_backup_plan_templates::builders::ListBackupPlanTemplatesFluentBuilder::new(self.handle.clone())
    }
}
