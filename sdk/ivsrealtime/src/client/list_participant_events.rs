// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListParticipantEvents`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`stage_arn(impl ::std::convert::Into<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::stage_arn) / [`set_stage_arn(Option<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::set_stage_arn): <p>Stage ARN.</p>
    ///   - [`session_id(impl ::std::convert::Into<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::session_id) / [`set_session_id(Option<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::set_session_id): <p>ID of a session within the stage.</p>
    ///   - [`participant_id(impl ::std::convert::Into<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::participant_id) / [`set_participant_id(Option<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::set_participant_id): <p>Unique identifier for this participant. This is assigned by IVS and returned by <code>CreateParticipantToken</code>.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::set_next_token): <p>The first participant to retrieve. This is used for pagination; see the <code>nextToken</code> response field.</p>
    ///   - [`max_results(i32)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::set_max_results): <p>Maximum number of results to return. Default: 50.</p>
    /// - On success, responds with [`ListParticipantEventsOutput`](crate::operation::list_participant_events::ListParticipantEventsOutput) with field(s):
    ///   - [`events(Option<Vec<Event>>)`](crate::operation::list_participant_events::ListParticipantEventsOutput::events): <p>List of the matching events.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_participant_events::ListParticipantEventsOutput::next_token): <p>If there are more rooms than <code>maxResults</code>, use <code>nextToken</code> in the request to get the next set. </p>
    /// - On failure, responds with [`SdkError<ListParticipantEventsError>`](crate::operation::list_participant_events::ListParticipantEventsError)
    pub fn list_participant_events(
        &self,
    ) -> crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder
    {
        crate::operation::list_participant_events::builders::ListParticipantEventsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
