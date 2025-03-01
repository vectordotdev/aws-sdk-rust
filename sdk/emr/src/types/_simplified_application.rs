// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The returned release label application names or versions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SimplifiedApplication {
    /// <p>The returned release label application name. For example, <code>hadoop</code>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The returned release label application version. For example, <code>3.2.1</code>.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
}
impl SimplifiedApplication {
    /// <p>The returned release label application name. For example, <code>hadoop</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The returned release label application version. For example, <code>3.2.1</code>.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl SimplifiedApplication {
    /// Creates a new builder-style object to manufacture [`SimplifiedApplication`](crate::types::SimplifiedApplication).
    pub fn builder() -> crate::types::builders::SimplifiedApplicationBuilder {
        crate::types::builders::SimplifiedApplicationBuilder::default()
    }
}

/// A builder for [`SimplifiedApplication`](crate::types::SimplifiedApplication).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SimplifiedApplicationBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl SimplifiedApplicationBuilder {
    /// <p>The returned release label application name. For example, <code>hadoop</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The returned release label application name. For example, <code>hadoop</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The returned release label application version. For example, <code>3.2.1</code>.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The returned release label application version. For example, <code>3.2.1</code>.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`SimplifiedApplication`](crate::types::SimplifiedApplication).
    pub fn build(self) -> crate::types::SimplifiedApplication {
        crate::types::SimplifiedApplication {
            name: self.name,
            version: self.version,
        }
    }
}
