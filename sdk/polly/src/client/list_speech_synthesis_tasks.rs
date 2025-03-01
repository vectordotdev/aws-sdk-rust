// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSpeechSynthesisTasks`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::set_max_results): <p>Maximum number of speech synthesis tasks returned in a List operation.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::set_next_token): <p>The pagination token to use in the next request to continue the listing of speech synthesis tasks. </p>
    ///   - [`status(TaskStatus)`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::status) / [`set_status(Option<TaskStatus>)`](crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::set_status): <p>Status of the speech synthesis tasks returned in a List operation</p>
    /// - On success, responds with [`ListSpeechSynthesisTasksOutput`](crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksOutput::next_token): <p>An opaque pagination token returned from the previous List operation in this request. If present, this indicates where to continue the listing.</p>
    ///   - [`synthesis_tasks(Option<Vec<SynthesisTask>>)`](crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksOutput::synthesis_tasks): <p>List of SynthesisTask objects that provides information from the specified task in the list request, including output format, creation time, task status, and so on.</p>
    /// - On failure, responds with [`SdkError<ListSpeechSynthesisTasksError>`](crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError)
    pub fn list_speech_synthesis_tasks(&self) -> crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder{
        crate::operation::list_speech_synthesis_tasks::builders::ListSpeechSynthesisTasksFluentBuilder::new(self.handle.clone())
    }
}
