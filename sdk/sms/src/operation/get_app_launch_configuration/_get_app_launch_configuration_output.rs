// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAppLaunchConfigurationOutput {
    /// <p>The ID of the application.</p>
    #[doc(hidden)]
    pub app_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service role in the customer's account that CloudFormation uses to launch the application.</p>
    #[doc(hidden)]
    pub role_name: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    #[doc(hidden)]
    pub auto_launch: ::std::option::Option<bool>,
    /// <p>The launch configurations for server groups in this application.</p>
    #[doc(hidden)]
    pub server_group_launch_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::ServerGroupLaunchConfiguration>>,
    _request_id: Option<String>,
}
impl GetAppLaunchConfigurationOutput {
    /// <p>The ID of the application.</p>
    pub fn app_id(&self) -> ::std::option::Option<&str> {
        self.app_id.as_deref()
    }
    /// <p>The name of the service role in the customer's account that CloudFormation uses to launch the application.</p>
    pub fn role_name(&self) -> ::std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    pub fn auto_launch(&self) -> ::std::option::Option<bool> {
        self.auto_launch
    }
    /// <p>The launch configurations for server groups in this application.</p>
    pub fn server_group_launch_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::ServerGroupLaunchConfiguration]> {
        self.server_group_launch_configurations.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetAppLaunchConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAppLaunchConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`GetAppLaunchConfigurationOutput`](crate::operation::get_app_launch_configuration::GetAppLaunchConfigurationOutput).
    pub fn builder() -> crate::operation::get_app_launch_configuration::builders::GetAppLaunchConfigurationOutputBuilder{
        crate::operation::get_app_launch_configuration::builders::GetAppLaunchConfigurationOutputBuilder::default()
    }
}

/// A builder for [`GetAppLaunchConfigurationOutput`](crate::operation::get_app_launch_configuration::GetAppLaunchConfigurationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAppLaunchConfigurationOutputBuilder {
    pub(crate) app_id: ::std::option::Option<::std::string::String>,
    pub(crate) role_name: ::std::option::Option<::std::string::String>,
    pub(crate) auto_launch: ::std::option::Option<bool>,
    pub(crate) server_group_launch_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::ServerGroupLaunchConfiguration>>,
    _request_id: Option<String>,
}
impl GetAppLaunchConfigurationOutputBuilder {
    /// <p>The ID of the application.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the application.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_id = input;
        self
    }
    /// <p>The name of the service role in the customer's account that CloudFormation uses to launch the application.</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service role in the customer's account that CloudFormation uses to launch the application.</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    pub fn auto_launch(mut self, input: bool) -> Self {
        self.auto_launch = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    pub fn set_auto_launch(mut self, input: ::std::option::Option<bool>) -> Self {
        self.auto_launch = input;
        self
    }
    /// Appends an item to `server_group_launch_configurations`.
    ///
    /// To override the contents of this collection use [`set_server_group_launch_configurations`](Self::set_server_group_launch_configurations).
    ///
    /// <p>The launch configurations for server groups in this application.</p>
    pub fn server_group_launch_configurations(
        mut self,
        input: crate::types::ServerGroupLaunchConfiguration,
    ) -> Self {
        let mut v = self.server_group_launch_configurations.unwrap_or_default();
        v.push(input);
        self.server_group_launch_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The launch configurations for server groups in this application.</p>
    pub fn set_server_group_launch_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ServerGroupLaunchConfiguration>>,
    ) -> Self {
        self.server_group_launch_configurations = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetAppLaunchConfigurationOutput`](crate::operation::get_app_launch_configuration::GetAppLaunchConfigurationOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_app_launch_configuration::GetAppLaunchConfigurationOutput {
        crate::operation::get_app_launch_configuration::GetAppLaunchConfigurationOutput {
            app_id: self.app_id,
            role_name: self.role_name,
            auto_launch: self.auto_launch,
            server_group_launch_configurations: self.server_group_launch_configurations,
            _request_id: self._request_id,
        }
    }
}
