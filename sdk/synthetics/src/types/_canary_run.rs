// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure contains the details about one run of one canary.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CanaryRun {
    /// <p>A unique ID that identifies this canary run.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the canary.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The status of this run.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::CanaryRunStatus>,
    /// <p>A structure that contains the start and end times of this run.</p>
    #[doc(hidden)]
    pub timeline: ::std::option::Option<crate::types::CanaryRunTimeline>,
    /// <p>The location where the canary stored artifacts from the run. Artifacts include the log file, screenshots, and HAR files.</p>
    #[doc(hidden)]
    pub artifact_s3_location: ::std::option::Option<::std::string::String>,
}
impl CanaryRun {
    /// <p>A unique ID that identifies this canary run.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The name of the canary.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The status of this run.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::CanaryRunStatus> {
        self.status.as_ref()
    }
    /// <p>A structure that contains the start and end times of this run.</p>
    pub fn timeline(&self) -> ::std::option::Option<&crate::types::CanaryRunTimeline> {
        self.timeline.as_ref()
    }
    /// <p>The location where the canary stored artifacts from the run. Artifacts include the log file, screenshots, and HAR files.</p>
    pub fn artifact_s3_location(&self) -> ::std::option::Option<&str> {
        self.artifact_s3_location.as_deref()
    }
}
impl CanaryRun {
    /// Creates a new builder-style object to manufacture [`CanaryRun`](crate::types::CanaryRun).
    pub fn builder() -> crate::types::builders::CanaryRunBuilder {
        crate::types::builders::CanaryRunBuilder::default()
    }
}

/// A builder for [`CanaryRun`](crate::types::CanaryRun).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CanaryRunBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::CanaryRunStatus>,
    pub(crate) timeline: ::std::option::Option<crate::types::CanaryRunTimeline>,
    pub(crate) artifact_s3_location: ::std::option::Option<::std::string::String>,
}
impl CanaryRunBuilder {
    /// <p>A unique ID that identifies this canary run.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique ID that identifies this canary run.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The name of the canary.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the canary.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The status of this run.</p>
    pub fn status(mut self, input: crate::types::CanaryRunStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of this run.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::CanaryRunStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>A structure that contains the start and end times of this run.</p>
    pub fn timeline(mut self, input: crate::types::CanaryRunTimeline) -> Self {
        self.timeline = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure that contains the start and end times of this run.</p>
    pub fn set_timeline(
        mut self,
        input: ::std::option::Option<crate::types::CanaryRunTimeline>,
    ) -> Self {
        self.timeline = input;
        self
    }
    /// <p>The location where the canary stored artifacts from the run. Artifacts include the log file, screenshots, and HAR files.</p>
    pub fn artifact_s3_location(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.artifact_s3_location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The location where the canary stored artifacts from the run. Artifacts include the log file, screenshots, and HAR files.</p>
    pub fn set_artifact_s3_location(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.artifact_s3_location = input;
        self
    }
    /// Consumes the builder and constructs a [`CanaryRun`](crate::types::CanaryRun).
    pub fn build(self) -> crate::types::CanaryRun {
        crate::types::CanaryRun {
            id: self.id,
            name: self.name,
            status: self.status,
            timeline: self.timeline,
            artifact_s3_location: self.artifact_s3_location,
        }
    }
}
