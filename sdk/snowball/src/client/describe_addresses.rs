// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAddresses`](crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder::set_max_results): <p>The number of <code>ADDRESS</code> objects to return.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder::set_next_token): <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>ADDRESS</code> objects, you have the option of specifying a value for <code>NextToken</code> as the starting point for your list of returned addresses.</p>
    /// - On success, responds with [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput) with field(s):
    ///   - [`addresses(Option<Vec<Address>>)`](crate::operation::describe_addresses::DescribeAddressesOutput::addresses): <p>The Snow device shipping addresses that were created for this account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_addresses::DescribeAddressesOutput::next_token): <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
    /// - On failure, responds with [`SdkError<DescribeAddressesError>`](crate::operation::describe_addresses::DescribeAddressesError)
    pub fn describe_addresses(
        &self,
    ) -> crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder {
        crate::operation::describe_addresses::builders::DescribeAddressesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
