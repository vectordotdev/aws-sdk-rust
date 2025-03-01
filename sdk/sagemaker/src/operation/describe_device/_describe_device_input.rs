// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeDeviceInput {
    /// <p>Next token of device description.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The unique ID of the device.</p>
    #[doc(hidden)]
    pub device_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the fleet the devices belong to.</p>
    #[doc(hidden)]
    pub device_fleet_name: ::std::option::Option<::std::string::String>,
}
impl DescribeDeviceInput {
    /// <p>Next token of device description.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The unique ID of the device.</p>
    pub fn device_name(&self) -> ::std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn device_fleet_name(&self) -> ::std::option::Option<&str> {
        self.device_fleet_name.as_deref()
    }
}
impl DescribeDeviceInput {
    /// Creates a new builder-style object to manufacture [`DescribeDeviceInput`](crate::operation::describe_device::DescribeDeviceInput).
    pub fn builder() -> crate::operation::describe_device::builders::DescribeDeviceInputBuilder {
        crate::operation::describe_device::builders::DescribeDeviceInputBuilder::default()
    }
}

/// A builder for [`DescribeDeviceInput`](crate::operation::describe_device::DescribeDeviceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeDeviceInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) device_name: ::std::option::Option<::std::string::String>,
    pub(crate) device_fleet_name: ::std::option::Option<::std::string::String>,
}
impl DescribeDeviceInputBuilder {
    /// <p>Next token of device description.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Next token of device description.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The unique ID of the device.</p>
    pub fn device_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of the device.</p>
    pub fn set_device_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_name = input;
        self
    }
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn device_fleet_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.device_fleet_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn set_device_fleet_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.device_fleet_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeDeviceInput`](crate::operation::describe_device::DescribeDeviceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_device::DescribeDeviceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_device::DescribeDeviceInput {
            next_token: self.next_token,
            device_name: self.device_name,
            device_fleet_name: self.device_fleet_name,
        })
    }
}
