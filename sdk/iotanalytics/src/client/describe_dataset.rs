// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDataset`](crate::operation::describe_dataset::builders::DescribeDatasetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dataset_name(impl ::std::convert::Into<String>)`](crate::operation::describe_dataset::builders::DescribeDatasetFluentBuilder::dataset_name) / [`set_dataset_name(Option<String>)`](crate::operation::describe_dataset::builders::DescribeDatasetFluentBuilder::set_dataset_name): <p>The name of the dataset whose information is retrieved.</p>
    /// - On success, responds with [`DescribeDatasetOutput`](crate::operation::describe_dataset::DescribeDatasetOutput) with field(s):
    ///   - [`dataset(Option<Dataset>)`](crate::operation::describe_dataset::DescribeDatasetOutput::dataset): <p>An object that contains information about the dataset.</p>
    /// - On failure, responds with [`SdkError<DescribeDatasetError>`](crate::operation::describe_dataset::DescribeDatasetError)
    pub fn describe_dataset(
        &self,
    ) -> crate::operation::describe_dataset::builders::DescribeDatasetFluentBuilder {
        crate::operation::describe_dataset::builders::DescribeDatasetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
