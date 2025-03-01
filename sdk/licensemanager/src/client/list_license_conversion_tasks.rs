// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLicenseConversionTasks`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::set_next_token): <p>Token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::set_max_results): <p>Maximum number of results to return in a single call.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::set_filters): <p> Filters to scope the results. Valid filters are <code>ResourceArns</code> and <code>Status</code>. </p>
    /// - On success, responds with [`ListLicenseConversionTasksOutput`](crate::operation::list_license_conversion_tasks::ListLicenseConversionTasksOutput) with field(s):
    ///   - [`license_conversion_tasks(Option<Vec<LicenseConversionTask>>)`](crate::operation::list_license_conversion_tasks::ListLicenseConversionTasksOutput::license_conversion_tasks): <p>Information about the license configuration tasks for your account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_license_conversion_tasks::ListLicenseConversionTasksOutput::next_token): <p>Token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListLicenseConversionTasksError>`](crate::operation::list_license_conversion_tasks::ListLicenseConversionTasksError)
    pub fn list_license_conversion_tasks(&self) -> crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder{
        crate::operation::list_license_conversion_tasks::builders::ListLicenseConversionTasksFluentBuilder::new(self.handle.clone())
    }
}
