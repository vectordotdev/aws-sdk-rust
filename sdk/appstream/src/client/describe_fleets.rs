// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFleets`](crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`names(Vec<String>)`](crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder::names) / [`set_names(Option<Vec<String>>)`](crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder::set_names): <p>The names of the fleets to describe.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder::set_next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    /// - On success, responds with [`DescribeFleetsOutput`](crate::operation::describe_fleets::DescribeFleetsOutput) with field(s):
    ///   - [`fleets(Option<Vec<Fleet>>)`](crate::operation::describe_fleets::DescribeFleetsOutput::fleets): <p>Information about the fleets.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_fleets::DescribeFleetsOutput::next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    /// - On failure, responds with [`SdkError<DescribeFleetsError>`](crate::operation::describe_fleets::DescribeFleetsError)
    pub fn describe_fleets(
        &self,
    ) -> crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder {
        crate::operation::describe_fleets::builders::DescribeFleetsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
