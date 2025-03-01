// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetChannelMessageStatus`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl ::std::convert::Into<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::set_channel_arn): <p>The ARN of the channel</p>
    ///   - [`message_id(impl ::std::convert::Into<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::message_id) / [`set_message_id(Option<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::set_message_id): <p>The ID of the message.</p>
    ///   - [`chime_bearer(impl ::std::convert::Into<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    ///   - [`sub_channel_id(impl ::std::convert::Into<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::sub_channel_id) / [`set_sub_channel_id(Option<String>)`](crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::set_sub_channel_id): <p>The ID of the SubChannel in the request.</p> <note>   <p>Only required when getting message status in a SubChannel that the user belongs to.</p>  </note>
    /// - On success, responds with [`GetChannelMessageStatusOutput`](crate::operation::get_channel_message_status::GetChannelMessageStatusOutput) with field(s):
    ///   - [`status(Option<ChannelMessageStatusStructure>)`](crate::operation::get_channel_message_status::GetChannelMessageStatusOutput::status): <p>The message status and details.</p>
    /// - On failure, responds with [`SdkError<GetChannelMessageStatusError>`](crate::operation::get_channel_message_status::GetChannelMessageStatusError)
    pub fn get_channel_message_status(
        &self,
    ) -> crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder
    {
        crate::operation::get_channel_message_status::builders::GetChannelMessageStatusFluentBuilder::new(self.handle.clone())
    }
}
