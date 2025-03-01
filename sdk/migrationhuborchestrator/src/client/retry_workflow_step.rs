// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RetryWorkflowStep`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workflow_id(impl ::std::convert::Into<String>)`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::workflow_id) / [`set_workflow_id(Option<String>)`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::set_workflow_id): <p>The ID of the migration workflow.</p>
    ///   - [`step_group_id(impl ::std::convert::Into<String>)`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::step_group_id) / [`set_step_group_id(Option<String>)`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::set_step_group_id): <p>The ID of the step group.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::set_id): <p>The ID of the step.</p>
    /// - On success, responds with [`RetryWorkflowStepOutput`](crate::operation::retry_workflow_step::RetryWorkflowStepOutput) with field(s):
    ///   - [`step_group_id(Option<String>)`](crate::operation::retry_workflow_step::RetryWorkflowStepOutput::step_group_id): <p>The ID of the step group.</p>
    ///   - [`workflow_id(Option<String>)`](crate::operation::retry_workflow_step::RetryWorkflowStepOutput::workflow_id): <p>The ID of the migration workflow.</p>
    ///   - [`id(Option<String>)`](crate::operation::retry_workflow_step::RetryWorkflowStepOutput::id): <p>The ID of the step.</p>
    ///   - [`status(Option<StepStatus>)`](crate::operation::retry_workflow_step::RetryWorkflowStepOutput::status): <p>The status of the step.</p>
    /// - On failure, responds with [`SdkError<RetryWorkflowStepError>`](crate::operation::retry_workflow_step::RetryWorkflowStepError)
    pub fn retry_workflow_step(
        &self,
    ) -> crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder {
        crate::operation::retry_workflow_step::builders::RetryWorkflowStepFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
