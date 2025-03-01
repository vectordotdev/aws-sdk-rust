// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTableDataImportJob`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workbook_id(impl ::std::convert::Into<String>)`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::workbook_id) / [`set_workbook_id(Option<String>)`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::set_workbook_id): <p>The ID of the workbook into which data was imported.</p>  <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    ///   - [`table_id(impl ::std::convert::Into<String>)`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::table_id) / [`set_table_id(Option<String>)`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::set_table_id): <p>The ID of the table into which data was imported.</p>  <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    ///   - [`job_id(impl ::std::convert::Into<String>)`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::set_job_id): <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>  <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    /// - On success, responds with [`DescribeTableDataImportJobOutput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput) with field(s):
    ///   - [`job_status(Option<TableDataImportJobStatus>)`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput::job_status): <p> The current status of the import job. </p>
    ///   - [`message(Option<String>)`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput::message): <p> A message providing more details about the current status of the import job. </p>
    ///   - [`job_metadata(Option<TableDataImportJobMetadata>)`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput::job_metadata): <p> The metadata about the job that was submitted for import. </p>
    ///   - [`error_code(Option<ErrorCode>)`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput::error_code): <p> If job status is failed, error code to understand reason for the failure. </p>
    /// - On failure, responds with [`SdkError<DescribeTableDataImportJobError>`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobError)
    pub fn describe_table_data_import_job(&self) -> crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder{
        crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobFluentBuilder::new(self.handle.clone())
    }
}
