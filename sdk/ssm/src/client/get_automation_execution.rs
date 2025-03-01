// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAutomationExecution`](crate::operation::get_automation_execution::builders::GetAutomationExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`automation_execution_id(impl ::std::convert::Into<String>)`](crate::operation::get_automation_execution::builders::GetAutomationExecutionFluentBuilder::automation_execution_id) / [`set_automation_execution_id(Option<String>)`](crate::operation::get_automation_execution::builders::GetAutomationExecutionFluentBuilder::set_automation_execution_id): <p>The unique identifier for an existing automation execution to examine. The execution ID is returned by StartAutomationExecution when the execution of an Automation runbook is initiated.</p>
    /// - On success, responds with [`GetAutomationExecutionOutput`](crate::operation::get_automation_execution::GetAutomationExecutionOutput) with field(s):
    ///   - [`automation_execution(Option<AutomationExecution>)`](crate::operation::get_automation_execution::GetAutomationExecutionOutput::automation_execution): <p>Detailed information about the current state of an automation execution.</p>
    /// - On failure, responds with [`SdkError<GetAutomationExecutionError>`](crate::operation::get_automation_execution::GetAutomationExecutionError)
    pub fn get_automation_execution(
        &self,
    ) -> crate::operation::get_automation_execution::builders::GetAutomationExecutionFluentBuilder
    {
        crate::operation::get_automation_execution::builders::GetAutomationExecutionFluentBuilder::new(self.handle.clone())
    }
}
