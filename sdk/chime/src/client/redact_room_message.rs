// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RedactRoomMessage`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`room_id(impl ::std::convert::Into<String>)`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::room_id) / [`set_room_id(Option<String>)`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::set_room_id): <p>The room ID.</p>
    ///   - [`message_id(impl ::std::convert::Into<String>)`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::message_id) / [`set_message_id(Option<String>)`](crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::set_message_id): <p>The message ID.</p>
    /// - On success, responds with [`RedactRoomMessageOutput`](crate::operation::redact_room_message::RedactRoomMessageOutput)
    /// - On failure, responds with [`SdkError<RedactRoomMessageError>`](crate::operation::redact_room_message::RedactRoomMessageError)
    pub fn redact_room_message(
        &self,
    ) -> crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder {
        crate::operation::redact_room_message::builders::RedactRoomMessageFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
