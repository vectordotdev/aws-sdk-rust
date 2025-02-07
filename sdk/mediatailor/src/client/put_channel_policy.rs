// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutChannelPolicy`](crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_name(impl ::std::convert::Into<String>)`](crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder::channel_name) / [`set_channel_name(Option<String>)`](crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder::set_channel_name): <p>The channel name associated with this Channel Policy.</p>
    ///   - [`policy(impl ::std::convert::Into<String>)`](crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder::set_policy): <p>Adds an IAM role that determines the permissions of your channel.</p>
    /// - On success, responds with [`PutChannelPolicyOutput`](crate::operation::put_channel_policy::PutChannelPolicyOutput)
    /// - On failure, responds with [`SdkError<PutChannelPolicyError>`](crate::operation::put_channel_policy::PutChannelPolicyError)
    pub fn put_channel_policy(
        &self,
    ) -> crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder {
        crate::operation::put_channel_policy::builders::PutChannelPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
