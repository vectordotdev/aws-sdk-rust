// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeStep`](crate::operation::describe_step::builders::DescribeStepFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl ::std::convert::Into<String>)`](crate::operation::describe_step::builders::DescribeStepFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::describe_step::builders::DescribeStepFluentBuilder::set_cluster_id): <p>The identifier of the cluster with steps to describe.</p>
    ///   - [`step_id(impl ::std::convert::Into<String>)`](crate::operation::describe_step::builders::DescribeStepFluentBuilder::step_id) / [`set_step_id(Option<String>)`](crate::operation::describe_step::builders::DescribeStepFluentBuilder::set_step_id): <p>The identifier of the step to describe.</p>
    /// - On success, responds with [`DescribeStepOutput`](crate::operation::describe_step::DescribeStepOutput) with field(s):
    ///   - [`step(Option<Step>)`](crate::operation::describe_step::DescribeStepOutput::step): <p>The step details for the requested step identifier.</p>
    /// - On failure, responds with [`SdkError<DescribeStepError>`](crate::operation::describe_step::DescribeStepError)
    pub fn describe_step(
        &self,
    ) -> crate::operation::describe_step::builders::DescribeStepFluentBuilder {
        crate::operation::describe_step::builders::DescribeStepFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
