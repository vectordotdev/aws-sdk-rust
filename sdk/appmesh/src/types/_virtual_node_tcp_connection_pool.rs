// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a type of connection pool.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualNodeTcpConnectionPool {
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    #[doc(hidden)]
    pub max_connections: i32,
}
impl VirtualNodeTcpConnectionPool {
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    pub fn max_connections(&self) -> i32 {
        self.max_connections
    }
}
impl VirtualNodeTcpConnectionPool {
    /// Creates a new builder-style object to manufacture [`VirtualNodeTcpConnectionPool`](crate::types::VirtualNodeTcpConnectionPool).
    pub fn builder() -> crate::types::builders::VirtualNodeTcpConnectionPoolBuilder {
        crate::types::builders::VirtualNodeTcpConnectionPoolBuilder::default()
    }
}

/// A builder for [`VirtualNodeTcpConnectionPool`](crate::types::VirtualNodeTcpConnectionPool).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualNodeTcpConnectionPoolBuilder {
    pub(crate) max_connections: ::std::option::Option<i32>,
}
impl VirtualNodeTcpConnectionPoolBuilder {
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    pub fn max_connections(mut self, input: i32) -> Self {
        self.max_connections = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    pub fn set_max_connections(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_connections = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualNodeTcpConnectionPool`](crate::types::VirtualNodeTcpConnectionPool).
    pub fn build(self) -> crate::types::VirtualNodeTcpConnectionPool {
        crate::types::VirtualNodeTcpConnectionPool {
            max_connections: self.max_connections.unwrap_or_default(),
        }
    }
}
