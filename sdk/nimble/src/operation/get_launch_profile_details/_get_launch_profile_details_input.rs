// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLaunchProfileDetailsInput {
    /// <p>The ID of the launch profile used to control access from the streaming session.</p>
    #[doc(hidden)]
    pub launch_profile_id: ::std::option::Option<::std::string::String>,
    /// <p>The studio ID. </p>
    #[doc(hidden)]
    pub studio_id: ::std::option::Option<::std::string::String>,
}
impl GetLaunchProfileDetailsInput {
    /// <p>The ID of the launch profile used to control access from the streaming session.</p>
    pub fn launch_profile_id(&self) -> ::std::option::Option<&str> {
        self.launch_profile_id.as_deref()
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(&self) -> ::std::option::Option<&str> {
        self.studio_id.as_deref()
    }
}
impl GetLaunchProfileDetailsInput {
    /// Creates a new builder-style object to manufacture [`GetLaunchProfileDetailsInput`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsInput).
    pub fn builder(
    ) -> crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsInputBuilder
    {
        crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsInputBuilder::default()
    }
}

/// A builder for [`GetLaunchProfileDetailsInput`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetLaunchProfileDetailsInputBuilder {
    pub(crate) launch_profile_id: ::std::option::Option<::std::string::String>,
    pub(crate) studio_id: ::std::option::Option<::std::string::String>,
}
impl GetLaunchProfileDetailsInputBuilder {
    /// <p>The ID of the launch profile used to control access from the streaming session.</p>
    pub fn launch_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.launch_profile_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the launch profile used to control access from the streaming session.</p>
    pub fn set_launch_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.launch_profile_id = input;
        self
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.studio_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The studio ID. </p>
    pub fn set_studio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.studio_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetLaunchProfileDetailsInput`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_launch_profile_details::GetLaunchProfileDetailsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_launch_profile_details::GetLaunchProfileDetailsInput {
                launch_profile_id: self.launch_profile_id,
                studio_id: self.studio_id,
            },
        )
    }
}
