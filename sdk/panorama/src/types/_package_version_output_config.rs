// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A package version output configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PackageVersionOutputConfig {
    /// <p>The output's package name.</p>
    #[doc(hidden)]
    pub package_name: ::std::option::Option<::std::string::String>,
    /// <p>The output's package version.</p>
    #[doc(hidden)]
    pub package_version: ::std::option::Option<::std::string::String>,
    /// <p>Indicates that the version is recommended for all users.</p>
    #[doc(hidden)]
    pub mark_latest: bool,
}
impl PackageVersionOutputConfig {
    /// <p>The output's package name.</p>
    pub fn package_name(&self) -> ::std::option::Option<&str> {
        self.package_name.as_deref()
    }
    /// <p>The output's package version.</p>
    pub fn package_version(&self) -> ::std::option::Option<&str> {
        self.package_version.as_deref()
    }
    /// <p>Indicates that the version is recommended for all users.</p>
    pub fn mark_latest(&self) -> bool {
        self.mark_latest
    }
}
impl PackageVersionOutputConfig {
    /// Creates a new builder-style object to manufacture [`PackageVersionOutputConfig`](crate::types::PackageVersionOutputConfig).
    pub fn builder() -> crate::types::builders::PackageVersionOutputConfigBuilder {
        crate::types::builders::PackageVersionOutputConfigBuilder::default()
    }
}

/// A builder for [`PackageVersionOutputConfig`](crate::types::PackageVersionOutputConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PackageVersionOutputConfigBuilder {
    pub(crate) package_name: ::std::option::Option<::std::string::String>,
    pub(crate) package_version: ::std::option::Option<::std::string::String>,
    pub(crate) mark_latest: ::std::option::Option<bool>,
}
impl PackageVersionOutputConfigBuilder {
    /// <p>The output's package name.</p>
    pub fn package_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The output's package name.</p>
    pub fn set_package_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_name = input;
        self
    }
    /// <p>The output's package version.</p>
    pub fn package_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.package_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The output's package version.</p>
    pub fn set_package_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.package_version = input;
        self
    }
    /// <p>Indicates that the version is recommended for all users.</p>
    pub fn mark_latest(mut self, input: bool) -> Self {
        self.mark_latest = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates that the version is recommended for all users.</p>
    pub fn set_mark_latest(mut self, input: ::std::option::Option<bool>) -> Self {
        self.mark_latest = input;
        self
    }
    /// Consumes the builder and constructs a [`PackageVersionOutputConfig`](crate::types::PackageVersionOutputConfig).
    pub fn build(self) -> crate::types::PackageVersionOutputConfig {
        crate::types::PackageVersionOutputConfig {
            package_name: self.package_name,
            package_version: self.package_version,
            mark_latest: self.mark_latest.unwrap_or_default(),
        }
    }
}
