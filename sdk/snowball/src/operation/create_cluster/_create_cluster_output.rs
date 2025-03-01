// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateClusterOutput {
    /// <p>The automatically generated ID for a cluster.</p>
    #[doc(hidden)]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    /// <p>List of jobs created for this cluster. For syntax, see <a href="https://docs.aws.amazon.com/snowball/latest/api-reference/API_ListJobs.html#API_ListJobs_ResponseSyntax">ListJobsResult$JobListEntries</a> in this guide.</p>
    #[doc(hidden)]
    pub job_list_entries: ::std::option::Option<::std::vec::Vec<crate::types::JobListEntry>>,
    _request_id: Option<String>,
}
impl CreateClusterOutput {
    /// <p>The automatically generated ID for a cluster.</p>
    pub fn cluster_id(&self) -> ::std::option::Option<&str> {
        self.cluster_id.as_deref()
    }
    /// <p>List of jobs created for this cluster. For syntax, see <a href="https://docs.aws.amazon.com/snowball/latest/api-reference/API_ListJobs.html#API_ListJobs_ResponseSyntax">ListJobsResult$JobListEntries</a> in this guide.</p>
    pub fn job_list_entries(&self) -> ::std::option::Option<&[crate::types::JobListEntry]> {
        self.job_list_entries.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateClusterOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateClusterOutput {
    /// Creates a new builder-style object to manufacture [`CreateClusterOutput`](crate::operation::create_cluster::CreateClusterOutput).
    pub fn builder() -> crate::operation::create_cluster::builders::CreateClusterOutputBuilder {
        crate::operation::create_cluster::builders::CreateClusterOutputBuilder::default()
    }
}

/// A builder for [`CreateClusterOutput`](crate::operation::create_cluster::CreateClusterOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateClusterOutputBuilder {
    pub(crate) cluster_id: ::std::option::Option<::std::string::String>,
    pub(crate) job_list_entries: ::std::option::Option<::std::vec::Vec<crate::types::JobListEntry>>,
    _request_id: Option<String>,
}
impl CreateClusterOutputBuilder {
    /// <p>The automatically generated ID for a cluster.</p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The automatically generated ID for a cluster.</p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_id = input;
        self
    }
    /// Appends an item to `job_list_entries`.
    ///
    /// To override the contents of this collection use [`set_job_list_entries`](Self::set_job_list_entries).
    ///
    /// <p>List of jobs created for this cluster. For syntax, see <a href="https://docs.aws.amazon.com/snowball/latest/api-reference/API_ListJobs.html#API_ListJobs_ResponseSyntax">ListJobsResult$JobListEntries</a> in this guide.</p>
    pub fn job_list_entries(mut self, input: crate::types::JobListEntry) -> Self {
        let mut v = self.job_list_entries.unwrap_or_default();
        v.push(input);
        self.job_list_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of jobs created for this cluster. For syntax, see <a href="https://docs.aws.amazon.com/snowball/latest/api-reference/API_ListJobs.html#API_ListJobs_ResponseSyntax">ListJobsResult$JobListEntries</a> in this guide.</p>
    pub fn set_job_list_entries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::JobListEntry>>,
    ) -> Self {
        self.job_list_entries = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateClusterOutput`](crate::operation::create_cluster::CreateClusterOutput).
    pub fn build(self) -> crate::operation::create_cluster::CreateClusterOutput {
        crate::operation::create_cluster::CreateClusterOutput {
            cluster_id: self.cluster_id,
            job_list_entries: self.job_list_entries,
            _request_id: self._request_id,
        }
    }
}
