// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopWorkflowRun`](crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder::set_name): <p>The name of the workflow to stop.</p>
    ///   - [`run_id(impl ::std::convert::Into<String>)`](crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder::run_id) / [`set_run_id(Option<String>)`](crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder::set_run_id): <p>The ID of the workflow run to stop.</p>
    /// - On success, responds with [`StopWorkflowRunOutput`](crate::operation::stop_workflow_run::StopWorkflowRunOutput)
    /// - On failure, responds with [`SdkError<StopWorkflowRunError>`](crate::operation::stop_workflow_run::StopWorkflowRunError)
    pub fn stop_workflow_run(
        &self,
    ) -> crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder {
        crate::operation::stop_workflow_run::builders::StopWorkflowRunFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
