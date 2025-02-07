// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> One or more network interfaces to attach to an Amazon EC2 instance. If you specify a network interface, you must specify security groups and subnets as part of the network interface. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails {
    /// <p> Indicates whether to associate a Carrier IP address with eth0 for a new network interface. You use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Wavelength Developer Guide</i>. </p>
    #[doc(hidden)]
    pub associate_carrier_ip_address: bool,
    /// <p> Associates a public IPv4 address with eth0 for a new network interface. </p>
    #[doc(hidden)]
    pub associate_public_ip_address: bool,
    /// <p> Indicates whether the network interface is deleted when the instance is terminated. </p>
    #[doc(hidden)]
    pub delete_on_termination: bool,
    /// <p> A description for the network interface. </p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p> The device index for the network interface attachment. </p>
    #[doc(hidden)]
    pub device_index: i32,
    /// <p> The IDs of one or more security groups. </p>
    #[doc(hidden)]
    pub groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p> The type of network interface. </p>
    #[doc(hidden)]
    pub interface_type: ::std::option::Option<::std::string::String>,
    /// <p> The number of IPv4 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefixes</code> option. </p>
    #[doc(hidden)]
    pub ipv4_prefix_count: i32,
    /// <p> One or more IPv4 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option. </p>
    #[doc(hidden)]
    pub ipv4_prefixes: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails,
        >,
    >,
    /// <p> The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range. You can't use this option if you use <code>Ipv6Addresses</code>. </p>
    #[doc(hidden)]
    pub ipv6_address_count: i32,
    /// <p> One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You can't use this option if you use <code>Ipv6AddressCount</code>. </p>
    #[doc(hidden)]
    pub ipv6_addresses: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails,
        >,
    >,
    /// <p> The number of IPv6 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option. </p>
    #[doc(hidden)]
    pub ipv6_prefix_count: i32,
    /// <p> One or more IPv6 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option. </p>
    #[doc(hidden)]
    pub ipv6_prefixes: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails,
        >,
    >,
    /// <p> The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index <code>0</code>. The default is network card index <code>0</code>. </p>
    #[doc(hidden)]
    pub network_card_index: i32,
    /// <p> The ID of the network interface. </p>
    #[doc(hidden)]
    pub network_interface_id: ::std::option::Option<::std::string::String>,
    /// <p> The primary private IPv4 address of the network interface. </p>
    #[doc(hidden)]
    pub private_ip_address: ::std::option::Option<::std::string::String>,
    /// <p> One or more private IPv4 addresses. </p>
    #[doc(hidden)]
    pub private_ip_addresses: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails,
        >,
    >,
    /// <p> The number of secondary private IPv4 addresses to assign to a network interface. </p>
    #[doc(hidden)]
    pub secondary_private_ip_address_count: i32,
    /// <p> The ID of the subnet for the network interface. </p>
    #[doc(hidden)]
    pub subnet_id: ::std::option::Option<::std::string::String>,
}
impl AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails {
    /// <p> Indicates whether to associate a Carrier IP address with eth0 for a new network interface. You use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Wavelength Developer Guide</i>. </p>
    pub fn associate_carrier_ip_address(&self) -> bool {
        self.associate_carrier_ip_address
    }
    /// <p> Associates a public IPv4 address with eth0 for a new network interface. </p>
    pub fn associate_public_ip_address(&self) -> bool {
        self.associate_public_ip_address
    }
    /// <p> Indicates whether the network interface is deleted when the instance is terminated. </p>
    pub fn delete_on_termination(&self) -> bool {
        self.delete_on_termination
    }
    /// <p> A description for the network interface. </p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p> The device index for the network interface attachment. </p>
    pub fn device_index(&self) -> i32 {
        self.device_index
    }
    /// <p> The IDs of one or more security groups. </p>
    pub fn groups(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.groups.as_deref()
    }
    /// <p> The type of network interface. </p>
    pub fn interface_type(&self) -> ::std::option::Option<&str> {
        self.interface_type.as_deref()
    }
    /// <p> The number of IPv4 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefixes</code> option. </p>
    pub fn ipv4_prefix_count(&self) -> i32 {
        self.ipv4_prefix_count
    }
    /// <p> One or more IPv4 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option. </p>
    pub fn ipv4_prefixes(
        &self,
    ) -> ::std::option::Option<
        &[crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails],
    > {
        self.ipv4_prefixes.as_deref()
    }
    /// <p> The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range. You can't use this option if you use <code>Ipv6Addresses</code>. </p>
    pub fn ipv6_address_count(&self) -> i32 {
        self.ipv6_address_count
    }
    /// <p> One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You can't use this option if you use <code>Ipv6AddressCount</code>. </p>
    pub fn ipv6_addresses(
        &self,
    ) -> ::std::option::Option<
        &[crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails],
    > {
        self.ipv6_addresses.as_deref()
    }
    /// <p> The number of IPv6 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option. </p>
    pub fn ipv6_prefix_count(&self) -> i32 {
        self.ipv6_prefix_count
    }
    /// <p> One or more IPv6 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option. </p>
    pub fn ipv6_prefixes(
        &self,
    ) -> ::std::option::Option<
        &[crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails],
    > {
        self.ipv6_prefixes.as_deref()
    }
    /// <p> The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index <code>0</code>. The default is network card index <code>0</code>. </p>
    pub fn network_card_index(&self) -> i32 {
        self.network_card_index
    }
    /// <p> The ID of the network interface. </p>
    pub fn network_interface_id(&self) -> ::std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p> The primary private IPv4 address of the network interface. </p>
    pub fn private_ip_address(&self) -> ::std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
    /// <p> One or more private IPv4 addresses. </p>
    pub fn private_ip_addresses(
        &self,
    ) -> ::std::option::Option<
        &[crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails],
    > {
        self.private_ip_addresses.as_deref()
    }
    /// <p> The number of secondary private IPv4 addresses to assign to a network interface. </p>
    pub fn secondary_private_ip_address_count(&self) -> i32 {
        self.secondary_private_ip_address_count
    }
    /// <p> The ID of the subnet for the network interface. </p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
}
impl AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails {
    /// Creates a new builder-style object to manufacture [`AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails`](crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails).
    pub fn builder(
    ) -> crate::types::builders::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetailsBuilder {
        crate::types::builders::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetailsBuilder::default()
    }
}

/// A builder for [`AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails`](crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetDetailsBuilder {
    pub(crate) associate_carrier_ip_address: ::std::option::Option<bool>,
    pub(crate) associate_public_ip_address: ::std::option::Option<bool>,
    pub(crate) delete_on_termination: ::std::option::Option<bool>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) device_index: ::std::option::Option<i32>,
    pub(crate) groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) interface_type: ::std::option::Option<::std::string::String>,
    pub(crate) ipv4_prefix_count: ::std::option::Option<i32>,
    pub(crate) ipv4_prefixes: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails,
        >,
    >,
    pub(crate) ipv6_address_count: ::std::option::Option<i32>,
    pub(crate) ipv6_addresses: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails,
        >,
    >,
    pub(crate) ipv6_prefix_count: ::std::option::Option<i32>,
    pub(crate) ipv6_prefixes: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails,
        >,
    >,
    pub(crate) network_card_index: ::std::option::Option<i32>,
    pub(crate) network_interface_id: ::std::option::Option<::std::string::String>,
    pub(crate) private_ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) private_ip_addresses: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails,
        >,
    >,
    pub(crate) secondary_private_ip_address_count: ::std::option::Option<i32>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
}
impl AwsEc2LaunchTemplateDataNetworkInterfaceSetDetailsBuilder {
    /// <p> Indicates whether to associate a Carrier IP address with eth0 for a new network interface. You use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Wavelength Developer Guide</i>. </p>
    pub fn associate_carrier_ip_address(mut self, input: bool) -> Self {
        self.associate_carrier_ip_address = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates whether to associate a Carrier IP address with eth0 for a new network interface. You use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Wavelength Developer Guide</i>. </p>
    pub fn set_associate_carrier_ip_address(mut self, input: ::std::option::Option<bool>) -> Self {
        self.associate_carrier_ip_address = input;
        self
    }
    /// <p> Associates a public IPv4 address with eth0 for a new network interface. </p>
    pub fn associate_public_ip_address(mut self, input: bool) -> Self {
        self.associate_public_ip_address = ::std::option::Option::Some(input);
        self
    }
    /// <p> Associates a public IPv4 address with eth0 for a new network interface. </p>
    pub fn set_associate_public_ip_address(mut self, input: ::std::option::Option<bool>) -> Self {
        self.associate_public_ip_address = input;
        self
    }
    /// <p> Indicates whether the network interface is deleted when the instance is terminated. </p>
    pub fn delete_on_termination(mut self, input: bool) -> Self {
        self.delete_on_termination = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates whether the network interface is deleted when the instance is terminated. </p>
    pub fn set_delete_on_termination(mut self, input: ::std::option::Option<bool>) -> Self {
        self.delete_on_termination = input;
        self
    }
    /// <p> A description for the network interface. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> A description for the network interface. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p> The device index for the network interface attachment. </p>
    pub fn device_index(mut self, input: i32) -> Self {
        self.device_index = ::std::option::Option::Some(input);
        self
    }
    /// <p> The device index for the network interface attachment. </p>
    pub fn set_device_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.device_index = input;
        self
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p> The IDs of one or more security groups. </p>
    pub fn groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input.into());
        self.groups = ::std::option::Option::Some(v);
        self
    }
    /// <p> The IDs of one or more security groups. </p>
    pub fn set_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// <p> The type of network interface. </p>
    pub fn interface_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.interface_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The type of network interface. </p>
    pub fn set_interface_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.interface_type = input;
        self
    }
    /// <p> The number of IPv4 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefixes</code> option. </p>
    pub fn ipv4_prefix_count(mut self, input: i32) -> Self {
        self.ipv4_prefix_count = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of IPv4 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefixes</code> option. </p>
    pub fn set_ipv4_prefix_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ipv4_prefix_count = input;
        self
    }
    /// Appends an item to `ipv4_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv4_prefixes`](Self::set_ipv4_prefixes).
    ///
    /// <p> One or more IPv4 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option. </p>
    pub fn ipv4_prefixes(
        mut self,
        input: crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails,
    ) -> Self {
        let mut v = self.ipv4_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv4_prefixes = ::std::option::Option::Some(v);
        self
    }
    /// <p> One or more IPv4 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option. </p>
    pub fn set_ipv4_prefixes(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails,
            >,
        >,
    ) -> Self {
        self.ipv4_prefixes = input;
        self
    }
    /// <p> The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range. You can't use this option if you use <code>Ipv6Addresses</code>. </p>
    pub fn ipv6_address_count(mut self, input: i32) -> Self {
        self.ipv6_address_count = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range. You can't use this option if you use <code>Ipv6Addresses</code>. </p>
    pub fn set_ipv6_address_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ipv6_address_count = input;
        self
    }
    /// Appends an item to `ipv6_addresses`.
    ///
    /// To override the contents of this collection use [`set_ipv6_addresses`](Self::set_ipv6_addresses).
    ///
    /// <p> One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You can't use this option if you use <code>Ipv6AddressCount</code>. </p>
    pub fn ipv6_addresses(
        mut self,
        input: crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails,
    ) -> Self {
        let mut v = self.ipv6_addresses.unwrap_or_default();
        v.push(input);
        self.ipv6_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p> One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. You can't use this option if you use <code>Ipv6AddressCount</code>. </p>
    pub fn set_ipv6_addresses(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails,
            >,
        >,
    ) -> Self {
        self.ipv6_addresses = input;
        self
    }
    /// <p> The number of IPv6 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option. </p>
    pub fn ipv6_prefix_count(mut self, input: i32) -> Self {
        self.ipv6_prefix_count = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of IPv6 prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option. </p>
    pub fn set_ipv6_prefix_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ipv6_prefix_count = input;
        self
    }
    /// Appends an item to `ipv6_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv6_prefixes`](Self::set_ipv6_prefixes).
    ///
    /// <p> One or more IPv6 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option. </p>
    pub fn ipv6_prefixes(
        mut self,
        input: crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails,
    ) -> Self {
        let mut v = self.ipv6_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv6_prefixes = ::std::option::Option::Some(v);
        self
    }
    /// <p> One or more IPv6 prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option. </p>
    pub fn set_ipv6_prefixes(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails,
            >,
        >,
    ) -> Self {
        self.ipv6_prefixes = input;
        self
    }
    /// <p> The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index <code>0</code>. The default is network card index <code>0</code>. </p>
    pub fn network_card_index(mut self, input: i32) -> Self {
        self.network_card_index = ::std::option::Option::Some(input);
        self
    }
    /// <p> The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index <code>0</code>. The default is network card index <code>0</code>. </p>
    pub fn set_network_card_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.network_card_index = input;
        self
    }
    /// <p> The ID of the network interface. </p>
    pub fn network_interface_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.network_interface_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of the network interface. </p>
    pub fn set_network_interface_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p> The primary private IPv4 address of the network interface. </p>
    pub fn private_ip_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.private_ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The primary private IPv4 address of the network interface. </p>
    pub fn set_private_ip_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.private_ip_address = input;
        self
    }
    /// Appends an item to `private_ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p> One or more private IPv4 addresses. </p>
    pub fn private_ip_addresses(
        mut self,
        input: crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails,
    ) -> Self {
        let mut v = self.private_ip_addresses.unwrap_or_default();
        v.push(input);
        self.private_ip_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p> One or more private IPv4 addresses. </p>
    pub fn set_private_ip_addresses(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails,
            >,
        >,
    ) -> Self {
        self.private_ip_addresses = input;
        self
    }
    /// <p> The number of secondary private IPv4 addresses to assign to a network interface. </p>
    pub fn secondary_private_ip_address_count(mut self, input: i32) -> Self {
        self.secondary_private_ip_address_count = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of secondary private IPv4 addresses to assign to a network interface. </p>
    pub fn set_secondary_private_ip_address_count(
        mut self,
        input: ::std::option::Option<i32>,
    ) -> Self {
        self.secondary_private_ip_address_count = input;
        self
    }
    /// <p> The ID of the subnet for the network interface. </p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of the subnet for the network interface. </p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails`](crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails).
    pub fn build(self) -> crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails {
        crate::types::AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails {
            associate_carrier_ip_address: self.associate_carrier_ip_address.unwrap_or_default(),
            associate_public_ip_address: self.associate_public_ip_address.unwrap_or_default(),
            delete_on_termination: self.delete_on_termination.unwrap_or_default(),
            description: self.description,
            device_index: self.device_index.unwrap_or_default(),
            groups: self.groups,
            interface_type: self.interface_type,
            ipv4_prefix_count: self.ipv4_prefix_count.unwrap_or_default(),
            ipv4_prefixes: self.ipv4_prefixes,
            ipv6_address_count: self.ipv6_address_count.unwrap_or_default(),
            ipv6_addresses: self.ipv6_addresses,
            ipv6_prefix_count: self.ipv6_prefix_count.unwrap_or_default(),
            ipv6_prefixes: self.ipv6_prefixes,
            network_card_index: self.network_card_index.unwrap_or_default(),
            network_interface_id: self.network_interface_id,
            private_ip_address: self.private_ip_address,
            private_ip_addresses: self.private_ip_addresses,
            secondary_private_ip_address_count: self
                .secondary_private_ip_address_count
                .unwrap_or_default(),
            subnet_id: self.subnet_id,
        }
    }
}
