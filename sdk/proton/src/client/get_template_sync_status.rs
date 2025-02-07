// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTemplateSyncStatus`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_name(impl ::std::convert::Into<String>)`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::template_name) / [`set_template_name(Option<String>)`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::set_template_name): <p>The template name.</p>
    ///   - [`template_type(TemplateType)`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::template_type) / [`set_template_type(Option<TemplateType>)`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::set_template_type): <p>The template type.</p>
    ///   - [`template_version(impl ::std::convert::Into<String>)`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::template_version) / [`set_template_version(Option<String>)`](crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::set_template_version): <p>The template major version.</p>
    /// - On success, responds with [`GetTemplateSyncStatusOutput`](crate::operation::get_template_sync_status::GetTemplateSyncStatusOutput) with field(s):
    ///   - [`latest_sync(Option<ResourceSyncAttempt>)`](crate::operation::get_template_sync_status::GetTemplateSyncStatusOutput::latest_sync): <p>The details of the last sync that's returned by Proton.</p>
    ///   - [`latest_successful_sync(Option<ResourceSyncAttempt>)`](crate::operation::get_template_sync_status::GetTemplateSyncStatusOutput::latest_successful_sync): <p>The details of the last successful sync that's returned by Proton.</p>
    ///   - [`desired_state(Option<Revision>)`](crate::operation::get_template_sync_status::GetTemplateSyncStatusOutput::desired_state): <p>The template sync desired state that's returned by Proton.</p>
    /// - On failure, responds with [`SdkError<GetTemplateSyncStatusError>`](crate::operation::get_template_sync_status::GetTemplateSyncStatusError)
    pub fn get_template_sync_status(
        &self,
    ) -> crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder
    {
        crate::operation::get_template_sync_status::builders::GetTemplateSyncStatusFluentBuilder::new(self.handle.clone())
    }
}
