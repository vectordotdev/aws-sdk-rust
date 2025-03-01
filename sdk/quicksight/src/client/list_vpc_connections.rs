// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListVPCConnections`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::set_aws_account_id): <p>The Amazon Web Services account ID of the account that contains the VPC connections that you want to list.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::set_max_results): <p>The maximum number of results to be returned per request.</p>
    /// - On success, responds with [`ListVpcConnectionsOutput`](crate::operation::list_vpc_connections::ListVpcConnectionsOutput) with field(s):
    ///   - [`vpc_connection_summaries(Option<Vec<VpcConnectionSummary>>)`](crate::operation::list_vpc_connections::ListVpcConnectionsOutput::vpc_connection_summaries): <p>A <code>VPCConnectionSummaries</code> object that returns a summary of VPC connection objects.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_vpc_connections::ListVpcConnectionsOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::list_vpc_connections::ListVpcConnectionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::list_vpc_connections::ListVpcConnectionsOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<ListVPCConnectionsError>`](crate::operation::list_vpc_connections::ListVPCConnectionsError)
    pub fn list_vpc_connections(
        &self,
    ) -> crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder {
        crate::operation::list_vpc_connections::builders::ListVPCConnectionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
