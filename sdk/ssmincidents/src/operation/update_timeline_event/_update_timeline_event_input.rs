// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTimelineEventInput {
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    #[doc(hidden)]
    pub incident_record_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the event to update. You can use <code>ListTimelineEvents</code> to find an event's ID.</p>
    #[doc(hidden)]
    pub event_id: ::std::option::Option<::std::string::String>,
    /// <p>The time that the event occurred.</p>
    #[doc(hidden)]
    pub event_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The type of event. You can update events of type <code>Custom Event</code>.</p>
    #[doc(hidden)]
    pub event_type: ::std::option::Option<::std::string::String>,
    /// <p>A short description of the event.</p>
    #[doc(hidden)]
    pub event_data: ::std::option::Option<::std::string::String>,
    /// <p>Updates all existing references in a <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with that resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use its ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p> <important>
    /// <p>This update action overrides all existing references. If you want to keep existing references, you must specify them in the call. If you don't, this action removes any existing references and enters only new references.</p>
    /// </important>
    #[doc(hidden)]
    pub event_references: ::std::option::Option<::std::vec::Vec<crate::types::EventReference>>,
}
impl UpdateTimelineEventInput {
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    pub fn incident_record_arn(&self) -> ::std::option::Option<&str> {
        self.incident_record_arn.as_deref()
    }
    /// <p>The ID of the event to update. You can use <code>ListTimelineEvents</code> to find an event's ID.</p>
    pub fn event_id(&self) -> ::std::option::Option<&str> {
        self.event_id.as_deref()
    }
    /// <p>The time that the event occurred.</p>
    pub fn event_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.event_time.as_ref()
    }
    /// <p>The type of event. You can update events of type <code>Custom Event</code>.</p>
    pub fn event_type(&self) -> ::std::option::Option<&str> {
        self.event_type.as_deref()
    }
    /// <p>A short description of the event.</p>
    pub fn event_data(&self) -> ::std::option::Option<&str> {
        self.event_data.as_deref()
    }
    /// <p>Updates all existing references in a <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with that resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use its ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p> <important>
    /// <p>This update action overrides all existing references. If you want to keep existing references, you must specify them in the call. If you don't, this action removes any existing references and enters only new references.</p>
    /// </important>
    pub fn event_references(&self) -> ::std::option::Option<&[crate::types::EventReference]> {
        self.event_references.as_deref()
    }
}
impl UpdateTimelineEventInput {
    /// Creates a new builder-style object to manufacture [`UpdateTimelineEventInput`](crate::operation::update_timeline_event::UpdateTimelineEventInput).
    pub fn builder(
    ) -> crate::operation::update_timeline_event::builders::UpdateTimelineEventInputBuilder {
        crate::operation::update_timeline_event::builders::UpdateTimelineEventInputBuilder::default(
        )
    }
}

/// A builder for [`UpdateTimelineEventInput`](crate::operation::update_timeline_event::UpdateTimelineEventInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateTimelineEventInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) incident_record_arn: ::std::option::Option<::std::string::String>,
    pub(crate) event_id: ::std::option::Option<::std::string::String>,
    pub(crate) event_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) event_type: ::std::option::Option<::std::string::String>,
    pub(crate) event_data: ::std::option::Option<::std::string::String>,
    pub(crate) event_references:
        ::std::option::Option<::std::vec::Vec<crate::types::EventReference>>,
}
impl UpdateTimelineEventInputBuilder {
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    pub fn incident_record_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.incident_record_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    pub fn set_incident_record_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.incident_record_arn = input;
        self
    }
    /// <p>The ID of the event to update. You can use <code>ListTimelineEvents</code> to find an event's ID.</p>
    pub fn event_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the event to update. You can use <code>ListTimelineEvents</code> to find an event's ID.</p>
    pub fn set_event_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_id = input;
        self
    }
    /// <p>The time that the event occurred.</p>
    pub fn event_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.event_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time that the event occurred.</p>
    pub fn set_event_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.event_time = input;
        self
    }
    /// <p>The type of event. You can update events of type <code>Custom Event</code>.</p>
    pub fn event_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of event. You can update events of type <code>Custom Event</code>.</p>
    pub fn set_event_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_type = input;
        self
    }
    /// <p>A short description of the event.</p>
    pub fn event_data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A short description of the event.</p>
    pub fn set_event_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_data = input;
        self
    }
    /// Appends an item to `event_references`.
    ///
    /// To override the contents of this collection use [`set_event_references`](Self::set_event_references).
    ///
    /// <p>Updates all existing references in a <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with that resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use its ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p> <important>
    /// <p>This update action overrides all existing references. If you want to keep existing references, you must specify them in the call. If you don't, this action removes any existing references and enters only new references.</p>
    /// </important>
    pub fn event_references(mut self, input: crate::types::EventReference) -> Self {
        let mut v = self.event_references.unwrap_or_default();
        v.push(input);
        self.event_references = ::std::option::Option::Some(v);
        self
    }
    /// <p>Updates all existing references in a <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with that resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use its ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p> <important>
    /// <p>This update action overrides all existing references. If you want to keep existing references, you must specify them in the call. If you don't, this action removes any existing references and enters only new references.</p>
    /// </important>
    pub fn set_event_references(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EventReference>>,
    ) -> Self {
        self.event_references = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateTimelineEventInput`](crate::operation::update_timeline_event::UpdateTimelineEventInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_timeline_event::UpdateTimelineEventInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_timeline_event::UpdateTimelineEventInput {
                client_token: self.client_token,
                incident_record_arn: self.incident_record_arn,
                event_id: self.event_id,
                event_time: self.event_time,
                event_type: self.event_type,
                event_data: self.event_data,
                event_references: self.event_references,
            },
        )
    }
}
