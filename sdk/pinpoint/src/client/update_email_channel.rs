// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateEmailChannel`](crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`email_channel_request(EmailChannelRequest)`](crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder::email_channel_request) / [`set_email_channel_request(Option<EmailChannelRequest>)`](crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder::set_email_channel_request): <p>Specifies the status and settings of the email channel for an application.</p>
    /// - On success, responds with [`UpdateEmailChannelOutput`](crate::operation::update_email_channel::UpdateEmailChannelOutput) with field(s):
    ///   - [`email_channel_response(Option<EmailChannelResponse>)`](crate::operation::update_email_channel::UpdateEmailChannelOutput::email_channel_response): <p>Provides information about the status and settings of the email channel for an application.</p>
    /// - On failure, responds with [`SdkError<UpdateEmailChannelError>`](crate::operation::update_email_channel::UpdateEmailChannelError)
    pub fn update_email_channel(
        &self,
    ) -> crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder {
        crate::operation::update_email_channel::builders::UpdateEmailChannelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
