// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopFlow`](crate::operation::stop_flow::builders::StopFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flow_arn(impl ::std::convert::Into<String>)`](crate::operation::stop_flow::builders::StopFlowFluentBuilder::flow_arn) / [`set_flow_arn(Option<String>)`](crate::operation::stop_flow::builders::StopFlowFluentBuilder::set_flow_arn): The ARN of the flow that you want to stop.
    /// - On success, responds with [`StopFlowOutput`](crate::operation::stop_flow::StopFlowOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::operation::stop_flow::StopFlowOutput::flow_arn): The ARN of the flow that you stopped.
    ///   - [`status(Option<Status>)`](crate::operation::stop_flow::StopFlowOutput::status): The status of the flow when the StopFlow process begins.
    /// - On failure, responds with [`SdkError<StopFlowError>`](crate::operation::stop_flow::StopFlowError)
    pub fn stop_flow(&self) -> crate::operation::stop_flow::builders::StopFlowFluentBuilder {
        crate::operation::stop_flow::builders::StopFlowFluentBuilder::new(self.handle.clone())
    }
}
