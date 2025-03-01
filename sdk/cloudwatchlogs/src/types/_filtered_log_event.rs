// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a matched event.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FilteredLogEvent {
    /// <p>The name of the log stream to which this event belongs.</p>
    #[doc(hidden)]
    pub log_stream_name: ::std::option::Option<::std::string::String>,
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    #[doc(hidden)]
    pub timestamp: ::std::option::Option<i64>,
    /// <p>The data contained in the log event.</p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// <p>The time the event was ingested, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    #[doc(hidden)]
    pub ingestion_time: ::std::option::Option<i64>,
    /// <p>The ID of the event.</p>
    #[doc(hidden)]
    pub event_id: ::std::option::Option<::std::string::String>,
}
impl FilteredLogEvent {
    /// <p>The name of the log stream to which this event belongs.</p>
    pub fn log_stream_name(&self) -> ::std::option::Option<&str> {
        self.log_stream_name.as_deref()
    }
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn timestamp(&self) -> ::std::option::Option<i64> {
        self.timestamp
    }
    /// <p>The data contained in the log event.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>The time the event was ingested, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn ingestion_time(&self) -> ::std::option::Option<i64> {
        self.ingestion_time
    }
    /// <p>The ID of the event.</p>
    pub fn event_id(&self) -> ::std::option::Option<&str> {
        self.event_id.as_deref()
    }
}
impl FilteredLogEvent {
    /// Creates a new builder-style object to manufacture [`FilteredLogEvent`](crate::types::FilteredLogEvent).
    pub fn builder() -> crate::types::builders::FilteredLogEventBuilder {
        crate::types::builders::FilteredLogEventBuilder::default()
    }
}

/// A builder for [`FilteredLogEvent`](crate::types::FilteredLogEvent).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FilteredLogEventBuilder {
    pub(crate) log_stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) timestamp: ::std::option::Option<i64>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) ingestion_time: ::std::option::Option<i64>,
    pub(crate) event_id: ::std::option::Option<::std::string::String>,
}
impl FilteredLogEventBuilder {
    /// <p>The name of the log stream to which this event belongs.</p>
    pub fn log_stream_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.log_stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the log stream to which this event belongs.</p>
    pub fn set_log_stream_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.log_stream_name = input;
        self
    }
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn timestamp(mut self, input: i64) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<i64>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The data contained in the log event.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The data contained in the log event.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The time the event was ingested, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn ingestion_time(mut self, input: i64) -> Self {
        self.ingestion_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the event was ingested, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn set_ingestion_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.ingestion_time = input;
        self
    }
    /// <p>The ID of the event.</p>
    pub fn event_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the event.</p>
    pub fn set_event_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_id = input;
        self
    }
    /// Consumes the builder and constructs a [`FilteredLogEvent`](crate::types::FilteredLogEvent).
    pub fn build(self) -> crate::types::FilteredLogEvent {
        crate::types::FilteredLogEvent {
            log_stream_name: self.log_stream_name,
            timestamp: self.timestamp,
            message: self.message,
            ingestion_time: self.ingestion_time,
            event_id: self.event_id,
        }
    }
}
