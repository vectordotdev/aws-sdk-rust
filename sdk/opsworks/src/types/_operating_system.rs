// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes supported operating systems in AWS OpsWorks Stacks.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OperatingSystem {
    /// <p>The name of the operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of a supported operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The type of a supported operating system, either <code>Linux</code> or <code>Windows</code>.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
    /// <p>Supported configuration manager name and versions for an AWS OpsWorks Stacks operating system.</p>
    #[doc(hidden)]
    pub configuration_managers:
        ::std::option::Option<::std::vec::Vec<crate::types::OperatingSystemConfigurationManager>>,
    /// <p>A short name for the operating system manufacturer.</p>
    #[doc(hidden)]
    pub reported_name: ::std::option::Option<::std::string::String>,
    /// <p>The version of the operating system, including the release and edition, if applicable.</p>
    #[doc(hidden)]
    pub reported_version: ::std::option::Option<::std::string::String>,
    /// <p>Indicates that an operating system is not supported for new instances.</p>
    #[doc(hidden)]
    pub supported: ::std::option::Option<bool>,
}
impl OperatingSystem {
    /// <p>The name of the operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ID of a supported operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The type of a supported operating system, either <code>Linux</code> or <code>Windows</code>.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>Supported configuration manager name and versions for an AWS OpsWorks Stacks operating system.</p>
    pub fn configuration_managers(
        &self,
    ) -> ::std::option::Option<&[crate::types::OperatingSystemConfigurationManager]> {
        self.configuration_managers.as_deref()
    }
    /// <p>A short name for the operating system manufacturer.</p>
    pub fn reported_name(&self) -> ::std::option::Option<&str> {
        self.reported_name.as_deref()
    }
    /// <p>The version of the operating system, including the release and edition, if applicable.</p>
    pub fn reported_version(&self) -> ::std::option::Option<&str> {
        self.reported_version.as_deref()
    }
    /// <p>Indicates that an operating system is not supported for new instances.</p>
    pub fn supported(&self) -> ::std::option::Option<bool> {
        self.supported
    }
}
impl OperatingSystem {
    /// Creates a new builder-style object to manufacture [`OperatingSystem`](crate::types::OperatingSystem).
    pub fn builder() -> crate::types::builders::OperatingSystemBuilder {
        crate::types::builders::OperatingSystemBuilder::default()
    }
}

/// A builder for [`OperatingSystem`](crate::types::OperatingSystem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OperatingSystemBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_managers:
        ::std::option::Option<::std::vec::Vec<crate::types::OperatingSystemConfigurationManager>>,
    pub(crate) reported_name: ::std::option::Option<::std::string::String>,
    pub(crate) reported_version: ::std::option::Option<::std::string::String>,
    pub(crate) supported: ::std::option::Option<bool>,
}
impl OperatingSystemBuilder {
    /// <p>The name of the operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The ID of a supported operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of a supported operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The type of a supported operating system, either <code>Linux</code> or <code>Windows</code>.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of a supported operating system, either <code>Linux</code> or <code>Windows</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// Appends an item to `configuration_managers`.
    ///
    /// To override the contents of this collection use [`set_configuration_managers`](Self::set_configuration_managers).
    ///
    /// <p>Supported configuration manager name and versions for an AWS OpsWorks Stacks operating system.</p>
    pub fn configuration_managers(
        mut self,
        input: crate::types::OperatingSystemConfigurationManager,
    ) -> Self {
        let mut v = self.configuration_managers.unwrap_or_default();
        v.push(input);
        self.configuration_managers = ::std::option::Option::Some(v);
        self
    }
    /// <p>Supported configuration manager name and versions for an AWS OpsWorks Stacks operating system.</p>
    pub fn set_configuration_managers(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::OperatingSystemConfigurationManager>,
        >,
    ) -> Self {
        self.configuration_managers = input;
        self
    }
    /// <p>A short name for the operating system manufacturer.</p>
    pub fn reported_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reported_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A short name for the operating system manufacturer.</p>
    pub fn set_reported_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reported_name = input;
        self
    }
    /// <p>The version of the operating system, including the release and edition, if applicable.</p>
    pub fn reported_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reported_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the operating system, including the release and edition, if applicable.</p>
    pub fn set_reported_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reported_version = input;
        self
    }
    /// <p>Indicates that an operating system is not supported for new instances.</p>
    pub fn supported(mut self, input: bool) -> Self {
        self.supported = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates that an operating system is not supported for new instances.</p>
    pub fn set_supported(mut self, input: ::std::option::Option<bool>) -> Self {
        self.supported = input;
        self
    }
    /// Consumes the builder and constructs a [`OperatingSystem`](crate::types::OperatingSystem).
    pub fn build(self) -> crate::types::OperatingSystem {
        crate::types::OperatingSystem {
            name: self.name,
            id: self.id,
            r#type: self.r#type,
            configuration_managers: self.configuration_managers,
            reported_name: self.reported_name,
            reported_version: self.reported_version,
            supported: self.supported,
        }
    }
}
