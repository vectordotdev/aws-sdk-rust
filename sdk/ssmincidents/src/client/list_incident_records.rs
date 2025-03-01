// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListIncidentRecords`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::set_filters): <p>Filters the list of incident records you want to search through. You can filter on the following keys:</p>  <ul>   <li> <p> <code>creationTime</code> </p> </li>   <li> <p> <code>impact</code> </p> </li>   <li> <p> <code>status</code> </p> </li>   <li> <p> <code>createdBy</code> </p> </li>  </ul>  <p>Note the following when when you use Filters:</p>  <ul>   <li> <p>If you don't specify a Filter, the response includes all incident records.</p> </li>   <li> <p>If you specify more than one filter in a single request, the response returns incident records that match all filters.</p> </li>   <li> <p>If you specify a filter with more than one value, the response returns incident records that match any of the values provided.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::set_max_results): <p>The maximum number of results per page.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::set_next_token): <p>The pagination token to continue to the next page of results.</p>
    /// - On success, responds with [`ListIncidentRecordsOutput`](crate::operation::list_incident_records::ListIncidentRecordsOutput) with field(s):
    ///   - [`incident_record_summaries(Option<Vec<IncidentRecordSummary>>)`](crate::operation::list_incident_records::ListIncidentRecordsOutput::incident_record_summaries): <p>The details of each listed incident record.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_incident_records::ListIncidentRecordsOutput::next_token): <p>The pagination token to continue to the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListIncidentRecordsError>`](crate::operation::list_incident_records::ListIncidentRecordsError)
    pub fn list_incident_records(
        &self,
    ) -> crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder {
        crate::operation::list_incident_records::builders::ListIncidentRecordsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
