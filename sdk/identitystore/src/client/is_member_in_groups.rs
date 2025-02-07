// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`IsMemberInGroups`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl ::std::convert::Into<String>)`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::set_identity_store_id): <p>The globally unique identifier for the identity store.</p>
    ///   - [`member_id(MemberId)`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::member_id) / [`set_member_id(Option<MemberId>)`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::set_member_id): <p>An object containing the identifier of a group member.</p>
    ///   - [`group_ids(Vec<String>)`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::group_ids) / [`set_group_ids(Option<Vec<String>>)`](crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::set_group_ids): <p>A list of identifiers for groups in the identity store.</p>
    /// - On success, responds with [`IsMemberInGroupsOutput`](crate::operation::is_member_in_groups::IsMemberInGroupsOutput) with field(s):
    ///   - [`results(Option<Vec<GroupMembershipExistenceResult>>)`](crate::operation::is_member_in_groups::IsMemberInGroupsOutput::results): <p>A list containing the results of membership existence checks.</p>
    /// - On failure, responds with [`SdkError<IsMemberInGroupsError>`](crate::operation::is_member_in_groups::IsMemberInGroupsError)
    pub fn is_member_in_groups(
        &self,
    ) -> crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder {
        crate::operation::is_member_in_groups::builders::IsMemberInGroupsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
