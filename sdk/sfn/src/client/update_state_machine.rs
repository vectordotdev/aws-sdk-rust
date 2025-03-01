// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateStateMachine`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`state_machine_arn(impl ::std::convert::Into<String>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::state_machine_arn) / [`set_state_machine_arn(Option<String>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::set_state_machine_arn): <p>The Amazon Resource Name (ARN) of the state machine.</p>
    ///   - [`definition(impl ::std::convert::Into<String>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::definition) / [`set_definition(Option<String>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::set_definition): <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    ///   - [`role_arn(impl ::std::convert::Into<String>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::set_role_arn): <p>The Amazon Resource Name (ARN) of the IAM role of the state machine.</p>
    ///   - [`logging_configuration(LoggingConfiguration)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::logging_configuration) / [`set_logging_configuration(Option<LoggingConfiguration>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::set_logging_configuration): <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    ///   - [`tracing_configuration(TracingConfiguration)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::tracing_configuration) / [`set_tracing_configuration(Option<TracingConfiguration>)`](crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::set_tracing_configuration): <p>Selects whether X-Ray tracing is enabled.</p>
    /// - On success, responds with [`UpdateStateMachineOutput`](crate::operation::update_state_machine::UpdateStateMachineOutput) with field(s):
    ///   - [`update_date(Option<DateTime>)`](crate::operation::update_state_machine::UpdateStateMachineOutput::update_date): <p>The date and time the state machine was updated.</p>
    /// - On failure, responds with [`SdkError<UpdateStateMachineError>`](crate::operation::update_state_machine::UpdateStateMachineError)
    pub fn update_state_machine(
        &self,
    ) -> crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder {
        crate::operation::update_state_machine::builders::UpdateStateMachineFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
