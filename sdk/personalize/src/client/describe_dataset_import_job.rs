// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDatasetImportJob`](crate::operation::describe_dataset_import_job::builders::DescribeDatasetImportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dataset_import_job_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_dataset_import_job::builders::DescribeDatasetImportJobFluentBuilder::dataset_import_job_arn) / [`set_dataset_import_job_arn(Option<String>)`](crate::operation::describe_dataset_import_job::builders::DescribeDatasetImportJobFluentBuilder::set_dataset_import_job_arn): <p>The Amazon Resource Name (ARN) of the dataset import job to describe.</p>
    /// - On success, responds with [`DescribeDatasetImportJobOutput`](crate::operation::describe_dataset_import_job::DescribeDatasetImportJobOutput) with field(s):
    ///   - [`dataset_import_job(Option<DatasetImportJob>)`](crate::operation::describe_dataset_import_job::DescribeDatasetImportJobOutput::dataset_import_job): <p>Information about the dataset import job, including the status.</p>  <p>The status is one of the following values:</p>  <ul>   <li> <p>CREATE PENDING</p> </li>   <li> <p>CREATE IN_PROGRESS</p> </li>   <li> <p>ACTIVE</p> </li>   <li> <p>CREATE FAILED</p> </li>  </ul>
    /// - On failure, responds with [`SdkError<DescribeDatasetImportJobError>`](crate::operation::describe_dataset_import_job::DescribeDatasetImportJobError)
    pub fn describe_dataset_import_job(&self) -> crate::operation::describe_dataset_import_job::builders::DescribeDatasetImportJobFluentBuilder{
        crate::operation::describe_dataset_import_job::builders::DescribeDatasetImportJobFluentBuilder::new(self.handle.clone())
    }
}
