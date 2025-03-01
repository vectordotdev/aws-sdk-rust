// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePreparedStatement`](crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`statement_name(impl ::std::convert::Into<String>)`](crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder::statement_name) / [`set_statement_name(Option<String>)`](crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder::set_statement_name): <p>The name of the prepared statement to delete.</p>
    ///   - [`work_group(impl ::std::convert::Into<String>)`](crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder::work_group) / [`set_work_group(Option<String>)`](crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder::set_work_group): <p>The workgroup to which the statement to be deleted belongs.</p>
    /// - On success, responds with [`DeletePreparedStatementOutput`](crate::operation::delete_prepared_statement::DeletePreparedStatementOutput)
    /// - On failure, responds with [`SdkError<DeletePreparedStatementError>`](crate::operation::delete_prepared_statement::DeletePreparedStatementError)
    pub fn delete_prepared_statement(
        &self,
    ) -> crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder
    {
        crate::operation::delete_prepared_statement::builders::DeletePreparedStatementFluentBuilder::new(self.handle.clone())
    }
}
