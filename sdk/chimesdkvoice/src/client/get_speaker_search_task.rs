// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSpeakerSearchTask`](crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder::set_voice_connector_id): <p>The Voice Connector ID.</p>
    ///   - [`speaker_search_task_id(impl ::std::convert::Into<String>)`](crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder::speaker_search_task_id) / [`set_speaker_search_task_id(Option<String>)`](crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder::set_speaker_search_task_id): <p>The ID of the speaker search task.</p>
    /// - On success, responds with [`GetSpeakerSearchTaskOutput`](crate::operation::get_speaker_search_task::GetSpeakerSearchTaskOutput) with field(s):
    ///   - [`speaker_search_task(Option<SpeakerSearchTask>)`](crate::operation::get_speaker_search_task::GetSpeakerSearchTaskOutput::speaker_search_task): <p>The details of the speaker search task.</p>
    /// - On failure, responds with [`SdkError<GetSpeakerSearchTaskError>`](crate::operation::get_speaker_search_task::GetSpeakerSearchTaskError)
    pub fn get_speaker_search_task(
        &self,
    ) -> crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder
    {
        crate::operation::get_speaker_search_task::builders::GetSpeakerSearchTaskFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
