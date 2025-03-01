// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of a Job node present in the workflow.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JobNodeDetails {
    /// <p>The information for the job runs represented by the job node.</p>
    #[doc(hidden)]
    pub job_runs: ::std::option::Option<::std::vec::Vec<crate::types::JobRun>>,
}
impl JobNodeDetails {
    /// <p>The information for the job runs represented by the job node.</p>
    pub fn job_runs(&self) -> ::std::option::Option<&[crate::types::JobRun]> {
        self.job_runs.as_deref()
    }
}
impl JobNodeDetails {
    /// Creates a new builder-style object to manufacture [`JobNodeDetails`](crate::types::JobNodeDetails).
    pub fn builder() -> crate::types::builders::JobNodeDetailsBuilder {
        crate::types::builders::JobNodeDetailsBuilder::default()
    }
}

/// A builder for [`JobNodeDetails`](crate::types::JobNodeDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JobNodeDetailsBuilder {
    pub(crate) job_runs: ::std::option::Option<::std::vec::Vec<crate::types::JobRun>>,
}
impl JobNodeDetailsBuilder {
    /// Appends an item to `job_runs`.
    ///
    /// To override the contents of this collection use [`set_job_runs`](Self::set_job_runs).
    ///
    /// <p>The information for the job runs represented by the job node.</p>
    pub fn job_runs(mut self, input: crate::types::JobRun) -> Self {
        let mut v = self.job_runs.unwrap_or_default();
        v.push(input);
        self.job_runs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The information for the job runs represented by the job node.</p>
    pub fn set_job_runs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::JobRun>>,
    ) -> Self {
        self.job_runs = input;
        self
    }
    /// Consumes the builder and constructs a [`JobNodeDetails`](crate::types::JobNodeDetails).
    pub fn build(self) -> crate::types::JobNodeDetails {
        crate::types::JobNodeDetails {
            job_runs: self.job_runs,
        }
    }
}
