// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBatchJobExecution`](crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder::set_application_id): <p>The identifier of the application.</p>
    ///   - [`execution_id(impl ::std::convert::Into<String>)`](crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder::execution_id) / [`set_execution_id(Option<String>)`](crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder::set_execution_id): <p>The unique identifier of the batch job execution.</p>
    /// - On success, responds with [`GetBatchJobExecutionOutput`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput) with field(s):
    ///   - [`execution_id(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::execution_id): <p>The unique identifier for this batch job execution.</p>
    ///   - [`application_id(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::application_id): <p>The identifier of the application.</p>
    ///   - [`job_id(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::job_id): <p>The unique identifier for this batch job.</p>
    ///   - [`job_name(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::job_name): <p>The name of this batch job.</p>
    ///   - [`job_user(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::job_user): <p>The user for the job.</p>
    ///   - [`job_type(Option<BatchJobType>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::job_type): <p>The type of job.</p>
    ///   - [`status(Option<BatchJobExecutionStatus>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::status): <p>The status of the batch job execution.</p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::start_time): <p>The timestamp when the batch job execution started.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::end_time): <p>The timestamp when the batch job execution ended.</p>
    ///   - [`status_reason(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::status_reason): <p>The reason for the reported status.</p>
    ///   - [`return_code(Option<String>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::return_code): <p></p>
    ///   - [`batch_job_identifier(Option<BatchJobIdentifier>)`](crate::operation::get_batch_job_execution::GetBatchJobExecutionOutput::batch_job_identifier): <p>Identifies a specific batch job.</p>
    /// - On failure, responds with [`SdkError<GetBatchJobExecutionError>`](crate::operation::get_batch_job_execution::GetBatchJobExecutionError)
    pub fn get_batch_job_execution(
        &self,
    ) -> crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder
    {
        crate::operation::get_batch_job_execution::builders::GetBatchJobExecutionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
