// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteConnection`](crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl ::std::convert::Into<String>)`](crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder::global_network_id) / [`set_global_network_id(Option<String>)`](crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`connection_id(impl ::std::convert::Into<String>)`](crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder::connection_id) / [`set_connection_id(Option<String>)`](crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder::set_connection_id): <p>The ID of the connection.</p>
    /// - On success, responds with [`DeleteConnectionOutput`](crate::operation::delete_connection::DeleteConnectionOutput) with field(s):
    ///   - [`connection(Option<Connection>)`](crate::operation::delete_connection::DeleteConnectionOutput::connection): <p>Information about the connection.</p>
    /// - On failure, responds with [`SdkError<DeleteConnectionError>`](crate::operation::delete_connection::DeleteConnectionError)
    pub fn delete_connection(
        &self,
    ) -> crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder {
        crate::operation::delete_connection::builders::DeleteConnectionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
