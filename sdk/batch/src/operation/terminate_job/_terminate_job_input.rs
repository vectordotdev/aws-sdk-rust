// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for <code>TerminateJob</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TerminateJobInput {
    /// <p>The Batch job ID of the job to terminate.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <code>DescribeJobs</code> operations on the job. This message is also recorded in the Batch activity logs.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<::std::string::String>,
}
impl TerminateJobInput {
    /// <p>The Batch job ID of the job to terminate.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <code>DescribeJobs</code> operations on the job. This message is also recorded in the Batch activity logs.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
}
impl TerminateJobInput {
    /// Creates a new builder-style object to manufacture [`TerminateJobInput`](crate::operation::terminate_job::TerminateJobInput).
    pub fn builder() -> crate::operation::terminate_job::builders::TerminateJobInputBuilder {
        crate::operation::terminate_job::builders::TerminateJobInputBuilder::default()
    }
}

/// A builder for [`TerminateJobInput`](crate::operation::terminate_job::TerminateJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TerminateJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) reason: ::std::option::Option<::std::string::String>,
}
impl TerminateJobInputBuilder {
    /// <p>The Batch job ID of the job to terminate.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Batch job ID of the job to terminate.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <code>DescribeJobs</code> operations on the job. This message is also recorded in the Batch activity logs.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <code>DescribeJobs</code> operations on the job. This message is also recorded in the Batch activity logs.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// Consumes the builder and constructs a [`TerminateJobInput`](crate::operation::terminate_job::TerminateJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::terminate_job::TerminateJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::terminate_job::TerminateJobInput {
            job_id: self.job_id,
            reason: self.reason,
        })
    }
}
