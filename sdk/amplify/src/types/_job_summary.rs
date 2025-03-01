// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Describes the summary for an execution job for an Amplify app. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JobSummary {
    /// <p> The Amazon Resource Name (ARN) for the job. </p>
    #[doc(hidden)]
    pub job_arn: ::std::option::Option<::std::string::String>,
    /// <p> The unique ID for the job. </p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
    /// <p> The commit ID from a third-party repository provider for the job. </p>
    #[doc(hidden)]
    pub commit_id: ::std::option::Option<::std::string::String>,
    /// <p> The commit message from a third-party repository provider for the job. </p>
    #[doc(hidden)]
    pub commit_message: ::std::option::Option<::std::string::String>,
    /// <p> The commit date and time for the job. </p>
    #[doc(hidden)]
    pub commit_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The start date and time for the job. </p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The current status for the job. </p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::JobStatus>,
    /// <p> The end date and time for the job. </p>
    #[doc(hidden)]
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The type for the job. If the value is <code>RELEASE</code>, the job was manually released from its source by using the <code>StartJob</code> API. If the value is <code>RETRY</code>, the job was manually retried using the <code>StartJob</code> API. If the value is <code>WEB_HOOK</code>, the job was automatically triggered by webhooks. </p>
    #[doc(hidden)]
    pub job_type: ::std::option::Option<crate::types::JobType>,
}
impl JobSummary {
    /// <p> The Amazon Resource Name (ARN) for the job. </p>
    pub fn job_arn(&self) -> ::std::option::Option<&str> {
        self.job_arn.as_deref()
    }
    /// <p> The unique ID for the job. </p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p> The commit ID from a third-party repository provider for the job. </p>
    pub fn commit_id(&self) -> ::std::option::Option<&str> {
        self.commit_id.as_deref()
    }
    /// <p> The commit message from a third-party repository provider for the job. </p>
    pub fn commit_message(&self) -> ::std::option::Option<&str> {
        self.commit_message.as_deref()
    }
    /// <p> The commit date and time for the job. </p>
    pub fn commit_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.commit_time.as_ref()
    }
    /// <p> The start date and time for the job. </p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p> The current status for the job. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::JobStatus> {
        self.status.as_ref()
    }
    /// <p> The end date and time for the job. </p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p> The type for the job. If the value is <code>RELEASE</code>, the job was manually released from its source by using the <code>StartJob</code> API. If the value is <code>RETRY</code>, the job was manually retried using the <code>StartJob</code> API. If the value is <code>WEB_HOOK</code>, the job was automatically triggered by webhooks. </p>
    pub fn job_type(&self) -> ::std::option::Option<&crate::types::JobType> {
        self.job_type.as_ref()
    }
}
impl JobSummary {
    /// Creates a new builder-style object to manufacture [`JobSummary`](crate::types::JobSummary).
    pub fn builder() -> crate::types::builders::JobSummaryBuilder {
        crate::types::builders::JobSummaryBuilder::default()
    }
}

/// A builder for [`JobSummary`](crate::types::JobSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JobSummaryBuilder {
    pub(crate) job_arn: ::std::option::Option<::std::string::String>,
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) commit_id: ::std::option::Option<::std::string::String>,
    pub(crate) commit_message: ::std::option::Option<::std::string::String>,
    pub(crate) commit_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<crate::types::JobStatus>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) job_type: ::std::option::Option<crate::types::JobType>,
}
impl JobSummaryBuilder {
    /// <p> The Amazon Resource Name (ARN) for the job. </p>
    pub fn job_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) for the job. </p>
    pub fn set_job_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_arn = input;
        self
    }
    /// <p> The unique ID for the job. </p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The unique ID for the job. </p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p> The commit ID from a third-party repository provider for the job. </p>
    pub fn commit_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.commit_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The commit ID from a third-party repository provider for the job. </p>
    pub fn set_commit_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.commit_id = input;
        self
    }
    /// <p> The commit message from a third-party repository provider for the job. </p>
    pub fn commit_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.commit_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The commit message from a third-party repository provider for the job. </p>
    pub fn set_commit_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.commit_message = input;
        self
    }
    /// <p> The commit date and time for the job. </p>
    pub fn commit_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.commit_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The commit date and time for the job. </p>
    pub fn set_commit_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.commit_time = input;
        self
    }
    /// <p> The start date and time for the job. </p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The start date and time for the job. </p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p> The current status for the job. </p>
    pub fn status(mut self, input: crate::types::JobStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p> The current status for the job. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::JobStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p> The end date and time for the job. </p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The end date and time for the job. </p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_time = input;
        self
    }
    /// <p> The type for the job. If the value is <code>RELEASE</code>, the job was manually released from its source by using the <code>StartJob</code> API. If the value is <code>RETRY</code>, the job was manually retried using the <code>StartJob</code> API. If the value is <code>WEB_HOOK</code>, the job was automatically triggered by webhooks. </p>
    pub fn job_type(mut self, input: crate::types::JobType) -> Self {
        self.job_type = ::std::option::Option::Some(input);
        self
    }
    /// <p> The type for the job. If the value is <code>RELEASE</code>, the job was manually released from its source by using the <code>StartJob</code> API. If the value is <code>RETRY</code>, the job was manually retried using the <code>StartJob</code> API. If the value is <code>WEB_HOOK</code>, the job was automatically triggered by webhooks. </p>
    pub fn set_job_type(mut self, input: ::std::option::Option<crate::types::JobType>) -> Self {
        self.job_type = input;
        self
    }
    /// Consumes the builder and constructs a [`JobSummary`](crate::types::JobSummary).
    pub fn build(self) -> crate::types::JobSummary {
        crate::types::JobSummary {
            job_arn: self.job_arn,
            job_id: self.job_id,
            commit_id: self.commit_id,
            commit_message: self.commit_message,
            commit_time: self.commit_time,
            start_time: self.start_time,
            status: self.status,
            end_time: self.end_time,
            job_type: self.job_type,
        }
    }
}
