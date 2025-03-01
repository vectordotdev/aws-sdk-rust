// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary information about dataset contents.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DatasetContentSummary {
    /// <p>The version of the dataset contents.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
    /// <p>The status of the dataset contents.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::DatasetContentStatus>,
    /// <p>The actual time the creation of the dataset contents was started.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time the creation of the dataset contents was scheduled to start.</p>
    #[doc(hidden)]
    pub schedule_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time the dataset content status was updated to SUCCEEDED or FAILED.</p>
    #[doc(hidden)]
    pub completion_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl DatasetContentSummary {
    /// <p>The version of the dataset contents.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
    /// <p>The status of the dataset contents.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DatasetContentStatus> {
        self.status.as_ref()
    }
    /// <p>The actual time the creation of the dataset contents was started.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The time the creation of the dataset contents was scheduled to start.</p>
    pub fn schedule_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.schedule_time.as_ref()
    }
    /// <p>The time the dataset content status was updated to SUCCEEDED or FAILED.</p>
    pub fn completion_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.completion_time.as_ref()
    }
}
impl DatasetContentSummary {
    /// Creates a new builder-style object to manufacture [`DatasetContentSummary`](crate::types::DatasetContentSummary).
    pub fn builder() -> crate::types::builders::DatasetContentSummaryBuilder {
        crate::types::builders::DatasetContentSummaryBuilder::default()
    }
}

/// A builder for [`DatasetContentSummary`](crate::types::DatasetContentSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DatasetContentSummaryBuilder {
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::DatasetContentStatus>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) schedule_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) completion_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl DatasetContentSummaryBuilder {
    /// <p>The version of the dataset contents.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the dataset contents.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The status of the dataset contents.</p>
    pub fn status(mut self, input: crate::types::DatasetContentStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the dataset contents.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::DatasetContentStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The actual time the creation of the dataset contents was started.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The actual time the creation of the dataset contents was started.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The time the creation of the dataset contents was scheduled to start.</p>
    pub fn schedule_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.schedule_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the creation of the dataset contents was scheduled to start.</p>
    pub fn set_schedule_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.schedule_time = input;
        self
    }
    /// <p>The time the dataset content status was updated to SUCCEEDED or FAILED.</p>
    pub fn completion_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.completion_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the dataset content status was updated to SUCCEEDED or FAILED.</p>
    pub fn set_completion_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.completion_time = input;
        self
    }
    /// Consumes the builder and constructs a [`DatasetContentSummary`](crate::types::DatasetContentSummary).
    pub fn build(self) -> crate::types::DatasetContentSummary {
        crate::types::DatasetContentSummary {
            version: self.version,
            status: self.status,
            creation_time: self.creation_time,
            schedule_time: self.schedule_time,
            completion_time: self.completion_time,
        }
    }
}
