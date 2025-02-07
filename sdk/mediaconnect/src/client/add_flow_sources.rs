// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddFlowSources`](crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flow_arn(impl ::std::convert::Into<String>)`](crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder::flow_arn) / [`set_flow_arn(Option<String>)`](crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder::set_flow_arn): The flow that you want to mutate.
    ///   - [`sources(Vec<SetSourceRequest>)`](crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder::sources) / [`set_sources(Option<Vec<SetSourceRequest>>)`](crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder::set_sources): A list of sources that you want to add.
    /// - On success, responds with [`AddFlowSourcesOutput`](crate::operation::add_flow_sources::AddFlowSourcesOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::operation::add_flow_sources::AddFlowSourcesOutput::flow_arn): The ARN of the flow that these sources were added to.
    ///   - [`sources(Option<Vec<Source>>)`](crate::operation::add_flow_sources::AddFlowSourcesOutput::sources): The details of the newly added sources.
    /// - On failure, responds with [`SdkError<AddFlowSourcesError>`](crate::operation::add_flow_sources::AddFlowSourcesError)
    pub fn add_flow_sources(
        &self,
    ) -> crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder {
        crate::operation::add_flow_sources::builders::AddFlowSourcesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
