// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The connection properties of an outbound connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConnectionProperties {
    /// <p>The endpoint of the remote domain.</p>
    #[doc(hidden)]
    pub endpoint: ::std::option::Option<::std::string::String>,
}
impl ConnectionProperties {
    /// <p>The endpoint of the remote domain.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
}
impl ConnectionProperties {
    /// Creates a new builder-style object to manufacture [`ConnectionProperties`](crate::types::ConnectionProperties).
    pub fn builder() -> crate::types::builders::ConnectionPropertiesBuilder {
        crate::types::builders::ConnectionPropertiesBuilder::default()
    }
}

/// A builder for [`ConnectionProperties`](crate::types::ConnectionProperties).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConnectionPropertiesBuilder {
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
}
impl ConnectionPropertiesBuilder {
    /// <p>The endpoint of the remote domain.</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The endpoint of the remote domain.</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// Consumes the builder and constructs a [`ConnectionProperties`](crate::types::ConnectionProperties).
    pub fn build(self) -> crate::types::ConnectionProperties {
        crate::types::ConnectionProperties {
            endpoint: self.endpoint,
        }
    }
}
