// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopAutoMLJob`](crate::operation::stop_auto_ml_job::builders::StopAutoMLJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_ml_job_name(impl ::std::convert::Into<String>)`](crate::operation::stop_auto_ml_job::builders::StopAutoMLJobFluentBuilder::auto_ml_job_name) / [`set_auto_ml_job_name(Option<String>)`](crate::operation::stop_auto_ml_job::builders::StopAutoMLJobFluentBuilder::set_auto_ml_job_name): <p>The name of the object you are requesting.</p>
    /// - On success, responds with [`StopAutoMlJobOutput`](crate::operation::stop_auto_ml_job::StopAutoMlJobOutput)
    /// - On failure, responds with [`SdkError<StopAutoMLJobError>`](crate::operation::stop_auto_ml_job::StopAutoMLJobError)
    pub fn stop_auto_ml_job(
        &self,
    ) -> crate::operation::stop_auto_ml_job::builders::StopAutoMLJobFluentBuilder {
        crate::operation::stop_auto_ml_job::builders::StopAutoMLJobFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
