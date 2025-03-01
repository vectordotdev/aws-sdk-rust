// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents an activity type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ActivityType {
    /// <p>The name of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique within a domain.</p>
    /// </note>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The version of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique with in a domain.</p>
    /// </note>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ActivityType {
    /// <p>The name of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique within a domain.</p>
    /// </note>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The version of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique with in a domain.</p>
    /// </note>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl ActivityType {
    /// Creates a new builder-style object to manufacture [`ActivityType`](crate::types::ActivityType).
    pub fn builder() -> crate::types::builders::ActivityTypeBuilder {
        crate::types::builders::ActivityTypeBuilder::default()
    }
}

/// A builder for [`ActivityType`](crate::types::ActivityType).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ActivityTypeBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl ActivityTypeBuilder {
    /// <p>The name of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique within a domain.</p>
    /// </note>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique within a domain.</p>
    /// </note>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The version of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique with in a domain.</p>
    /// </note>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of this activity.</p> <note>
    /// <p>The combination of activity type name and version must be unique with in a domain.</p>
    /// </note>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`ActivityType`](crate::types::ActivityType).
    pub fn build(self) -> crate::types::ActivityType {
        crate::types::ActivityType {
            name: self.name,
            version: self.version,
        }
    }
}
