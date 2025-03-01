// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output for <code>EnableEnhancedMonitoring</code> and <code>DisableEnhancedMonitoring</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableEnhancedMonitoringOutput {
    /// <p>The name of the Kinesis data stream.</p>
    #[doc(hidden)]
    pub stream_name: ::std::option::Option<::std::string::String>,
    /// <p>Represents the current state of the metrics that are in the enhanced state before the operation.</p>
    #[doc(hidden)]
    pub current_shard_level_metrics:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>,
    /// <p>Represents the list of all the metrics that would be in the enhanced state after the operation.</p>
    #[doc(hidden)]
    pub desired_shard_level_metrics:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>,
    /// <p>The ARN of the stream.</p>
    #[doc(hidden)]
    pub stream_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DisableEnhancedMonitoringOutput {
    /// <p>The name of the Kinesis data stream.</p>
    pub fn stream_name(&self) -> ::std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>Represents the current state of the metrics that are in the enhanced state before the operation.</p>
    pub fn current_shard_level_metrics(
        &self,
    ) -> ::std::option::Option<&[crate::types::MetricsName]> {
        self.current_shard_level_metrics.as_deref()
    }
    /// <p>Represents the list of all the metrics that would be in the enhanced state after the operation.</p>
    pub fn desired_shard_level_metrics(
        &self,
    ) -> ::std::option::Option<&[crate::types::MetricsName]> {
        self.desired_shard_level_metrics.as_deref()
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DisableEnhancedMonitoringOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisableEnhancedMonitoringOutput {
    /// Creates a new builder-style object to manufacture [`DisableEnhancedMonitoringOutput`](crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput).
    pub fn builder() -> crate::operation::disable_enhanced_monitoring::builders::DisableEnhancedMonitoringOutputBuilder{
        crate::operation::disable_enhanced_monitoring::builders::DisableEnhancedMonitoringOutputBuilder::default()
    }
}

/// A builder for [`DisableEnhancedMonitoringOutput`](crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisableEnhancedMonitoringOutputBuilder {
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) current_shard_level_metrics:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>,
    pub(crate) desired_shard_level_metrics:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DisableEnhancedMonitoringOutputBuilder {
    /// <p>The name of the Kinesis data stream.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Kinesis data stream.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_name = input;
        self
    }
    /// Appends an item to `current_shard_level_metrics`.
    ///
    /// To override the contents of this collection use [`set_current_shard_level_metrics`](Self::set_current_shard_level_metrics).
    ///
    /// <p>Represents the current state of the metrics that are in the enhanced state before the operation.</p>
    pub fn current_shard_level_metrics(mut self, input: crate::types::MetricsName) -> Self {
        let mut v = self.current_shard_level_metrics.unwrap_or_default();
        v.push(input);
        self.current_shard_level_metrics = ::std::option::Option::Some(v);
        self
    }
    /// <p>Represents the current state of the metrics that are in the enhanced state before the operation.</p>
    pub fn set_current_shard_level_metrics(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>,
    ) -> Self {
        self.current_shard_level_metrics = input;
        self
    }
    /// Appends an item to `desired_shard_level_metrics`.
    ///
    /// To override the contents of this collection use [`set_desired_shard_level_metrics`](Self::set_desired_shard_level_metrics).
    ///
    /// <p>Represents the list of all the metrics that would be in the enhanced state after the operation.</p>
    pub fn desired_shard_level_metrics(mut self, input: crate::types::MetricsName) -> Self {
        let mut v = self.desired_shard_level_metrics.unwrap_or_default();
        v.push(input);
        self.desired_shard_level_metrics = ::std::option::Option::Some(v);
        self
    }
    /// <p>Represents the list of all the metrics that would be in the enhanced state after the operation.</p>
    pub fn set_desired_shard_level_metrics(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>,
    ) -> Self {
        self.desired_shard_level_metrics = input;
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisableEnhancedMonitoringOutput`](crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput).
    pub fn build(
        self,
    ) -> crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput {
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput {
            stream_name: self.stream_name,
            current_shard_level_metrics: self.current_shard_level_metrics,
            desired_shard_level_metrics: self.desired_shard_level_metrics,
            stream_arn: self.stream_arn,
            _request_id: self._request_id,
        }
    }
}
