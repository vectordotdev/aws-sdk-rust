// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPreparedStatement`](crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`statement_name(impl ::std::convert::Into<String>)`](crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder::statement_name) / [`set_statement_name(Option<String>)`](crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder::set_statement_name): <p>The name of the prepared statement to retrieve.</p>
    ///   - [`work_group(impl ::std::convert::Into<String>)`](crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder::work_group) / [`set_work_group(Option<String>)`](crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder::set_work_group): <p>The workgroup to which the statement to be retrieved belongs.</p>
    /// - On success, responds with [`GetPreparedStatementOutput`](crate::operation::get_prepared_statement::GetPreparedStatementOutput) with field(s):
    ///   - [`prepared_statement(Option<PreparedStatement>)`](crate::operation::get_prepared_statement::GetPreparedStatementOutput::prepared_statement): <p>The name of the prepared statement that was retrieved.</p>
    /// - On failure, responds with [`SdkError<GetPreparedStatementError>`](crate::operation::get_prepared_statement::GetPreparedStatementError)
    pub fn get_prepared_statement(
        &self,
    ) -> crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder {
        crate::operation::get_prepared_statement::builders::GetPreparedStatementFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
