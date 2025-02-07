// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopLabelingJob`](crate::operation::stop_labeling_job::builders::StopLabelingJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`labeling_job_name(impl ::std::convert::Into<String>)`](crate::operation::stop_labeling_job::builders::StopLabelingJobFluentBuilder::labeling_job_name) / [`set_labeling_job_name(Option<String>)`](crate::operation::stop_labeling_job::builders::StopLabelingJobFluentBuilder::set_labeling_job_name): <p>The name of the labeling job to stop.</p>
    /// - On success, responds with [`StopLabelingJobOutput`](crate::operation::stop_labeling_job::StopLabelingJobOutput)
    /// - On failure, responds with [`SdkError<StopLabelingJobError>`](crate::operation::stop_labeling_job::StopLabelingJobError)
    pub fn stop_labeling_job(
        &self,
    ) -> crate::operation::stop_labeling_job::builders::StopLabelingJobFluentBuilder {
        crate::operation::stop_labeling_job::builders::StopLabelingJobFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
