// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the health check policy for a virtual node's listener.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HealthCheckPolicy {
    /// <p>The amount of time to wait when receiving a response from the health check, in milliseconds.</p>
    #[doc(hidden)]
    pub timeout_millis: ::std::option::Option<i64>,
    /// <p>The time period in milliseconds between each health check execution.</p>
    #[doc(hidden)]
    pub interval_millis: ::std::option::Option<i64>,
    /// <p>The protocol for the health check request. If you specify <code>grpc</code>, then your service must conform to the <a href="https://github.com/grpc/grpc/blob/master/doc/health-checking.md">GRPC Health Checking Protocol</a>.</p>
    #[doc(hidden)]
    pub protocol: ::std::option::Option<crate::types::PortProtocol>,
    /// <p>The destination port for the health check request. This port must match the port defined in the <code>PortMapping</code> for the listener.</p>
    #[doc(hidden)]
    pub port: i32,
    /// <p>The destination path for the health check request. This value is only used if the specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.</p>
    #[doc(hidden)]
    pub path: ::std::option::Option<::std::string::String>,
    /// <p>The number of consecutive successful health checks that must occur before declaring listener healthy.</p>
    #[doc(hidden)]
    pub healthy_threshold: i32,
    /// <p>The number of consecutive failed health checks that must occur before declaring a virtual node unhealthy. </p>
    #[doc(hidden)]
    pub unhealthy_threshold: i32,
}
impl HealthCheckPolicy {
    /// <p>The amount of time to wait when receiving a response from the health check, in milliseconds.</p>
    pub fn timeout_millis(&self) -> ::std::option::Option<i64> {
        self.timeout_millis
    }
    /// <p>The time period in milliseconds between each health check execution.</p>
    pub fn interval_millis(&self) -> ::std::option::Option<i64> {
        self.interval_millis
    }
    /// <p>The protocol for the health check request. If you specify <code>grpc</code>, then your service must conform to the <a href="https://github.com/grpc/grpc/blob/master/doc/health-checking.md">GRPC Health Checking Protocol</a>.</p>
    pub fn protocol(&self) -> ::std::option::Option<&crate::types::PortProtocol> {
        self.protocol.as_ref()
    }
    /// <p>The destination port for the health check request. This port must match the port defined in the <code>PortMapping</code> for the listener.</p>
    pub fn port(&self) -> i32 {
        self.port
    }
    /// <p>The destination path for the health check request. This value is only used if the specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
    /// <p>The number of consecutive successful health checks that must occur before declaring listener healthy.</p>
    pub fn healthy_threshold(&self) -> i32 {
        self.healthy_threshold
    }
    /// <p>The number of consecutive failed health checks that must occur before declaring a virtual node unhealthy. </p>
    pub fn unhealthy_threshold(&self) -> i32 {
        self.unhealthy_threshold
    }
}
impl HealthCheckPolicy {
    /// Creates a new builder-style object to manufacture [`HealthCheckPolicy`](crate::types::HealthCheckPolicy).
    pub fn builder() -> crate::types::builders::HealthCheckPolicyBuilder {
        crate::types::builders::HealthCheckPolicyBuilder::default()
    }
}

/// A builder for [`HealthCheckPolicy`](crate::types::HealthCheckPolicy).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HealthCheckPolicyBuilder {
    pub(crate) timeout_millis: ::std::option::Option<i64>,
    pub(crate) interval_millis: ::std::option::Option<i64>,
    pub(crate) protocol: ::std::option::Option<crate::types::PortProtocol>,
    pub(crate) port: ::std::option::Option<i32>,
    pub(crate) path: ::std::option::Option<::std::string::String>,
    pub(crate) healthy_threshold: ::std::option::Option<i32>,
    pub(crate) unhealthy_threshold: ::std::option::Option<i32>,
}
impl HealthCheckPolicyBuilder {
    /// <p>The amount of time to wait when receiving a response from the health check, in milliseconds.</p>
    pub fn timeout_millis(mut self, input: i64) -> Self {
        self.timeout_millis = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of time to wait when receiving a response from the health check, in milliseconds.</p>
    pub fn set_timeout_millis(mut self, input: ::std::option::Option<i64>) -> Self {
        self.timeout_millis = input;
        self
    }
    /// <p>The time period in milliseconds between each health check execution.</p>
    pub fn interval_millis(mut self, input: i64) -> Self {
        self.interval_millis = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time period in milliseconds between each health check execution.</p>
    pub fn set_interval_millis(mut self, input: ::std::option::Option<i64>) -> Self {
        self.interval_millis = input;
        self
    }
    /// <p>The protocol for the health check request. If you specify <code>grpc</code>, then your service must conform to the <a href="https://github.com/grpc/grpc/blob/master/doc/health-checking.md">GRPC Health Checking Protocol</a>.</p>
    pub fn protocol(mut self, input: crate::types::PortProtocol) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The protocol for the health check request. If you specify <code>grpc</code>, then your service must conform to the <a href="https://github.com/grpc/grpc/blob/master/doc/health-checking.md">GRPC Health Checking Protocol</a>.</p>
    pub fn set_protocol(
        mut self,
        input: ::std::option::Option<crate::types::PortProtocol>,
    ) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The destination port for the health check request. This port must match the port defined in the <code>PortMapping</code> for the listener.</p>
    pub fn port(mut self, input: i32) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination port for the health check request. This port must match the port defined in the <code>PortMapping</code> for the listener.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.port = input;
        self
    }
    /// <p>The destination path for the health check request. This value is only used if the specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination path for the health check request. This value is only used if the specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>The number of consecutive successful health checks that must occur before declaring listener healthy.</p>
    pub fn healthy_threshold(mut self, input: i32) -> Self {
        self.healthy_threshold = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of consecutive successful health checks that must occur before declaring listener healthy.</p>
    pub fn set_healthy_threshold(mut self, input: ::std::option::Option<i32>) -> Self {
        self.healthy_threshold = input;
        self
    }
    /// <p>The number of consecutive failed health checks that must occur before declaring a virtual node unhealthy. </p>
    pub fn unhealthy_threshold(mut self, input: i32) -> Self {
        self.unhealthy_threshold = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of consecutive failed health checks that must occur before declaring a virtual node unhealthy. </p>
    pub fn set_unhealthy_threshold(mut self, input: ::std::option::Option<i32>) -> Self {
        self.unhealthy_threshold = input;
        self
    }
    /// Consumes the builder and constructs a [`HealthCheckPolicy`](crate::types::HealthCheckPolicy).
    pub fn build(self) -> crate::types::HealthCheckPolicy {
        crate::types::HealthCheckPolicy {
            timeout_millis: self.timeout_millis,
            interval_millis: self.interval_millis,
            protocol: self.protocol,
            port: self.port.unwrap_or_default(),
            path: self.path,
            healthy_threshold: self.healthy_threshold.unwrap_or_default(),
            unhealthy_threshold: self.unhealthy_threshold.unwrap_or_default(),
        }
    }
}
