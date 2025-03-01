// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConnection`](crate::operation::get_connection::builders::GetConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connection_id(impl ::std::convert::Into<String>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::connection_id) / [`set_connection_id(Option<String>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::set_connection_id): (undocumented)
    /// - On success, responds with [`GetConnectionOutput`](crate::operation::get_connection::GetConnectionOutput) with field(s):
    ///   - [`connected_at(Option<DateTime>)`](crate::operation::get_connection::GetConnectionOutput::connected_at): <p>The time in ISO 8601 format for when the connection was established.</p>
    ///   - [`identity(Option<Identity>)`](crate::operation::get_connection::GetConnectionOutput::identity): (undocumented)
    ///   - [`last_active_at(Option<DateTime>)`](crate::operation::get_connection::GetConnectionOutput::last_active_at): <p>The time in ISO 8601 format for when the connection was last active.</p>
    /// - On failure, responds with [`SdkError<GetConnectionError>`](crate::operation::get_connection::GetConnectionError)
    pub fn get_connection(
        &self,
    ) -> crate::operation::get_connection::builders::GetConnectionFluentBuilder {
        crate::operation::get_connection::builders::GetConnectionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
