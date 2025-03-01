// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTable`](crate::operation::delete_table::builders::DeleteTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`keyspace_name(impl ::std::convert::Into<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::keyspace_name) / [`set_keyspace_name(Option<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_keyspace_name): <p>The name of the keyspace of the to be deleted table.</p>
    ///   - [`table_name(impl ::std::convert::Into<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_table_name): <p>The name of the table to be deleted.</p>
    /// - On success, responds with [`DeleteTableOutput`](crate::operation::delete_table::DeleteTableOutput)
    /// - On failure, responds with [`SdkError<DeleteTableError>`](crate::operation::delete_table::DeleteTableError)
    pub fn delete_table(
        &self,
    ) -> crate::operation::delete_table::builders::DeleteTableFluentBuilder {
        crate::operation::delete_table::builders::DeleteTableFluentBuilder::new(self.handle.clone())
    }
}
