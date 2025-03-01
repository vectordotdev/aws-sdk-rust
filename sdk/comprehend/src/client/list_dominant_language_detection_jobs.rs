// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDominantLanguageDetectionJobs`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filter(DominantLanguageDetectionJobFilter)`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::filter) / [`set_filter(Option<DominantLanguageDetectionJobFilter>)`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::set_filter): <p>Filters that jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::set_next_token): <p>Identifies the next page of results to return.</p>
    ///   - [`max_results(i32)`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::set_max_results): <p>The maximum number of results to return in each page. The default is 100.</p>
    /// - On success, responds with [`ListDominantLanguageDetectionJobsOutput`](crate::operation::list_dominant_language_detection_jobs::ListDominantLanguageDetectionJobsOutput) with field(s):
    ///   - [`dominant_language_detection_job_properties_list(Option<Vec<DominantLanguageDetectionJobProperties>>)`](crate::operation::list_dominant_language_detection_jobs::ListDominantLanguageDetectionJobsOutput::dominant_language_detection_job_properties_list): <p>A list containing the properties of each job that is returned.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_dominant_language_detection_jobs::ListDominantLanguageDetectionJobsOutput::next_token): <p>Identifies the next page of results to return.</p>
    /// - On failure, responds with [`SdkError<ListDominantLanguageDetectionJobsError>`](crate::operation::list_dominant_language_detection_jobs::ListDominantLanguageDetectionJobsError)
    pub fn list_dominant_language_detection_jobs(&self) -> crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder{
        crate::operation::list_dominant_language_detection_jobs::builders::ListDominantLanguageDetectionJobsFluentBuilder::new(self.handle.clone())
    }
}
