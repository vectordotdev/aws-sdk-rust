// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRoom`](crate::operation::get_room::builders::GetRoomFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::get_room::builders::GetRoomFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_room::builders::GetRoomFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`room_id(impl ::std::convert::Into<String>)`](crate::operation::get_room::builders::GetRoomFluentBuilder::room_id) / [`set_room_id(Option<String>)`](crate::operation::get_room::builders::GetRoomFluentBuilder::set_room_id): <p>The room ID.</p>
    /// - On success, responds with [`GetRoomOutput`](crate::operation::get_room::GetRoomOutput) with field(s):
    ///   - [`room(Option<Room>)`](crate::operation::get_room::GetRoomOutput::room): <p>The room details.</p>
    /// - On failure, responds with [`SdkError<GetRoomError>`](crate::operation::get_room::GetRoomError)
    pub fn get_room(&self) -> crate::operation::get_room::builders::GetRoomFluentBuilder {
        crate::operation::get_room::builders::GetRoomFluentBuilder::new(self.handle.clone())
    }
}
