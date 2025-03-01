// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a service.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceInfo {
    /// <p>The service identifier.</p>
    #[doc(hidden)]
    pub service_code: ::std::option::Option<::std::string::String>,
    /// <p>The service name.</p>
    #[doc(hidden)]
    pub service_name: ::std::option::Option<::std::string::String>,
}
impl ServiceInfo {
    /// <p>The service identifier.</p>
    pub fn service_code(&self) -> ::std::option::Option<&str> {
        self.service_code.as_deref()
    }
    /// <p>The service name.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
}
impl ServiceInfo {
    /// Creates a new builder-style object to manufacture [`ServiceInfo`](crate::types::ServiceInfo).
    pub fn builder() -> crate::types::builders::ServiceInfoBuilder {
        crate::types::builders::ServiceInfoBuilder::default()
    }
}

/// A builder for [`ServiceInfo`](crate::types::ServiceInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ServiceInfoBuilder {
    pub(crate) service_code: ::std::option::Option<::std::string::String>,
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
}
impl ServiceInfoBuilder {
    /// <p>The service identifier.</p>
    pub fn service_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The service identifier.</p>
    pub fn set_service_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_code = input;
        self
    }
    /// <p>The service name.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The service name.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ServiceInfo`](crate::types::ServiceInfo).
    pub fn build(self) -> crate::types::ServiceInfo {
        crate::types::ServiceInfo {
            service_code: self.service_code,
            service_name: self.service_name,
        }
    }
}
