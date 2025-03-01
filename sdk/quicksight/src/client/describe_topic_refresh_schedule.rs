// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTopicRefreshSchedule`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::set_aws_account_id): <p>The Amazon Web Services account ID.</p>
    ///   - [`topic_id(impl ::std::convert::Into<String>)`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::topic_id) / [`set_topic_id(Option<String>)`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::set_topic_id): <p>The ID of the topic that contains the refresh schedule that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    ///   - [`dataset_id(impl ::std::convert::Into<String>)`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::dataset_id) / [`set_dataset_id(Option<String>)`](crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::set_dataset_id): <p>The ID of the dataset.</p>
    /// - On success, responds with [`DescribeTopicRefreshScheduleOutput`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput) with field(s):
    ///   - [`topic_id(Option<String>)`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput::topic_id): <p>The ID of the topic that contains the refresh schedule that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    ///   - [`topic_arn(Option<String>)`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput::topic_arn): <p>The Amazon Resource Name (ARN) of the topic.</p>
    ///   - [`dataset_arn(Option<String>)`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput::dataset_arn): <p>The Amazon Resource Name (ARN) of the dataset.</p>
    ///   - [`refresh_schedule(Option<TopicRefreshSchedule>)`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput::refresh_schedule): <p>The definition of a refresh schedule.</p>
    ///   - [`status(i32)`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<DescribeTopicRefreshScheduleError>`](crate::operation::describe_topic_refresh_schedule::DescribeTopicRefreshScheduleError)
    pub fn describe_topic_refresh_schedule(&self) -> crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder{
        crate::operation::describe_topic_refresh_schedule::builders::DescribeTopicRefreshScheduleFluentBuilder::new(self.handle.clone())
    }
}
