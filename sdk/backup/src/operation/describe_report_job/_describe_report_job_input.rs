// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeReportJobInput {
    /// <p>The identifier of the report job. A unique, randomly generated, Unicode, UTF-8 encoded string that is at most 1,024 bytes long. The report job ID cannot be edited.</p>
    #[doc(hidden)]
    pub report_job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeReportJobInput {
    /// <p>The identifier of the report job. A unique, randomly generated, Unicode, UTF-8 encoded string that is at most 1,024 bytes long. The report job ID cannot be edited.</p>
    pub fn report_job_id(&self) -> ::std::option::Option<&str> {
        self.report_job_id.as_deref()
    }
}
impl DescribeReportJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeReportJobInput`](crate::operation::describe_report_job::DescribeReportJobInput).
    pub fn builder(
    ) -> crate::operation::describe_report_job::builders::DescribeReportJobInputBuilder {
        crate::operation::describe_report_job::builders::DescribeReportJobInputBuilder::default()
    }
}

/// A builder for [`DescribeReportJobInput`](crate::operation::describe_report_job::DescribeReportJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeReportJobInputBuilder {
    pub(crate) report_job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeReportJobInputBuilder {
    /// <p>The identifier of the report job. A unique, randomly generated, Unicode, UTF-8 encoded string that is at most 1,024 bytes long. The report job ID cannot be edited.</p>
    pub fn report_job_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.report_job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the report job. A unique, randomly generated, Unicode, UTF-8 encoded string that is at most 1,024 bytes long. The report job ID cannot be edited.</p>
    pub fn set_report_job_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.report_job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeReportJobInput`](crate::operation::describe_report_job::DescribeReportJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_report_job::DescribeReportJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_report_job::DescribeReportJobInput {
                report_job_id: self.report_job_id,
            },
        )
    }
}
