// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDeviceInput {
    /// <p>The ID of the global network.</p>
    #[doc(hidden)]
    pub global_network_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the device.</p>
    #[doc(hidden)]
    pub device_id: ::std::option::Option<::std::string::String>,
}
impl DeleteDeviceInput {
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(&self) -> ::std::option::Option<&str> {
        self.global_network_id.as_deref()
    }
    /// <p>The ID of the device.</p>
    pub fn device_id(&self) -> ::std::option::Option<&str> {
        self.device_id.as_deref()
    }
}
impl DeleteDeviceInput {
    /// Creates a new builder-style object to manufacture [`DeleteDeviceInput`](crate::operation::delete_device::DeleteDeviceInput).
    pub fn builder() -> crate::operation::delete_device::builders::DeleteDeviceInputBuilder {
        crate::operation::delete_device::builders::DeleteDeviceInputBuilder::default()
    }
}

/// A builder for [`DeleteDeviceInput`](crate::operation::delete_device::DeleteDeviceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteDeviceInputBuilder {
    pub(crate) global_network_id: ::std::option::Option<::std::string::String>,
    pub(crate) device_id: ::std::option::Option<::std::string::String>,
}
impl DeleteDeviceInputBuilder {
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.global_network_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.global_network_id = input;
        self
    }
    /// <p>The ID of the device.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the device.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDeviceInput`](crate::operation::delete_device::DeleteDeviceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_device::DeleteDeviceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_device::DeleteDeviceInput {
            global_network_id: self.global_network_id,
            device_id: self.device_id,
        })
    }
}
