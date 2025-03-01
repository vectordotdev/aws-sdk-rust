// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container image used to create an Amazon Braket job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContainerImage {
    /// <p>The URI locating the container image.</p>
    #[doc(hidden)]
    pub uri: ::std::option::Option<::std::string::String>,
}
impl ContainerImage {
    /// <p>The URI locating the container image.</p>
    pub fn uri(&self) -> ::std::option::Option<&str> {
        self.uri.as_deref()
    }
}
impl ContainerImage {
    /// Creates a new builder-style object to manufacture [`ContainerImage`](crate::types::ContainerImage).
    pub fn builder() -> crate::types::builders::ContainerImageBuilder {
        crate::types::builders::ContainerImageBuilder::default()
    }
}

/// A builder for [`ContainerImage`](crate::types::ContainerImage).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ContainerImageBuilder {
    pub(crate) uri: ::std::option::Option<::std::string::String>,
}
impl ContainerImageBuilder {
    /// <p>The URI locating the container image.</p>
    pub fn uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URI locating the container image.</p>
    pub fn set_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.uri = input;
        self
    }
    /// Consumes the builder and constructs a [`ContainerImage`](crate::types::ContainerImage).
    pub fn build(self) -> crate::types::ContainerImage {
        crate::types::ContainerImage { uri: self.uri }
    }
}
