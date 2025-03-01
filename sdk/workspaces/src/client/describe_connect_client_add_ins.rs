// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeConnectClientAddIns`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::set_resource_id): <p>The directory identifier for which the client add-in is configured.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::set_next_token): <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::set_max_results): <p>The maximum number of items to return.</p>
    /// - On success, responds with [`DescribeConnectClientAddInsOutput`](crate::operation::describe_connect_client_add_ins::DescribeConnectClientAddInsOutput) with field(s):
    ///   - [`add_ins(Option<Vec<ConnectClientAddIn>>)`](crate::operation::describe_connect_client_add_ins::DescribeConnectClientAddInsOutput::add_ins): <p>Information about client add-ins.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_connect_client_add_ins::DescribeConnectClientAddInsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
    /// - On failure, responds with [`SdkError<DescribeConnectClientAddInsError>`](crate::operation::describe_connect_client_add_ins::DescribeConnectClientAddInsError)
    pub fn describe_connect_client_add_ins(&self) -> crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder{
        crate::operation::describe_connect_client_add_ins::builders::DescribeConnectClientAddInsFluentBuilder::new(self.handle.clone())
    }
}
