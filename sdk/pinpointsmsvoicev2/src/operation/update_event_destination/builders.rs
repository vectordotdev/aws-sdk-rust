// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_event_destination::_update_event_destination_output::UpdateEventDestinationOutputBuilder;

pub use crate::operation::update_event_destination::_update_event_destination_input::UpdateEventDestinationInputBuilder;

/// Fluent builder constructing a request to `UpdateEventDestination`.
///
/// <p>Updates an existing event destination in a configuration set. You can update the IAM role ARN for CloudWatch Logs and Kinesis Data Firehose. You can also enable or disable the event destination.</p>
/// <p>You may want to update an event destination to change its matching event types or updating the destination resource ARN. You can't change an event destination's type between CloudWatch Logs, Kinesis Data Firehose, and Amazon SNS.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEventDestinationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_event_destination::builders::UpdateEventDestinationInputBuilder,
}
impl UpdateEventDestinationFluentBuilder {
    /// Creates a new `UpdateEventDestination`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_event_destination::UpdateEventDestination,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_destination::UpdateEventDestinationError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_event_destination::UpdateEventDestinationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_destination::UpdateEventDestinationError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_event_destination::UpdateEventDestinationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_destination::UpdateEventDestinationError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_event_destination::UpdateEventDestination,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_event_destination::UpdateEventDestinationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The configuration set to update with the new event destination. Valid values for this can be the ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn configuration_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_set_name(input.into());
        self
    }
    /// <p>The configuration set to update with the new event destination. Valid values for this can be the ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_set_name(input);
        self
    }
    /// <p>The name to use for the event destination.</p>
    pub fn event_destination_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.event_destination_name(input.into());
        self
    }
    /// <p>The name to use for the event destination.</p>
    pub fn set_event_destination_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_event_destination_name(input);
        self
    }
    /// <p>When set to true logging is enabled.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>When set to true logging is enabled.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
    /// Appends an item to `MatchingEventTypes`.
    ///
    /// To override the contents of this collection use [`set_matching_event_types`](Self::set_matching_event_types).
    ///
    /// <p>An array of event types that determine which events to log.</p>
    pub fn matching_event_types(mut self, input: crate::types::EventType) -> Self {
        self.inner = self.inner.matching_event_types(input);
        self
    }
    /// <p>An array of event types that determine which events to log.</p>
    pub fn set_matching_event_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EventType>>,
    ) -> Self {
        self.inner = self.inner.set_matching_event_types(input);
        self
    }
    /// <p>An object that contains information about an event destination that sends data to CloudWatch Logs.</p>
    pub fn cloud_watch_logs_destination(
        mut self,
        input: crate::types::CloudWatchLogsDestination,
    ) -> Self {
        self.inner = self.inner.cloud_watch_logs_destination(input);
        self
    }
    /// <p>An object that contains information about an event destination that sends data to CloudWatch Logs.</p>
    pub fn set_cloud_watch_logs_destination(
        mut self,
        input: ::std::option::Option<crate::types::CloudWatchLogsDestination>,
    ) -> Self {
        self.inner = self.inner.set_cloud_watch_logs_destination(input);
        self
    }
    /// <p>An object that contains information about an event destination for logging to Kinesis Data Firehose.</p>
    pub fn kinesis_firehose_destination(
        mut self,
        input: crate::types::KinesisFirehoseDestination,
    ) -> Self {
        self.inner = self.inner.kinesis_firehose_destination(input);
        self
    }
    /// <p>An object that contains information about an event destination for logging to Kinesis Data Firehose.</p>
    pub fn set_kinesis_firehose_destination(
        mut self,
        input: ::std::option::Option<crate::types::KinesisFirehoseDestination>,
    ) -> Self {
        self.inner = self.inner.set_kinesis_firehose_destination(input);
        self
    }
    /// <p>An object that contains information about an event destination that sends data to Amazon SNS.</p>
    pub fn sns_destination(mut self, input: crate::types::SnsDestination) -> Self {
        self.inner = self.inner.sns_destination(input);
        self
    }
    /// <p>An object that contains information about an event destination that sends data to Amazon SNS.</p>
    pub fn set_sns_destination(
        mut self,
        input: ::std::option::Option<crate::types::SnsDestination>,
    ) -> Self {
        self.inner = self.inner.set_sns_destination(input);
        self
    }
}
