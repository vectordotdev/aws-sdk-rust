// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopRun`](crate::operation::stop_run::builders::StopRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::stop_run::builders::StopRunFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::stop_run::builders::StopRunFluentBuilder::set_arn): <p>Represents the Amazon Resource Name (ARN) of the Device Farm run to stop.</p>
    /// - On success, responds with [`StopRunOutput`](crate::operation::stop_run::StopRunOutput) with field(s):
    ///   - [`run(Option<Run>)`](crate::operation::stop_run::StopRunOutput::run): <p>The run that was stopped.</p>
    /// - On failure, responds with [`SdkError<StopRunError>`](crate::operation::stop_run::StopRunError)
    pub fn stop_run(&self) -> crate::operation::stop_run::builders::StopRunFluentBuilder {
        crate::operation::stop_run::builders::StopRunFluentBuilder::new(self.handle.clone())
    }
}
