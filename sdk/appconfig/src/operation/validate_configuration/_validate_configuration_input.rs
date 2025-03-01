// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidateConfigurationInput {
    /// <p>The application ID.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The configuration profile ID.</p>
    #[doc(hidden)]
    pub configuration_profile_id: ::std::option::Option<::std::string::String>,
    /// <p>The version of the configuration to validate.</p>
    #[doc(hidden)]
    pub configuration_version: ::std::option::Option<::std::string::String>,
}
impl ValidateConfigurationInput {
    /// <p>The application ID.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The configuration profile ID.</p>
    pub fn configuration_profile_id(&self) -> ::std::option::Option<&str> {
        self.configuration_profile_id.as_deref()
    }
    /// <p>The version of the configuration to validate.</p>
    pub fn configuration_version(&self) -> ::std::option::Option<&str> {
        self.configuration_version.as_deref()
    }
}
impl ValidateConfigurationInput {
    /// Creates a new builder-style object to manufacture [`ValidateConfigurationInput`](crate::operation::validate_configuration::ValidateConfigurationInput).
    pub fn builder(
    ) -> crate::operation::validate_configuration::builders::ValidateConfigurationInputBuilder {
        crate::operation::validate_configuration::builders::ValidateConfigurationInputBuilder::default()
    }
}

/// A builder for [`ValidateConfigurationInput`](crate::operation::validate_configuration::ValidateConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidateConfigurationInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_profile_id: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_version: ::std::option::Option<::std::string::String>,
}
impl ValidateConfigurationInputBuilder {
    /// <p>The application ID.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The application ID.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn configuration_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_profile_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn set_configuration_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_profile_id = input;
        self
    }
    /// <p>The version of the configuration to validate.</p>
    pub fn configuration_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the configuration to validate.</p>
    pub fn set_configuration_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_version = input;
        self
    }
    /// Consumes the builder and constructs a [`ValidateConfigurationInput`](crate::operation::validate_configuration::ValidateConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::validate_configuration::ValidateConfigurationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::validate_configuration::ValidateConfigurationInput {
                application_id: self.application_id,
                configuration_profile_id: self.configuration_profile_id,
                configuration_version: self.configuration_version,
            },
        )
    }
}
