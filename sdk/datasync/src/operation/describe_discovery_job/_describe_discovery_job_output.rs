// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeDiscoveryJobOutput {
    /// <p>The ARN of the on-premises storage system you're running the discovery job on.</p>
    #[doc(hidden)]
    pub storage_system_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the discovery job.</p>
    #[doc(hidden)]
    pub discovery_job_arn: ::std::option::Option<::std::string::String>,
    /// <p>The number of minutes that the discovery job runs.</p>
    #[doc(hidden)]
    pub collection_duration_minutes: ::std::option::Option<i32>,
    /// <p>Indicates the status of a discovery job. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/discovery-job-statuses.html#discovery-job-statuses-table">Discovery job statuses</a>.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::DiscoveryJobStatus>,
    /// <p>The time when the discovery job started.</p>
    #[doc(hidden)]
    pub job_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time when the discovery job ended.</p>
    #[doc(hidden)]
    pub job_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl DescribeDiscoveryJobOutput {
    /// <p>The ARN of the on-premises storage system you're running the discovery job on.</p>
    pub fn storage_system_arn(&self) -> ::std::option::Option<&str> {
        self.storage_system_arn.as_deref()
    }
    /// <p>The ARN of the discovery job.</p>
    pub fn discovery_job_arn(&self) -> ::std::option::Option<&str> {
        self.discovery_job_arn.as_deref()
    }
    /// <p>The number of minutes that the discovery job runs.</p>
    pub fn collection_duration_minutes(&self) -> ::std::option::Option<i32> {
        self.collection_duration_minutes
    }
    /// <p>Indicates the status of a discovery job. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/discovery-job-statuses.html#discovery-job-statuses-table">Discovery job statuses</a>.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DiscoveryJobStatus> {
        self.status.as_ref()
    }
    /// <p>The time when the discovery job started.</p>
    pub fn job_start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.job_start_time.as_ref()
    }
    /// <p>The time when the discovery job ended.</p>
    pub fn job_end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.job_end_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeDiscoveryJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeDiscoveryJobOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDiscoveryJobOutput`](crate::operation::describe_discovery_job::DescribeDiscoveryJobOutput).
    pub fn builder(
    ) -> crate::operation::describe_discovery_job::builders::DescribeDiscoveryJobOutputBuilder {
        crate::operation::describe_discovery_job::builders::DescribeDiscoveryJobOutputBuilder::default()
    }
}

/// A builder for [`DescribeDiscoveryJobOutput`](crate::operation::describe_discovery_job::DescribeDiscoveryJobOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeDiscoveryJobOutputBuilder {
    pub(crate) storage_system_arn: ::std::option::Option<::std::string::String>,
    pub(crate) discovery_job_arn: ::std::option::Option<::std::string::String>,
    pub(crate) collection_duration_minutes: ::std::option::Option<i32>,
    pub(crate) status: ::std::option::Option<crate::types::DiscoveryJobStatus>,
    pub(crate) job_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) job_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl DescribeDiscoveryJobOutputBuilder {
    /// <p>The ARN of the on-premises storage system you're running the discovery job on.</p>
    pub fn storage_system_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.storage_system_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the on-premises storage system you're running the discovery job on.</p>
    pub fn set_storage_system_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.storage_system_arn = input;
        self
    }
    /// <p>The ARN of the discovery job.</p>
    pub fn discovery_job_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.discovery_job_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the discovery job.</p>
    pub fn set_discovery_job_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.discovery_job_arn = input;
        self
    }
    /// <p>The number of minutes that the discovery job runs.</p>
    pub fn collection_duration_minutes(mut self, input: i32) -> Self {
        self.collection_duration_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of minutes that the discovery job runs.</p>
    pub fn set_collection_duration_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.collection_duration_minutes = input;
        self
    }
    /// <p>Indicates the status of a discovery job. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/discovery-job-statuses.html#discovery-job-statuses-table">Discovery job statuses</a>.</p>
    pub fn status(mut self, input: crate::types::DiscoveryJobStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the status of a discovery job. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/discovery-job-statuses.html#discovery-job-statuses-table">Discovery job statuses</a>.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::DiscoveryJobStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The time when the discovery job started.</p>
    pub fn job_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.job_start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the discovery job started.</p>
    pub fn set_job_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.job_start_time = input;
        self
    }
    /// <p>The time when the discovery job ended.</p>
    pub fn job_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.job_end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the discovery job ended.</p>
    pub fn set_job_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.job_end_time = input;
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
    /// Consumes the builder and constructs a [`DescribeDiscoveryJobOutput`](crate::operation::describe_discovery_job::DescribeDiscoveryJobOutput).
    pub fn build(self) -> crate::operation::describe_discovery_job::DescribeDiscoveryJobOutput {
        crate::operation::describe_discovery_job::DescribeDiscoveryJobOutput {
            storage_system_arn: self.storage_system_arn,
            discovery_job_arn: self.discovery_job_arn,
            collection_duration_minutes: self.collection_duration_minutes,
            status: self.status,
            job_start_time: self.job_start_time,
            job_end_time: self.job_end_time,
            _request_id: self._request_id,
        }
    }
}
