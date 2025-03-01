// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListOrganizationAdminAccounts`](crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder::set_next_token): <p>For requests to get the next page of results, the pagination token that was returned with the previous set of results. The initial request does not include a pagination token.</p>
    ///   - [`max_results(i32)`](crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder::set_max_results): <p>The maximum number of results to return.</p>
    /// - On success, responds with [`ListOrganizationAdminAccountsOutput`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput) with field(s):
    ///   - [`administrators(Option<Vec<Administrator>>)`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput::administrators): <p>The list of Detective administrator accounts.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput::next_token): <p>If there are more accounts remaining in the results, then this is the pagination token to use to request the next page of accounts.</p>
    /// - On failure, responds with [`SdkError<ListOrganizationAdminAccountsError>`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsError)
    pub fn list_organization_admin_accounts(&self) -> crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder{
        crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsFluentBuilder::new(self.handle.clone())
    }
}
