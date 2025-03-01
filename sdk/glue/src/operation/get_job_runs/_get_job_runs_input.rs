// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetJobRunsInput {
    /// <p>The name of the job definition for which to retrieve all job runs.</p>
    #[doc(hidden)]
    pub job_name: ::std::option::Option<::std::string::String>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum size of the response.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl GetJobRunsInput {
    /// <p>The name of the job definition for which to retrieve all job runs.</p>
    pub fn job_name(&self) -> ::std::option::Option<&str> {
        self.job_name.as_deref()
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum size of the response.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl GetJobRunsInput {
    /// Creates a new builder-style object to manufacture [`GetJobRunsInput`](crate::operation::get_job_runs::GetJobRunsInput).
    pub fn builder() -> crate::operation::get_job_runs::builders::GetJobRunsInputBuilder {
        crate::operation::get_job_runs::builders::GetJobRunsInputBuilder::default()
    }
}

/// A builder for [`GetJobRunsInput`](crate::operation::get_job_runs::GetJobRunsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetJobRunsInputBuilder {
    pub(crate) job_name: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl GetJobRunsInputBuilder {
    /// <p>The name of the job definition for which to retrieve all job runs.</p>
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the job definition for which to retrieve all job runs.</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_name = input;
        self
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum size of the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum size of the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`GetJobRunsInput`](crate::operation::get_job_runs::GetJobRunsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_job_runs::GetJobRunsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_job_runs::GetJobRunsInput {
            job_name: self.job_name,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
