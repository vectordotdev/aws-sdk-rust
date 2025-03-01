// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartPipelineExecution`](crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder::set_name): <p>The name of the pipeline to start.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder::set_client_request_token): <p>The system-generated unique ID used to identify a unique execution request.</p>
    /// - On success, responds with [`StartPipelineExecutionOutput`](crate::operation::start_pipeline_execution::StartPipelineExecutionOutput) with field(s):
    ///   - [`pipeline_execution_id(Option<String>)`](crate::operation::start_pipeline_execution::StartPipelineExecutionOutput::pipeline_execution_id): <p>The unique system-generated ID of the pipeline execution that was started.</p>
    /// - On failure, responds with [`SdkError<StartPipelineExecutionError>`](crate::operation::start_pipeline_execution::StartPipelineExecutionError)
    pub fn start_pipeline_execution(
        &self,
    ) -> crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder
    {
        crate::operation::start_pipeline_execution::builders::StartPipelineExecutionFluentBuilder::new(self.handle.clone())
    }
}
