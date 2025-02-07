// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateChannel`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_description): A short text description of the Channel.
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_id): The ID of the Channel to update.
    /// - On success, responds with [`UpdateChannelOutput`](crate::operation::update_channel::UpdateChannelOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::arn): The Amazon Resource Name (ARN) assigned to the Channel.
    ///   - [`created_at(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::created_at): The date and time the Channel was created.
    ///   - [`description(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::description): A short text description of the Channel.
    ///   - [`egress_access_logs(Option<EgressAccessLogs>)`](crate::operation::update_channel::UpdateChannelOutput::egress_access_logs): Configure egress access logging.
    ///   - [`hls_ingest(Option<HlsIngest>)`](crate::operation::update_channel::UpdateChannelOutput::hls_ingest): An HTTP Live Streaming (HLS) ingest resource configuration.
    ///   - [`id(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::id): The ID of the Channel.
    ///   - [`ingress_access_logs(Option<IngressAccessLogs>)`](crate::operation::update_channel::UpdateChannelOutput::ingress_access_logs): Configure ingress access logging.
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_channel::UpdateChannelOutput::tags): A collection of tags associated with a resource
    /// - On failure, responds with [`SdkError<UpdateChannelError>`](crate::operation::update_channel::UpdateChannelError)
    pub fn update_channel(
        &self,
    ) -> crate::operation::update_channel::builders::UpdateChannelFluentBuilder {
        crate::operation::update_channel::builders::UpdateChannelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
