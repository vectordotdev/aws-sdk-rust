// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListUsers`](crate::operation::list_users::builders::ListUsersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_users::builders::ListUsersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_users::builders::ListUsersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_users::builders::ListUsersFluentBuilder::set_max_results): <p>Specifies the number of users to return as a response to the <code>ListUsers</code> request.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_users::builders::ListUsersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_users::builders::ListUsersFluentBuilder::set_next_token): <p>When you can get additional results from the <code>ListUsers</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional users.</p>
    ///   - [`server_id(impl ::std::convert::Into<String>)`](crate::operation::list_users::builders::ListUsersFluentBuilder::server_id) / [`set_server_id(Option<String>)`](crate::operation::list_users::builders::ListUsersFluentBuilder::set_server_id): <p>A system-assigned unique identifier for a server that has users assigned to it.</p>
    /// - On success, responds with [`ListUsersOutput`](crate::operation::list_users::ListUsersOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_users::ListUsersOutput::next_token): <p>When you can get additional results from the <code>ListUsers</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional users.</p>
    ///   - [`server_id(Option<String>)`](crate::operation::list_users::ListUsersOutput::server_id): <p>A system-assigned unique identifier for a server that the users are assigned to.</p>
    ///   - [`users(Option<Vec<ListedUser>>)`](crate::operation::list_users::ListUsersOutput::users): <p>Returns the Transfer Family users and their properties for the <code>ServerId</code> value that you specify.</p>
    /// - On failure, responds with [`SdkError<ListUsersError>`](crate::operation::list_users::ListUsersError)
    pub fn list_users(&self) -> crate::operation::list_users::builders::ListUsersFluentBuilder {
        crate::operation::list_users::builders::ListUsersFluentBuilder::new(self.handle.clone())
    }
}
