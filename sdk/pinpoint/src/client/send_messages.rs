// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendMessages`](crate::operation::send_messages::builders::SendMessagesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::send_messages::builders::SendMessagesFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::send_messages::builders::SendMessagesFluentBuilder::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`message_request(MessageRequest)`](crate::operation::send_messages::builders::SendMessagesFluentBuilder::message_request) / [`set_message_request(Option<MessageRequest>)`](crate::operation::send_messages::builders::SendMessagesFluentBuilder::set_message_request): <p>Specifies the configuration and other settings for a message.</p>
    /// - On success, responds with [`SendMessagesOutput`](crate::operation::send_messages::SendMessagesOutput) with field(s):
    ///   - [`message_response(Option<MessageResponse>)`](crate::operation::send_messages::SendMessagesOutput::message_response): <p>Provides information about the results of a request to send a message to an endpoint address.</p>
    /// - On failure, responds with [`SdkError<SendMessagesError>`](crate::operation::send_messages::SendMessagesError)
    pub fn send_messages(
        &self,
    ) -> crate::operation::send_messages::builders::SendMessagesFluentBuilder {
        crate::operation::send_messages::builders::SendMessagesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
