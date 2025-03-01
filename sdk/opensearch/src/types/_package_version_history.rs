// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about a package version.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PackageVersionHistory {
    /// <p>The package version.</p>
    #[doc(hidden)]
    pub package_version: ::std::option::Option<::std::string::String>,
    /// <p>A message associated with the package version when it was uploaded.</p>
    #[doc(hidden)]
    pub commit_message: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the package was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PackageVersionHistory {
    /// <p>The package version.</p>
    pub fn package_version(&self) -> ::std::option::Option<&str> {
        self.package_version.as_deref()
    }
    /// <p>A message associated with the package version when it was uploaded.</p>
    pub fn commit_message(&self) -> ::std::option::Option<&str> {
        self.commit_message.as_deref()
    }
    /// <p>The date and time when the package was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
}
impl PackageVersionHistory {
    /// Creates a new builder-style object to manufacture [`PackageVersionHistory`](crate::types::PackageVersionHistory).
    pub fn builder() -> crate::types::builders::PackageVersionHistoryBuilder {
        crate::types::builders::PackageVersionHistoryBuilder::default()
    }
}

/// A builder for [`PackageVersionHistory`](crate::types::PackageVersionHistory).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PackageVersionHistoryBuilder {
    pub(crate) package_version: ::std::option::Option<::std::string::String>,
    pub(crate) commit_message: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PackageVersionHistoryBuilder {
    /// <p>The package version.</p>
    pub fn package_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.package_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The package version.</p>
    pub fn set_package_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.package_version = input;
        self
    }
    /// <p>A message associated with the package version when it was uploaded.</p>
    pub fn commit_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.commit_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A message associated with the package version when it was uploaded.</p>
    pub fn set_commit_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.commit_message = input;
        self
    }
    /// <p>The date and time when the package was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the package was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// Consumes the builder and constructs a [`PackageVersionHistory`](crate::types::PackageVersionHistory).
    pub fn build(self) -> crate::types::PackageVersionHistory {
        crate::types::PackageVersionHistory {
            package_version: self.package_version,
            commit_message: self.commit_message,
            created_at: self.created_at,
        }
    }
}
