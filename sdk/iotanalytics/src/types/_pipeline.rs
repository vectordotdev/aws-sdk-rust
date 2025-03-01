// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a pipeline.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Pipeline {
    /// <p>The name of the pipeline.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the pipeline.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The activities that perform transformations on the messages.</p>
    #[doc(hidden)]
    pub activities: ::std::option::Option<::std::vec::Vec<crate::types::PipelineActivity>>,
    /// <p>A summary of information about the pipeline reprocessing.</p>
    #[doc(hidden)]
    pub reprocessing_summaries:
        ::std::option::Option<::std::vec::Vec<crate::types::ReprocessingSummary>>,
    /// <p>When the pipeline was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The last time the pipeline was updated.</p>
    #[doc(hidden)]
    pub last_update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl Pipeline {
    /// <p>The name of the pipeline.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ARN of the pipeline.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The activities that perform transformations on the messages.</p>
    pub fn activities(&self) -> ::std::option::Option<&[crate::types::PipelineActivity]> {
        self.activities.as_deref()
    }
    /// <p>A summary of information about the pipeline reprocessing.</p>
    pub fn reprocessing_summaries(
        &self,
    ) -> ::std::option::Option<&[crate::types::ReprocessingSummary]> {
        self.reprocessing_summaries.as_deref()
    }
    /// <p>When the pipeline was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The last time the pipeline was updated.</p>
    pub fn last_update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_update_time.as_ref()
    }
}
impl Pipeline {
    /// Creates a new builder-style object to manufacture [`Pipeline`](crate::types::Pipeline).
    pub fn builder() -> crate::types::builders::PipelineBuilder {
        crate::types::builders::PipelineBuilder::default()
    }
}

/// A builder for [`Pipeline`](crate::types::Pipeline).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PipelineBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) activities: ::std::option::Option<::std::vec::Vec<crate::types::PipelineActivity>>,
    pub(crate) reprocessing_summaries:
        ::std::option::Option<::std::vec::Vec<crate::types::ReprocessingSummary>>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PipelineBuilder {
    /// <p>The name of the pipeline.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pipeline.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The ARN of the pipeline.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the pipeline.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Appends an item to `activities`.
    ///
    /// To override the contents of this collection use [`set_activities`](Self::set_activities).
    ///
    /// <p>The activities that perform transformations on the messages.</p>
    pub fn activities(mut self, input: crate::types::PipelineActivity) -> Self {
        let mut v = self.activities.unwrap_or_default();
        v.push(input);
        self.activities = ::std::option::Option::Some(v);
        self
    }
    /// <p>The activities that perform transformations on the messages.</p>
    pub fn set_activities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PipelineActivity>>,
    ) -> Self {
        self.activities = input;
        self
    }
    /// Appends an item to `reprocessing_summaries`.
    ///
    /// To override the contents of this collection use [`set_reprocessing_summaries`](Self::set_reprocessing_summaries).
    ///
    /// <p>A summary of information about the pipeline reprocessing.</p>
    pub fn reprocessing_summaries(mut self, input: crate::types::ReprocessingSummary) -> Self {
        let mut v = self.reprocessing_summaries.unwrap_or_default();
        v.push(input);
        self.reprocessing_summaries = ::std::option::Option::Some(v);
        self
    }
    /// <p>A summary of information about the pipeline reprocessing.</p>
    pub fn set_reprocessing_summaries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ReprocessingSummary>>,
    ) -> Self {
        self.reprocessing_summaries = input;
        self
    }
    /// <p>When the pipeline was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the pipeline was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The last time the pipeline was updated.</p>
    pub fn last_update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last time the pipeline was updated.</p>
    pub fn set_last_update_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_update_time = input;
        self
    }
    /// Consumes the builder and constructs a [`Pipeline`](crate::types::Pipeline).
    pub fn build(self) -> crate::types::Pipeline {
        crate::types::Pipeline {
            name: self.name,
            arn: self.arn,
            activities: self.activities,
            reprocessing_summaries: self.reprocessing_summaries,
            creation_time: self.creation_time,
            last_update_time: self.last_update_time,
        }
    }
}
