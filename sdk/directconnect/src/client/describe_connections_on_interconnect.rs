// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeConnectionsOnInterconnect`](crate::operation::describe_connections_on_interconnect::builders::DescribeConnectionsOnInterconnectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`interconnect_id(impl ::std::convert::Into<String>)`](crate::operation::describe_connections_on_interconnect::builders::DescribeConnectionsOnInterconnectFluentBuilder::interconnect_id) / [`set_interconnect_id(Option<String>)`](crate::operation::describe_connections_on_interconnect::builders::DescribeConnectionsOnInterconnectFluentBuilder::set_interconnect_id): <p>The ID of the interconnect.</p>
    /// - On success, responds with [`DescribeConnectionsOnInterconnectOutput`](crate::operation::describe_connections_on_interconnect::DescribeConnectionsOnInterconnectOutput) with field(s):
    ///   - [`connections(Option<Vec<Connection>>)`](crate::operation::describe_connections_on_interconnect::DescribeConnectionsOnInterconnectOutput::connections): <p>The connections.</p>
    /// - On failure, responds with [`SdkError<DescribeConnectionsOnInterconnectError>`](crate::operation::describe_connections_on_interconnect::DescribeConnectionsOnInterconnectError)
    #[deprecated]
    pub fn describe_connections_on_interconnect(&self) -> crate::operation::describe_connections_on_interconnect::builders::DescribeConnectionsOnInterconnectFluentBuilder{
        crate::operation::describe_connections_on_interconnect::builders::DescribeConnectionsOnInterconnectFluentBuilder::new(self.handle.clone())
    }
}
