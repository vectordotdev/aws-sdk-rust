// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateMembers`](crate::operation::create_members::builders::CreateMembersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl ::std::convert::Into<String>)`](crate::operation::create_members::builders::CreateMembersFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::create_members::builders::CreateMembersFluentBuilder::set_detector_id): <p>The unique ID of the detector of the GuardDuty account that you want to associate member accounts with.</p>
    ///   - [`account_details(Vec<AccountDetail>)`](crate::operation::create_members::builders::CreateMembersFluentBuilder::account_details) / [`set_account_details(Option<Vec<AccountDetail>>)`](crate::operation::create_members::builders::CreateMembersFluentBuilder::set_account_details): <p>A list of account ID and email address pairs of the accounts that you want to associate with the GuardDuty administrator account.</p>
    /// - On success, responds with [`CreateMembersOutput`](crate::operation::create_members::CreateMembersOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec<UnprocessedAccount>>)`](crate::operation::create_members::CreateMembersOutput::unprocessed_accounts): <p>A list of objects that include the <code>accountIds</code> of the unprocessed accounts and a result string that explains why each was unprocessed.</p>
    /// - On failure, responds with [`SdkError<CreateMembersError>`](crate::operation::create_members::CreateMembersError)
    pub fn create_members(
        &self,
    ) -> crate::operation::create_members::builders::CreateMembersFluentBuilder {
        crate::operation::create_members::builders::CreateMembersFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
