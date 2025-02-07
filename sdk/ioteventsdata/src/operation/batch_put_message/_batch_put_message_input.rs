// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchPutMessageInput {
    /// <p>The list of messages to send. Each message has the following format: <code>'{ "messageId": "string", "inputName": "string", "payload": "string"}'</code> </p>
    #[doc(hidden)]
    pub messages: ::std::option::Option<::std::vec::Vec<crate::types::Message>>,
}
impl BatchPutMessageInput {
    /// <p>The list of messages to send. Each message has the following format: <code>'{ "messageId": "string", "inputName": "string", "payload": "string"}'</code> </p>
    pub fn messages(&self) -> ::std::option::Option<&[crate::types::Message]> {
        self.messages.as_deref()
    }
}
impl BatchPutMessageInput {
    /// Creates a new builder-style object to manufacture [`BatchPutMessageInput`](crate::operation::batch_put_message::BatchPutMessageInput).
    pub fn builder() -> crate::operation::batch_put_message::builders::BatchPutMessageInputBuilder {
        crate::operation::batch_put_message::builders::BatchPutMessageInputBuilder::default()
    }
}

/// A builder for [`BatchPutMessageInput`](crate::operation::batch_put_message::BatchPutMessageInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchPutMessageInputBuilder {
    pub(crate) messages: ::std::option::Option<::std::vec::Vec<crate::types::Message>>,
}
impl BatchPutMessageInputBuilder {
    /// Appends an item to `messages`.
    ///
    /// To override the contents of this collection use [`set_messages`](Self::set_messages).
    ///
    /// <p>The list of messages to send. Each message has the following format: <code>'{ "messageId": "string", "inputName": "string", "payload": "string"}'</code> </p>
    pub fn messages(mut self, input: crate::types::Message) -> Self {
        let mut v = self.messages.unwrap_or_default();
        v.push(input);
        self.messages = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of messages to send. Each message has the following format: <code>'{ "messageId": "string", "inputName": "string", "payload": "string"}'</code> </p>
    pub fn set_messages(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Message>>,
    ) -> Self {
        self.messages = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchPutMessageInput`](crate::operation::batch_put_message::BatchPutMessageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_put_message::BatchPutMessageInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_put_message::BatchPutMessageInput {
            messages: self.messages,
        })
    }
}
