// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDataSetImportTask`](crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder::set_application_id): <p>The application identifier.</p>
    ///   - [`task_id(impl ::std::convert::Into<String>)`](crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder::task_id) / [`set_task_id(Option<String>)`](crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder::set_task_id): <p>The task identifier returned by the <code>CreateDataSetImportTask</code> operation. </p>
    /// - On success, responds with [`GetDataSetImportTaskOutput`](crate::operation::get_data_set_import_task::GetDataSetImportTaskOutput) with field(s):
    ///   - [`task_id(Option<String>)`](crate::operation::get_data_set_import_task::GetDataSetImportTaskOutput::task_id): <p>The task identifier.</p>
    ///   - [`status(Option<DataSetTaskLifecycle>)`](crate::operation::get_data_set_import_task::GetDataSetImportTaskOutput::status): <p>The status of the task.</p>
    ///   - [`summary(Option<DataSetImportSummary>)`](crate::operation::get_data_set_import_task::GetDataSetImportTaskOutput::summary): <p>A summary of the status of the task.</p>
    /// - On failure, responds with [`SdkError<GetDataSetImportTaskError>`](crate::operation::get_data_set_import_task::GetDataSetImportTaskError)
    pub fn get_data_set_import_task(
        &self,
    ) -> crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder
    {
        crate::operation::get_data_set_import_task::builders::GetDataSetImportTaskFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
