// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConnections`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl ::std::convert::Into<String>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::set_catalog_id): <p>The ID of the Data Catalog in which the connections reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`filter(GetConnectionsFilter)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::filter) / [`set_filter(Option<GetConnectionsFilter>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::set_filter): <p>A filter that controls which connections are returned.</p>
    ///   - [`hide_password(bool)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::hide_password) / [`set_hide_password(Option<bool>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::set_hide_password): <p>Allows you to retrieve the connection metadata without returning the password. For instance, the Glue console uses this flag to retrieve the connection, and does not display the password. Set this parameter when the caller might not have permission to use the KMS key to decrypt the password, but it does have permission to access the rest of the connection properties.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::set_next_token): <p>A continuation token, if this is a continuation call.</p>
    ///   - [`max_results(i32)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_connections::builders::GetConnectionsFluentBuilder::set_max_results): <p>The maximum number of connections to return in one response.</p>
    /// - On success, responds with [`GetConnectionsOutput`](crate::operation::get_connections::GetConnectionsOutput) with field(s):
    ///   - [`connection_list(Option<Vec<Connection>>)`](crate::operation::get_connections::GetConnectionsOutput::connection_list): <p>A list of requested connection definitions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_connections::GetConnectionsOutput::next_token): <p>A continuation token, if the list of connections returned does not include the last of the filtered connections.</p>
    /// - On failure, responds with [`SdkError<GetConnectionsError>`](crate::operation::get_connections::GetConnectionsError)
    pub fn get_connections(
        &self,
    ) -> crate::operation::get_connections::builders::GetConnectionsFluentBuilder {
        crate::operation::get_connections::builders::GetConnectionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
