// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The flag that enables the matching process of duplicate profiles.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MatchingRequest {
    /// <p>The flag that enables the matching process of duplicate profiles.</p>
    #[doc(hidden)]
    pub enabled: ::std::option::Option<bool>,
    /// <p>The day and time when do you want to start the Identity Resolution Job every week.</p>
    #[doc(hidden)]
    pub job_schedule: ::std::option::Option<crate::types::JobSchedule>,
    /// <p>Configuration information about the auto-merging process.</p>
    #[doc(hidden)]
    pub auto_merging: ::std::option::Option<crate::types::AutoMerging>,
    /// <p>Configuration information for exporting Identity Resolution results, for example, to an S3 bucket.</p>
    #[doc(hidden)]
    pub exporting_config: ::std::option::Option<crate::types::ExportingConfig>,
}
impl MatchingRequest {
    /// <p>The flag that enables the matching process of duplicate profiles.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
    /// <p>The day and time when do you want to start the Identity Resolution Job every week.</p>
    pub fn job_schedule(&self) -> ::std::option::Option<&crate::types::JobSchedule> {
        self.job_schedule.as_ref()
    }
    /// <p>Configuration information about the auto-merging process.</p>
    pub fn auto_merging(&self) -> ::std::option::Option<&crate::types::AutoMerging> {
        self.auto_merging.as_ref()
    }
    /// <p>Configuration information for exporting Identity Resolution results, for example, to an S3 bucket.</p>
    pub fn exporting_config(&self) -> ::std::option::Option<&crate::types::ExportingConfig> {
        self.exporting_config.as_ref()
    }
}
impl MatchingRequest {
    /// Creates a new builder-style object to manufacture [`MatchingRequest`](crate::types::MatchingRequest).
    pub fn builder() -> crate::types::builders::MatchingRequestBuilder {
        crate::types::builders::MatchingRequestBuilder::default()
    }
}

/// A builder for [`MatchingRequest`](crate::types::MatchingRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MatchingRequestBuilder {
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) job_schedule: ::std::option::Option<crate::types::JobSchedule>,
    pub(crate) auto_merging: ::std::option::Option<crate::types::AutoMerging>,
    pub(crate) exporting_config: ::std::option::Option<crate::types::ExportingConfig>,
}
impl MatchingRequestBuilder {
    /// <p>The flag that enables the matching process of duplicate profiles.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>The flag that enables the matching process of duplicate profiles.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>The day and time when do you want to start the Identity Resolution Job every week.</p>
    pub fn job_schedule(mut self, input: crate::types::JobSchedule) -> Self {
        self.job_schedule = ::std::option::Option::Some(input);
        self
    }
    /// <p>The day and time when do you want to start the Identity Resolution Job every week.</p>
    pub fn set_job_schedule(
        mut self,
        input: ::std::option::Option<crate::types::JobSchedule>,
    ) -> Self {
        self.job_schedule = input;
        self
    }
    /// <p>Configuration information about the auto-merging process.</p>
    pub fn auto_merging(mut self, input: crate::types::AutoMerging) -> Self {
        self.auto_merging = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration information about the auto-merging process.</p>
    pub fn set_auto_merging(
        mut self,
        input: ::std::option::Option<crate::types::AutoMerging>,
    ) -> Self {
        self.auto_merging = input;
        self
    }
    /// <p>Configuration information for exporting Identity Resolution results, for example, to an S3 bucket.</p>
    pub fn exporting_config(mut self, input: crate::types::ExportingConfig) -> Self {
        self.exporting_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration information for exporting Identity Resolution results, for example, to an S3 bucket.</p>
    pub fn set_exporting_config(
        mut self,
        input: ::std::option::Option<crate::types::ExportingConfig>,
    ) -> Self {
        self.exporting_config = input;
        self
    }
    /// Consumes the builder and constructs a [`MatchingRequest`](crate::types::MatchingRequest).
    pub fn build(self) -> crate::types::MatchingRequest {
        crate::types::MatchingRequest {
            enabled: self.enabled,
            job_schedule: self.job_schedule,
            auto_merging: self.auto_merging,
            exporting_config: self.exporting_config,
        }
    }
}
