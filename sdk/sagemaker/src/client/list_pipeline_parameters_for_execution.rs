// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPipelineParametersForExecution`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_execution_arn(impl ::std::convert::Into<String>)`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::pipeline_execution_arn) / [`set_pipeline_execution_arn(Option<String>)`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::set_pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::set_next_token): <p>If the result of the previous <code>ListPipelineParametersForExecution</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of parameters, use the token in the next request.</p>
    ///   - [`max_results(i32)`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::set_max_results): <p>The maximum number of parameters to return in the response.</p>
    /// - On success, responds with [`ListPipelineParametersForExecutionOutput`](crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionOutput) with field(s):
    ///   - [`pipeline_parameters(Option<Vec<Parameter>>)`](crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionOutput::pipeline_parameters): <p>Contains a list of pipeline parameters. This list can be empty. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionOutput::next_token): <p>If the result of the previous <code>ListPipelineParametersForExecution</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of parameters, use the token in the next request.</p>
    /// - On failure, responds with [`SdkError<ListPipelineParametersForExecutionError>`](crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionError)
    pub fn list_pipeline_parameters_for_execution(&self) -> crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder{
        crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionFluentBuilder::new(self.handle.clone())
    }
}
