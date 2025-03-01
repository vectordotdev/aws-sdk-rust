// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceChannel`](crate::operation::get_voice_channel::builders::GetVoiceChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::get_voice_channel::builders::GetVoiceChannelFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::get_voice_channel::builders::GetVoiceChannelFluentBuilder::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    /// - On success, responds with [`GetVoiceChannelOutput`](crate::operation::get_voice_channel::GetVoiceChannelOutput) with field(s):
    ///   - [`voice_channel_response(Option<VoiceChannelResponse>)`](crate::operation::get_voice_channel::GetVoiceChannelOutput::voice_channel_response): <p>Provides information about the status and settings of the voice channel for an application.</p>
    /// - On failure, responds with [`SdkError<GetVoiceChannelError>`](crate::operation::get_voice_channel::GetVoiceChannelError)
    pub fn get_voice_channel(
        &self,
    ) -> crate::operation::get_voice_channel::builders::GetVoiceChannelFluentBuilder {
        crate::operation::get_voice_channel::builders::GetVoiceChannelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
