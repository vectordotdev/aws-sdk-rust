// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterComputeInput {
    /// <p>A unique identifier for the fleet to register the compute to. You can use either the fleet ID or ARN value.</p>
    #[doc(hidden)]
    pub fleet_id: ::std::option::Option<::std::string::String>,
    /// <p>A descriptive label that is associated with the compute resource registered to your fleet.</p>
    #[doc(hidden)]
    pub compute_name: ::std::option::Option<::std::string::String>,
    /// <p>The path to the TLS certificate on your compute resource. The path and certificate are not validated by Amazon GameLift.</p>
    #[doc(hidden)]
    pub certificate_path: ::std::option::Option<::std::string::String>,
    /// <p>The DNS name of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    #[doc(hidden)]
    pub dns_name: ::std::option::Option<::std::string::String>,
    /// <p>The IP address of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    #[doc(hidden)]
    pub ip_address: ::std::option::Option<::std::string::String>,
    /// <p>The name of the custom location you added to the fleet you are registering this compute resource to.</p>
    #[doc(hidden)]
    pub location: ::std::option::Option<::std::string::String>,
}
impl RegisterComputeInput {
    /// <p>A unique identifier for the fleet to register the compute to. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(&self) -> ::std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p>A descriptive label that is associated with the compute resource registered to your fleet.</p>
    pub fn compute_name(&self) -> ::std::option::Option<&str> {
        self.compute_name.as_deref()
    }
    /// <p>The path to the TLS certificate on your compute resource. The path and certificate are not validated by Amazon GameLift.</p>
    pub fn certificate_path(&self) -> ::std::option::Option<&str> {
        self.certificate_path.as_deref()
    }
    /// <p>The DNS name of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    pub fn dns_name(&self) -> ::std::option::Option<&str> {
        self.dns_name.as_deref()
    }
    /// <p>The IP address of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    pub fn ip_address(&self) -> ::std::option::Option<&str> {
        self.ip_address.as_deref()
    }
    /// <p>The name of the custom location you added to the fleet you are registering this compute resource to.</p>
    pub fn location(&self) -> ::std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl RegisterComputeInput {
    /// Creates a new builder-style object to manufacture [`RegisterComputeInput`](crate::operation::register_compute::RegisterComputeInput).
    pub fn builder() -> crate::operation::register_compute::builders::RegisterComputeInputBuilder {
        crate::operation::register_compute::builders::RegisterComputeInputBuilder::default()
    }
}

/// A builder for [`RegisterComputeInput`](crate::operation::register_compute::RegisterComputeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RegisterComputeInputBuilder {
    pub(crate) fleet_id: ::std::option::Option<::std::string::String>,
    pub(crate) compute_name: ::std::option::Option<::std::string::String>,
    pub(crate) certificate_path: ::std::option::Option<::std::string::String>,
    pub(crate) dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) location: ::std::option::Option<::std::string::String>,
}
impl RegisterComputeInputBuilder {
    /// <p>A unique identifier for the fleet to register the compute to. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to register the compute to. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// <p>A descriptive label that is associated with the compute resource registered to your fleet.</p>
    pub fn compute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.compute_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A descriptive label that is associated with the compute resource registered to your fleet.</p>
    pub fn set_compute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.compute_name = input;
        self
    }
    /// <p>The path to the TLS certificate on your compute resource. The path and certificate are not validated by Amazon GameLift.</p>
    pub fn certificate_path(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.certificate_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to the TLS certificate on your compute resource. The path and certificate are not validated by Amazon GameLift.</p>
    pub fn set_certificate_path(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.certificate_path = input;
        self
    }
    /// <p>The DNS name of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    pub fn dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The DNS name of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    pub fn set_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dns_name = input;
        self
    }
    /// <p>The IP address of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    pub fn ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the compute resource. Amazon GameLift requires the DNS name or IP address to manage your compute resource.</p>
    pub fn set_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_address = input;
        self
    }
    /// <p>The name of the custom location you added to the fleet you are registering this compute resource to.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the custom location you added to the fleet you are registering this compute resource to.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// Consumes the builder and constructs a [`RegisterComputeInput`](crate::operation::register_compute::RegisterComputeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::register_compute::RegisterComputeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::register_compute::RegisterComputeInput {
            fleet_id: self.fleet_id,
            compute_name: self.compute_name,
            certificate_path: self.certificate_path,
            dns_name: self.dns_name,
            ip_address: self.ip_address,
            location: self.location,
        })
    }
}
