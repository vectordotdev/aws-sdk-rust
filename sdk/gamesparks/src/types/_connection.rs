// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about a WebSocket connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Connection {
    /// <p>The identifier used to indicate a specific WebSocket connection.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the connection was created.</p>
    #[doc(hidden)]
    pub created: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl Connection {
    /// <p>The identifier used to indicate a specific WebSocket connection.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The date and time when the connection was created.</p>
    pub fn created(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created.as_ref()
    }
}
impl Connection {
    /// Creates a new builder-style object to manufacture [`Connection`](crate::types::Connection).
    pub fn builder() -> crate::types::builders::ConnectionBuilder {
        crate::types::builders::ConnectionBuilder::default()
    }
}

/// A builder for [`Connection`](crate::types::Connection).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConnectionBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) created: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ConnectionBuilder {
    /// <p>The identifier used to indicate a specific WebSocket connection.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier used to indicate a specific WebSocket connection.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The date and time when the connection was created.</p>
    pub fn created(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the connection was created.</p>
    pub fn set_created(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created = input;
        self
    }
    /// Consumes the builder and constructs a [`Connection`](crate::types::Connection).
    pub fn build(self) -> crate::types::Connection {
        crate::types::Connection {
            id: self.id,
            created: self.created,
        }
    }
}
