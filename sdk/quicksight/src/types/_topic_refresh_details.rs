// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details about the refresh of a topic.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TopicRefreshDetails {
    /// <p>The Amazon Resource Name (ARN) of the topic refresh.</p>
    #[doc(hidden)]
    pub refresh_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the refresh, which occurs as a result of topic creation or topic update.</p>
    #[doc(hidden)]
    pub refresh_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the refresh job that indicates whether the job is still running, completed successfully, or failed.</p>
    #[doc(hidden)]
    pub refresh_status: ::std::option::Option<crate::types::TopicRefreshStatus>,
}
impl TopicRefreshDetails {
    /// <p>The Amazon Resource Name (ARN) of the topic refresh.</p>
    pub fn refresh_arn(&self) -> ::std::option::Option<&str> {
        self.refresh_arn.as_deref()
    }
    /// <p>The ID of the refresh, which occurs as a result of topic creation or topic update.</p>
    pub fn refresh_id(&self) -> ::std::option::Option<&str> {
        self.refresh_id.as_deref()
    }
    /// <p>The status of the refresh job that indicates whether the job is still running, completed successfully, or failed.</p>
    pub fn refresh_status(&self) -> ::std::option::Option<&crate::types::TopicRefreshStatus> {
        self.refresh_status.as_ref()
    }
}
impl TopicRefreshDetails {
    /// Creates a new builder-style object to manufacture [`TopicRefreshDetails`](crate::types::TopicRefreshDetails).
    pub fn builder() -> crate::types::builders::TopicRefreshDetailsBuilder {
        crate::types::builders::TopicRefreshDetailsBuilder::default()
    }
}

/// A builder for [`TopicRefreshDetails`](crate::types::TopicRefreshDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TopicRefreshDetailsBuilder {
    pub(crate) refresh_arn: ::std::option::Option<::std::string::String>,
    pub(crate) refresh_id: ::std::option::Option<::std::string::String>,
    pub(crate) refresh_status: ::std::option::Option<crate::types::TopicRefreshStatus>,
}
impl TopicRefreshDetailsBuilder {
    /// <p>The Amazon Resource Name (ARN) of the topic refresh.</p>
    pub fn refresh_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.refresh_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the topic refresh.</p>
    pub fn set_refresh_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.refresh_arn = input;
        self
    }
    /// <p>The ID of the refresh, which occurs as a result of topic creation or topic update.</p>
    pub fn refresh_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.refresh_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the refresh, which occurs as a result of topic creation or topic update.</p>
    pub fn set_refresh_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.refresh_id = input;
        self
    }
    /// <p>The status of the refresh job that indicates whether the job is still running, completed successfully, or failed.</p>
    pub fn refresh_status(mut self, input: crate::types::TopicRefreshStatus) -> Self {
        self.refresh_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the refresh job that indicates whether the job is still running, completed successfully, or failed.</p>
    pub fn set_refresh_status(
        mut self,
        input: ::std::option::Option<crate::types::TopicRefreshStatus>,
    ) -> Self {
        self.refresh_status = input;
        self
    }
    /// Consumes the builder and constructs a [`TopicRefreshDetails`](crate::types::TopicRefreshDetails).
    pub fn build(self) -> crate::types::TopicRefreshDetails {
        crate::types::TopicRefreshDetails {
            refresh_arn: self.refresh_arn,
            refresh_id: self.refresh_id,
            refresh_status: self.refresh_status,
        }
    }
}
