// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateScheduleInput {
    /// <p>The name or names of one or more jobs to be run for this schedule.</p>
    #[doc(hidden)]
    pub job_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The date or dates and time or times when the jobs are to be run. For more information, see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron expressions</a> in the <i>Glue DataBrew Developer Guide</i>.</p>
    #[doc(hidden)]
    pub cron_expression: ::std::option::Option<::std::string::String>,
    /// <p>The name of the schedule to update.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl UpdateScheduleInput {
    /// <p>The name or names of one or more jobs to be run for this schedule.</p>
    pub fn job_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.job_names.as_deref()
    }
    /// <p>The date or dates and time or times when the jobs are to be run. For more information, see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron expressions</a> in the <i>Glue DataBrew Developer Guide</i>.</p>
    pub fn cron_expression(&self) -> ::std::option::Option<&str> {
        self.cron_expression.as_deref()
    }
    /// <p>The name of the schedule to update.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl UpdateScheduleInput {
    /// Creates a new builder-style object to manufacture [`UpdateScheduleInput`](crate::operation::update_schedule::UpdateScheduleInput).
    pub fn builder() -> crate::operation::update_schedule::builders::UpdateScheduleInputBuilder {
        crate::operation::update_schedule::builders::UpdateScheduleInputBuilder::default()
    }
}

/// A builder for [`UpdateScheduleInput`](crate::operation::update_schedule::UpdateScheduleInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateScheduleInputBuilder {
    pub(crate) job_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) cron_expression: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl UpdateScheduleInputBuilder {
    /// Appends an item to `job_names`.
    ///
    /// To override the contents of this collection use [`set_job_names`](Self::set_job_names).
    ///
    /// <p>The name or names of one or more jobs to be run for this schedule.</p>
    pub fn job_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.job_names.unwrap_or_default();
        v.push(input.into());
        self.job_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The name or names of one or more jobs to be run for this schedule.</p>
    pub fn set_job_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.job_names = input;
        self
    }
    /// <p>The date or dates and time or times when the jobs are to be run. For more information, see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron expressions</a> in the <i>Glue DataBrew Developer Guide</i>.</p>
    pub fn cron_expression(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.cron_expression = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The date or dates and time or times when the jobs are to be run. For more information, see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron expressions</a> in the <i>Glue DataBrew Developer Guide</i>.</p>
    pub fn set_cron_expression(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.cron_expression = input;
        self
    }
    /// <p>The name of the schedule to update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the schedule to update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateScheduleInput`](crate::operation::update_schedule::UpdateScheduleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_schedule::UpdateScheduleInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_schedule::UpdateScheduleInput {
            job_names: self.job_names,
            cron_expression: self.cron_expression,
            name: self.name,
        })
    }
}
