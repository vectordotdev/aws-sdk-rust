// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NotifyApplicationStateInput {
    /// <p>The configurationId in Application Discovery Service that uniquely identifies the grouped application.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>Status of the application - Not Started, In-Progress, Complete.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ApplicationStatus>,
    /// <p>The timestamp when the application state changed.</p>
    #[doc(hidden)]
    pub update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    #[doc(hidden)]
    pub dry_run: bool,
}
impl NotifyApplicationStateInput {
    /// <p>The configurationId in Application Discovery Service that uniquely identifies the grouped application.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>Status of the application - Not Started, In-Progress, Complete.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ApplicationStatus> {
        self.status.as_ref()
    }
    /// <p>The timestamp when the application state changed.</p>
    pub fn update_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_date_time.as_ref()
    }
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}
impl NotifyApplicationStateInput {
    /// Creates a new builder-style object to manufacture [`NotifyApplicationStateInput`](crate::operation::notify_application_state::NotifyApplicationStateInput).
    pub fn builder(
    ) -> crate::operation::notify_application_state::builders::NotifyApplicationStateInputBuilder
    {
        crate::operation::notify_application_state::builders::NotifyApplicationStateInputBuilder::default()
    }
}

/// A builder for [`NotifyApplicationStateInput`](crate::operation::notify_application_state::NotifyApplicationStateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NotifyApplicationStateInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ApplicationStatus>,
    pub(crate) update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl NotifyApplicationStateInputBuilder {
    /// <p>The configurationId in Application Discovery Service that uniquely identifies the grouped application.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The configurationId in Application Discovery Service that uniquely identifies the grouped application.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>Status of the application - Not Started, In-Progress, Complete.</p>
    pub fn status(mut self, input: crate::types::ApplicationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Status of the application - Not Started, In-Progress, Complete.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ApplicationStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The timestamp when the application state changed.</p>
    pub fn update_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the application state changed.</p>
    pub fn set_update_date_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_date_time = input;
        self
    }
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`NotifyApplicationStateInput`](crate::operation::notify_application_state::NotifyApplicationStateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::notify_application_state::NotifyApplicationStateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::notify_application_state::NotifyApplicationStateInput {
                application_id: self.application_id,
                status: self.status,
                update_date_time: self.update_date_time,
                dry_run: self.dry_run.unwrap_or_default(),
            },
        )
    }
}
