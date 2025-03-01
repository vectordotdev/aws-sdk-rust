// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFHIRExportJobs`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`datastore_id(impl ::std::convert::Into<String>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::datastore_id) / [`set_datastore_id(Option<String>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_datastore_id): <p> This parameter limits the response to the export job with the specified Data Store ID. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_next_token): <p> A pagination token used to identify the next page of results to return for a ListFHIRExportJobs query. </p>
    ///   - [`max_results(i32)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_max_results): <p> This parameter limits the number of results returned for a ListFHIRExportJobs to a maximum quantity specified by the user. </p>
    ///   - [`job_name(impl ::std::convert::Into<String>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::job_name) / [`set_job_name(Option<String>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_job_name): <p> This parameter limits the response to the export job with the specified job name. </p>
    ///   - [`job_status(JobStatus)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::job_status) / [`set_job_status(Option<JobStatus>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_job_status): <p> This parameter limits the response to the export jobs with the specified job status. </p>
    ///   - [`submitted_before(DateTime)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::submitted_before) / [`set_submitted_before(Option<DateTime>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_submitted_before): <p> This parameter limits the response to FHIR export jobs submitted before a user specified date. </p>
    ///   - [`submitted_after(DateTime)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::submitted_after) / [`set_submitted_after(Option<DateTime>)`](crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::set_submitted_after): <p> This parameter limits the response to FHIR export jobs submitted after a user specified date. </p>
    /// - On success, responds with [`ListFhirExportJobsOutput`](crate::operation::list_fhir_export_jobs::ListFhirExportJobsOutput) with field(s):
    ///   - [`export_job_properties_list(Option<Vec<ExportJobProperties>>)`](crate::operation::list_fhir_export_jobs::ListFhirExportJobsOutput::export_job_properties_list): <p> The properties of listed FHIR export jobs, including the ID, ARN, name, and the status of the job. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_fhir_export_jobs::ListFhirExportJobsOutput::next_token): <p> A pagination token used to identify the next page of results to return for a ListFHIRExportJobs query. </p>
    /// - On failure, responds with [`SdkError<ListFHIRExportJobsError>`](crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError)
    pub fn list_fhir_export_jobs(
        &self,
    ) -> crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder {
        crate::operation::list_fhir_export_jobs::builders::ListFHIRExportJobsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
