// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFindingAggregator`](crate::operation::delete_finding_aggregator::builders::DeleteFindingAggregatorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`finding_aggregator_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_finding_aggregator::builders::DeleteFindingAggregatorFluentBuilder::finding_aggregator_arn) / [`set_finding_aggregator_arn(Option<String>)`](crate::operation::delete_finding_aggregator::builders::DeleteFindingAggregatorFluentBuilder::set_finding_aggregator_arn): <p>The ARN of the finding aggregator to delete. To obtain the ARN, use <code>ListFindingAggregators</code>.</p>
    /// - On success, responds with [`DeleteFindingAggregatorOutput`](crate::operation::delete_finding_aggregator::DeleteFindingAggregatorOutput)
    /// - On failure, responds with [`SdkError<DeleteFindingAggregatorError>`](crate::operation::delete_finding_aggregator::DeleteFindingAggregatorError)
    pub fn delete_finding_aggregator(
        &self,
    ) -> crate::operation::delete_finding_aggregator::builders::DeleteFindingAggregatorFluentBuilder
    {
        crate::operation::delete_finding_aggregator::builders::DeleteFindingAggregatorFluentBuilder::new(self.handle.clone())
    }
}
