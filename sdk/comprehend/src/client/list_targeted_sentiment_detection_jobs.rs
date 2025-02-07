// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTargetedSentimentDetectionJobs`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filter(TargetedSentimentDetectionJobFilter)`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::filter) / [`set_filter(Option<TargetedSentimentDetectionJobFilter>)`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::set_filter): <p>Filters the jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::set_next_token): <p>Identifies the next page of results to return.</p>
    ///   - [`max_results(i32)`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::set_max_results): <p>The maximum number of results to return in each page. The default is 100.</p>
    /// - On success, responds with [`ListTargetedSentimentDetectionJobsOutput`](crate::operation::list_targeted_sentiment_detection_jobs::ListTargetedSentimentDetectionJobsOutput) with field(s):
    ///   - [`targeted_sentiment_detection_job_properties_list(Option<Vec<TargetedSentimentDetectionJobProperties>>)`](crate::operation::list_targeted_sentiment_detection_jobs::ListTargetedSentimentDetectionJobsOutput::targeted_sentiment_detection_job_properties_list): <p>A list containing the properties of each job that is returned.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_targeted_sentiment_detection_jobs::ListTargetedSentimentDetectionJobsOutput::next_token): <p>Identifies the next page of results to return.</p>
    /// - On failure, responds with [`SdkError<ListTargetedSentimentDetectionJobsError>`](crate::operation::list_targeted_sentiment_detection_jobs::ListTargetedSentimentDetectionJobsError)
    pub fn list_targeted_sentiment_detection_jobs(&self) -> crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder{
        crate::operation::list_targeted_sentiment_detection_jobs::builders::ListTargetedSentimentDetectionJobsFluentBuilder::new(self.handle.clone())
    }
}
