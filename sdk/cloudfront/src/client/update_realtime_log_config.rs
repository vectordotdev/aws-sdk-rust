// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRealtimeLogConfig`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`end_points(Vec<EndPoint>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::end_points) / [`set_end_points(Option<Vec<EndPoint>>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::set_end_points): <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
    ///   - [`fields(Vec<String>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::fields) / [`set_fields(Option<Vec<String>>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::set_fields): <p>A list of fields to include in each real-time log record.</p>  <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::set_name): <p>The name for this real-time log configuration.</p>
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) for this real-time log configuration.</p>
    ///   - [`sampling_rate(i64)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::sampling_rate) / [`set_sampling_rate(Option<i64>)`](crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::set_sampling_rate): <p>The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. You must provide an integer between 1 and 100, inclusive.</p>
    /// - On success, responds with [`UpdateRealtimeLogConfigOutput`](crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigOutput) with field(s):
    ///   - [`realtime_log_config(Option<RealtimeLogConfig>)`](crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigOutput::realtime_log_config): <p>A real-time log configuration.</p>
    /// - On failure, responds with [`SdkError<UpdateRealtimeLogConfigError>`](crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError)
    pub fn update_realtime_log_config(
        &self,
    ) -> crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder
    {
        crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigFluentBuilder::new(self.handle.clone())
    }
}
