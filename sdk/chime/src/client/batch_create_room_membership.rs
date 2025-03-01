// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchCreateRoomMembership`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`room_id(impl ::std::convert::Into<String>)`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::room_id) / [`set_room_id(Option<String>)`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::set_room_id): <p>The room ID.</p>
    ///   - [`membership_item_list(Vec<MembershipItem>)`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::membership_item_list) / [`set_membership_item_list(Option<Vec<MembershipItem>>)`](crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::set_membership_item_list): <p>The list of membership items.</p>
    /// - On success, responds with [`BatchCreateRoomMembershipOutput`](crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput) with field(s):
    ///   - [`errors(Option<Vec<MemberError>>)`](crate::operation::batch_create_room_membership::BatchCreateRoomMembershipOutput::errors): <p>If the action fails for one or more of the member IDs in the request, a list of the member IDs is returned, along with error codes and error messages.</p>
    /// - On failure, responds with [`SdkError<BatchCreateRoomMembershipError>`](crate::operation::batch_create_room_membership::BatchCreateRoomMembershipError)
    pub fn batch_create_room_membership(&self) -> crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder{
        crate::operation::batch_create_room_membership::builders::BatchCreateRoomMembershipFluentBuilder::new(self.handle.clone())
    }
}
