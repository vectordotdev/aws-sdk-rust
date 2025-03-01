// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetParameterHistory`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::set_name): <p>The name of the parameter for which you want to review history.</p>
    ///   - [`with_decryption(bool)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::with_decryption) / [`set_with_decryption(Option<bool>)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::set_with_decryption): <p>Return decrypted values for secure string parameters. This flag is ignored for <code>String</code> and <code>StringList</code> parameter types.</p>
    ///   - [`max_results(i32)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::set_next_token): <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    /// - On success, responds with [`GetParameterHistoryOutput`](crate::operation::get_parameter_history::GetParameterHistoryOutput) with field(s):
    ///   - [`parameters(Option<Vec<ParameterHistory>>)`](crate::operation::get_parameter_history::GetParameterHistoryOutput::parameters): <p>A list of parameters returned by the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_parameter_history::GetParameterHistoryOutput::next_token): <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    /// - On failure, responds with [`SdkError<GetParameterHistoryError>`](crate::operation::get_parameter_history::GetParameterHistoryError)
    pub fn get_parameter_history(
        &self,
    ) -> crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder {
        crate::operation::get_parameter_history::builders::GetParameterHistoryFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
