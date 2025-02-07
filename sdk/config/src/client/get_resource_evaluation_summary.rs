// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetResourceEvaluationSummary`](crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_evaluation_id(impl ::std::convert::Into<String>)`](crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryFluentBuilder::resource_evaluation_id) / [`set_resource_evaluation_id(Option<String>)`](crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryFluentBuilder::set_resource_evaluation_id): <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
    /// - On success, responds with [`GetResourceEvaluationSummaryOutput`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput) with field(s):
    ///   - [`resource_evaluation_id(Option<String>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::resource_evaluation_id): <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
    ///   - [`evaluation_mode(Option<EvaluationMode>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::evaluation_mode): <p>Lists results of the mode that you requested to retrieve the resource evaluation summary. The valid values are Detective or Proactive.</p>
    ///   - [`evaluation_status(Option<EvaluationStatus>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::evaluation_status): <p>Returns an <code>EvaluationStatus</code> object.</p>
    ///   - [`evaluation_start_timestamp(Option<DateTime>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::evaluation_start_timestamp): <p>The start timestamp when Config rule starts evaluating compliance for the provided resource details.</p>
    ///   - [`compliance(Option<ComplianceType>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::compliance): <p>The compliance status of the resource evaluation summary.</p>
    ///   - [`evaluation_context(Option<EvaluationContext>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::evaluation_context): <p>Returns an <code>EvaluationContext</code> object.</p>
    ///   - [`resource_details(Option<ResourceDetails>)`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput::resource_details): <p>Returns a <code>ResourceDetails</code> object.</p>
    /// - On failure, responds with [`SdkError<GetResourceEvaluationSummaryError>`](crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryError)
    pub fn get_resource_evaluation_summary(&self) -> crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryFluentBuilder{
        crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryFluentBuilder::new(self.handle.clone())
    }
}
