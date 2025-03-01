// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopInferenceRecommendationsJob`](crate::operation::stop_inference_recommendations_job::builders::StopInferenceRecommendationsJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_name(impl ::std::convert::Into<String>)`](crate::operation::stop_inference_recommendations_job::builders::StopInferenceRecommendationsJobFluentBuilder::job_name) / [`set_job_name(Option<String>)`](crate::operation::stop_inference_recommendations_job::builders::StopInferenceRecommendationsJobFluentBuilder::set_job_name): <p>The name of the job you want to stop.</p>
    /// - On success, responds with [`StopInferenceRecommendationsJobOutput`](crate::operation::stop_inference_recommendations_job::StopInferenceRecommendationsJobOutput)
    /// - On failure, responds with [`SdkError<StopInferenceRecommendationsJobError>`](crate::operation::stop_inference_recommendations_job::StopInferenceRecommendationsJobError)
    pub fn stop_inference_recommendations_job(&self) -> crate::operation::stop_inference_recommendations_job::builders::StopInferenceRecommendationsJobFluentBuilder{
        crate::operation::stop_inference_recommendations_job::builders::StopInferenceRecommendationsJobFluentBuilder::new(self.handle.clone())
    }
}
