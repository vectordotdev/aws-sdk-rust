// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartMeetingTranscription`](crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`meeting_id(impl ::std::convert::Into<String>)`](crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder::meeting_id) / [`set_meeting_id(Option<String>)`](crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder::set_meeting_id): <p>The unique ID of the meeting being transcribed.</p>
    ///   - [`transcription_configuration(TranscriptionConfiguration)`](crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder::transcription_configuration) / [`set_transcription_configuration(Option<TranscriptionConfiguration>)`](crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder::set_transcription_configuration): <p>The configuration for the current transcription operation. Must contain <code>EngineTranscribeSettings</code> or <code>EngineTranscribeMedicalSettings</code>.</p>
    /// - On success, responds with [`StartMeetingTranscriptionOutput`](crate::operation::start_meeting_transcription::StartMeetingTranscriptionOutput)
    /// - On failure, responds with [`SdkError<StartMeetingTranscriptionError>`](crate::operation::start_meeting_transcription::StartMeetingTranscriptionError)
    pub fn start_meeting_transcription(&self) -> crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder{
        crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionFluentBuilder::new(self.handle.clone())
    }
}
