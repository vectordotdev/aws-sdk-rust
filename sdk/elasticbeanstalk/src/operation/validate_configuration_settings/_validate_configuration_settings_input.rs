// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of validation messages for a specified configuration template.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidateConfigurationSettingsInput {
    /// <p>The name of the application that the configuration template or environment belongs to.</p>
    #[doc(hidden)]
    pub application_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the configuration template to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and an environment name.</p>
    #[doc(hidden)]
    pub template_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the environment to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and a configuration template name.</p>
    #[doc(hidden)]
    pub environment_name: ::std::option::Option<::std::string::String>,
    /// <p>A list of the options and desired values to evaluate.</p>
    #[doc(hidden)]
    pub option_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationOptionSetting>>,
}
impl ValidateConfigurationSettingsInput {
    /// <p>The name of the application that the configuration template or environment belongs to.</p>
    pub fn application_name(&self) -> ::std::option::Option<&str> {
        self.application_name.as_deref()
    }
    /// <p>The name of the configuration template to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and an environment name.</p>
    pub fn template_name(&self) -> ::std::option::Option<&str> {
        self.template_name.as_deref()
    }
    /// <p>The name of the environment to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and a configuration template name.</p>
    pub fn environment_name(&self) -> ::std::option::Option<&str> {
        self.environment_name.as_deref()
    }
    /// <p>A list of the options and desired values to evaluate.</p>
    pub fn option_settings(
        &self,
    ) -> ::std::option::Option<&[crate::types::ConfigurationOptionSetting]> {
        self.option_settings.as_deref()
    }
}
impl ValidateConfigurationSettingsInput {
    /// Creates a new builder-style object to manufacture [`ValidateConfigurationSettingsInput`](crate::operation::validate_configuration_settings::ValidateConfigurationSettingsInput).
    pub fn builder() -> crate::operation::validate_configuration_settings::builders::ValidateConfigurationSettingsInputBuilder{
        crate::operation::validate_configuration_settings::builders::ValidateConfigurationSettingsInputBuilder::default()
    }
}

/// A builder for [`ValidateConfigurationSettingsInput`](crate::operation::validate_configuration_settings::ValidateConfigurationSettingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidateConfigurationSettingsInputBuilder {
    pub(crate) application_name: ::std::option::Option<::std::string::String>,
    pub(crate) template_name: ::std::option::Option<::std::string::String>,
    pub(crate) environment_name: ::std::option::Option<::std::string::String>,
    pub(crate) option_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationOptionSetting>>,
}
impl ValidateConfigurationSettingsInputBuilder {
    /// <p>The name of the application that the configuration template or environment belongs to.</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the application that the configuration template or environment belongs to.</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_name = input;
        self
    }
    /// <p>The name of the configuration template to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and an environment name.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the configuration template to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and an environment name.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.template_name = input;
        self
    }
    /// <p>The name of the environment to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and a configuration template name.</p>
    pub fn environment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the environment to validate the settings against.</p>
    /// <p>Condition: You cannot specify both this and a configuration template name.</p>
    pub fn set_environment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_name = input;
        self
    }
    /// Appends an item to `option_settings`.
    ///
    /// To override the contents of this collection use [`set_option_settings`](Self::set_option_settings).
    ///
    /// <p>A list of the options and desired values to evaluate.</p>
    pub fn option_settings(mut self, input: crate::types::ConfigurationOptionSetting) -> Self {
        let mut v = self.option_settings.unwrap_or_default();
        v.push(input);
        self.option_settings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the options and desired values to evaluate.</p>
    pub fn set_option_settings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationOptionSetting>>,
    ) -> Self {
        self.option_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`ValidateConfigurationSettingsInput`](crate::operation::validate_configuration_settings::ValidateConfigurationSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::validate_configuration_settings::ValidateConfigurationSettingsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::validate_configuration_settings::ValidateConfigurationSettingsInput {
                application_name: self.application_name,
                template_name: self.template_name,
                environment_name: self.environment_name,
                option_settings: self.option_settings,
            },
        )
    }
}
