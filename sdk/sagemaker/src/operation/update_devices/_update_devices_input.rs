// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateDevicesInput {
    /// <p>The name of the fleet the devices belong to.</p>
    #[doc(hidden)]
    pub device_fleet_name: ::std::option::Option<::std::string::String>,
    /// <p>List of devices to register with Edge Manager agent.</p>
    #[doc(hidden)]
    pub devices: ::std::option::Option<::std::vec::Vec<crate::types::Device>>,
}
impl UpdateDevicesInput {
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn device_fleet_name(&self) -> ::std::option::Option<&str> {
        self.device_fleet_name.as_deref()
    }
    /// <p>List of devices to register with Edge Manager agent.</p>
    pub fn devices(&self) -> ::std::option::Option<&[crate::types::Device]> {
        self.devices.as_deref()
    }
}
impl UpdateDevicesInput {
    /// Creates a new builder-style object to manufacture [`UpdateDevicesInput`](crate::operation::update_devices::UpdateDevicesInput).
    pub fn builder() -> crate::operation::update_devices::builders::UpdateDevicesInputBuilder {
        crate::operation::update_devices::builders::UpdateDevicesInputBuilder::default()
    }
}

/// A builder for [`UpdateDevicesInput`](crate::operation::update_devices::UpdateDevicesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateDevicesInputBuilder {
    pub(crate) device_fleet_name: ::std::option::Option<::std::string::String>,
    pub(crate) devices: ::std::option::Option<::std::vec::Vec<crate::types::Device>>,
}
impl UpdateDevicesInputBuilder {
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
    /// Appends an item to `devices`.
    ///
    /// To override the contents of this collection use [`set_devices`](Self::set_devices).
    ///
    /// <p>List of devices to register with Edge Manager agent.</p>
    pub fn devices(mut self, input: crate::types::Device) -> Self {
        let mut v = self.devices.unwrap_or_default();
        v.push(input);
        self.devices = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of devices to register with Edge Manager agent.</p>
    pub fn set_devices(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Device>>,
    ) -> Self {
        self.devices = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateDevicesInput`](crate::operation::update_devices::UpdateDevicesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_devices::UpdateDevicesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_devices::UpdateDevicesInput {
            device_fleet_name: self.device_fleet_name,
            devices: self.devices,
        })
    }
}
