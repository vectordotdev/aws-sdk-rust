// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Information about a device definition version.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeviceDefinitionVersion {
    /// A list of devices in the definition version.
    #[doc(hidden)]
    pub devices: ::std::option::Option<::std::vec::Vec<crate::types::Device>>,
}
impl DeviceDefinitionVersion {
    /// A list of devices in the definition version.
    pub fn devices(&self) -> ::std::option::Option<&[crate::types::Device]> {
        self.devices.as_deref()
    }
}
impl DeviceDefinitionVersion {
    /// Creates a new builder-style object to manufacture [`DeviceDefinitionVersion`](crate::types::DeviceDefinitionVersion).
    pub fn builder() -> crate::types::builders::DeviceDefinitionVersionBuilder {
        crate::types::builders::DeviceDefinitionVersionBuilder::default()
    }
}

/// A builder for [`DeviceDefinitionVersion`](crate::types::DeviceDefinitionVersion).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeviceDefinitionVersionBuilder {
    pub(crate) devices: ::std::option::Option<::std::vec::Vec<crate::types::Device>>,
}
impl DeviceDefinitionVersionBuilder {
    /// Appends an item to `devices`.
    ///
    /// To override the contents of this collection use [`set_devices`](Self::set_devices).
    ///
    /// A list of devices in the definition version.
    pub fn devices(mut self, input: crate::types::Device) -> Self {
        let mut v = self.devices.unwrap_or_default();
        v.push(input);
        self.devices = ::std::option::Option::Some(v);
        self
    }
    /// A list of devices in the definition version.
    pub fn set_devices(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Device>>,
    ) -> Self {
        self.devices = input;
        self
    }
    /// Consumes the builder and constructs a [`DeviceDefinitionVersion`](crate::types::DeviceDefinitionVersion).
    pub fn build(self) -> crate::types::DeviceDefinitionVersion {
        crate::types::DeviceDefinitionVersion {
            devices: self.devices,
        }
    }
}
