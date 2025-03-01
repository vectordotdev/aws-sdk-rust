// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a listener for a virtual gateway.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualGatewayListener {
    /// <p>The health check information for the listener.</p>
    #[doc(hidden)]
    pub health_check: ::std::option::Option<crate::types::VirtualGatewayHealthCheckPolicy>,
    /// <p>The port mapping information for the listener.</p>
    #[doc(hidden)]
    pub port_mapping: ::std::option::Option<crate::types::VirtualGatewayPortMapping>,
    /// <p>A reference to an object that represents the Transport Layer Security (TLS) properties for the listener.</p>
    #[doc(hidden)]
    pub tls: ::std::option::Option<crate::types::VirtualGatewayListenerTls>,
    /// <p>The connection pool information for the virtual gateway listener.</p>
    #[doc(hidden)]
    pub connection_pool: ::std::option::Option<crate::types::VirtualGatewayConnectionPool>,
}
impl VirtualGatewayListener {
    /// <p>The health check information for the listener.</p>
    pub fn health_check(
        &self,
    ) -> ::std::option::Option<&crate::types::VirtualGatewayHealthCheckPolicy> {
        self.health_check.as_ref()
    }
    /// <p>The port mapping information for the listener.</p>
    pub fn port_mapping(&self) -> ::std::option::Option<&crate::types::VirtualGatewayPortMapping> {
        self.port_mapping.as_ref()
    }
    /// <p>A reference to an object that represents the Transport Layer Security (TLS) properties for the listener.</p>
    pub fn tls(&self) -> ::std::option::Option<&crate::types::VirtualGatewayListenerTls> {
        self.tls.as_ref()
    }
    /// <p>The connection pool information for the virtual gateway listener.</p>
    pub fn connection_pool(
        &self,
    ) -> ::std::option::Option<&crate::types::VirtualGatewayConnectionPool> {
        self.connection_pool.as_ref()
    }
}
impl VirtualGatewayListener {
    /// Creates a new builder-style object to manufacture [`VirtualGatewayListener`](crate::types::VirtualGatewayListener).
    pub fn builder() -> crate::types::builders::VirtualGatewayListenerBuilder {
        crate::types::builders::VirtualGatewayListenerBuilder::default()
    }
}

/// A builder for [`VirtualGatewayListener`](crate::types::VirtualGatewayListener).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualGatewayListenerBuilder {
    pub(crate) health_check: ::std::option::Option<crate::types::VirtualGatewayHealthCheckPolicy>,
    pub(crate) port_mapping: ::std::option::Option<crate::types::VirtualGatewayPortMapping>,
    pub(crate) tls: ::std::option::Option<crate::types::VirtualGatewayListenerTls>,
    pub(crate) connection_pool: ::std::option::Option<crate::types::VirtualGatewayConnectionPool>,
}
impl VirtualGatewayListenerBuilder {
    /// <p>The health check information for the listener.</p>
    pub fn health_check(mut self, input: crate::types::VirtualGatewayHealthCheckPolicy) -> Self {
        self.health_check = ::std::option::Option::Some(input);
        self
    }
    /// <p>The health check information for the listener.</p>
    pub fn set_health_check(
        mut self,
        input: ::std::option::Option<crate::types::VirtualGatewayHealthCheckPolicy>,
    ) -> Self {
        self.health_check = input;
        self
    }
    /// <p>The port mapping information for the listener.</p>
    pub fn port_mapping(mut self, input: crate::types::VirtualGatewayPortMapping) -> Self {
        self.port_mapping = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port mapping information for the listener.</p>
    pub fn set_port_mapping(
        mut self,
        input: ::std::option::Option<crate::types::VirtualGatewayPortMapping>,
    ) -> Self {
        self.port_mapping = input;
        self
    }
    /// <p>A reference to an object that represents the Transport Layer Security (TLS) properties for the listener.</p>
    pub fn tls(mut self, input: crate::types::VirtualGatewayListenerTls) -> Self {
        self.tls = ::std::option::Option::Some(input);
        self
    }
    /// <p>A reference to an object that represents the Transport Layer Security (TLS) properties for the listener.</p>
    pub fn set_tls(
        mut self,
        input: ::std::option::Option<crate::types::VirtualGatewayListenerTls>,
    ) -> Self {
        self.tls = input;
        self
    }
    /// <p>The connection pool information for the virtual gateway listener.</p>
    pub fn connection_pool(mut self, input: crate::types::VirtualGatewayConnectionPool) -> Self {
        self.connection_pool = ::std::option::Option::Some(input);
        self
    }
    /// <p>The connection pool information for the virtual gateway listener.</p>
    pub fn set_connection_pool(
        mut self,
        input: ::std::option::Option<crate::types::VirtualGatewayConnectionPool>,
    ) -> Self {
        self.connection_pool = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualGatewayListener`](crate::types::VirtualGatewayListener).
    pub fn build(self) -> crate::types::VirtualGatewayListener {
        crate::types::VirtualGatewayListener {
            health_check: self.health_check,
            port_mapping: self.port_mapping,
            tls: self.tls,
            connection_pool: self.connection_pool,
        }
    }
}
