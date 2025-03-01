// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SuspendContactRecording`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`contact_id(impl ::std::convert::Into<String>)`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::contact_id) / [`set_contact_id(Option<String>)`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::set_contact_id): <p>The identifier of the contact.</p>
    ///   - [`initial_contact_id(impl ::std::convert::Into<String>)`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::initial_contact_id) / [`set_initial_contact_id(Option<String>)`](crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::set_initial_contact_id): <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    /// - On success, responds with [`SuspendContactRecordingOutput`](crate::operation::suspend_contact_recording::SuspendContactRecordingOutput)
    /// - On failure, responds with [`SdkError<SuspendContactRecordingError>`](crate::operation::suspend_contact_recording::SuspendContactRecordingError)
    pub fn suspend_contact_recording(
        &self,
    ) -> crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder
    {
        crate::operation::suspend_contact_recording::builders::SuspendContactRecordingFluentBuilder::new(self.handle.clone())
    }
}
