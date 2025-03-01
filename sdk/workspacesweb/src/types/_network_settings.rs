// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A network settings resource that can be associated with a web portal. Once associated with a web portal, network settings define how streaming instances will connect with your specified VPC. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NetworkSettings {
    /// <p>The ARN of the network settings.</p>
    #[doc(hidden)]
    pub network_settings_arn: ::std::option::Option<::std::string::String>,
    /// <p>A list of web portal ARNs that this network settings is associated with.</p>
    #[doc(hidden)]
    pub associated_portal_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The VPC that streaming instances will connect to.</p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The subnets in which network interfaces are created to connect streaming instances to your VPC. At least two of these subnets must be in different availability zones.</p>
    #[doc(hidden)]
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>One or more security groups used to control access from streaming instances to your VPC. </p>
    #[doc(hidden)]
    pub security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl NetworkSettings {
    /// <p>The ARN of the network settings.</p>
    pub fn network_settings_arn(&self) -> ::std::option::Option<&str> {
        self.network_settings_arn.as_deref()
    }
    /// <p>A list of web portal ARNs that this network settings is associated with.</p>
    pub fn associated_portal_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.associated_portal_arns.as_deref()
    }
    /// <p>The VPC that streaming instances will connect to.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The subnets in which network interfaces are created to connect streaming instances to your VPC. At least two of these subnets must be in different availability zones.</p>
    pub fn subnet_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>One or more security groups used to control access from streaming instances to your VPC. </p>
    pub fn security_group_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.security_group_ids.as_deref()
    }
}
impl NetworkSettings {
    /// Creates a new builder-style object to manufacture [`NetworkSettings`](crate::types::NetworkSettings).
    pub fn builder() -> crate::types::builders::NetworkSettingsBuilder {
        crate::types::builders::NetworkSettingsBuilder::default()
    }
}

/// A builder for [`NetworkSettings`](crate::types::NetworkSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NetworkSettingsBuilder {
    pub(crate) network_settings_arn: ::std::option::Option<::std::string::String>,
    pub(crate) associated_portal_arns:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl NetworkSettingsBuilder {
    /// <p>The ARN of the network settings.</p>
    pub fn network_settings_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.network_settings_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the network settings.</p>
    pub fn set_network_settings_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.network_settings_arn = input;
        self
    }
    /// Appends an item to `associated_portal_arns`.
    ///
    /// To override the contents of this collection use [`set_associated_portal_arns`](Self::set_associated_portal_arns).
    ///
    /// <p>A list of web portal ARNs that this network settings is associated with.</p>
    pub fn associated_portal_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.associated_portal_arns.unwrap_or_default();
        v.push(input.into());
        self.associated_portal_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of web portal ARNs that this network settings is associated with.</p>
    pub fn set_associated_portal_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.associated_portal_arns = input;
        self
    }
    /// <p>The VPC that streaming instances will connect to.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The VPC that streaming instances will connect to.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The subnets in which network interfaces are created to connect streaming instances to your VPC. At least two of these subnets must be in different availability zones.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The subnets in which network interfaces are created to connect streaming instances to your VPC. At least two of these subnets must be in different availability zones.</p>
    pub fn set_subnet_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnet_ids = input;
        self
    }
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>One or more security groups used to control access from streaming instances to your VPC. </p>
    pub fn security_group_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more security groups used to control access from streaming instances to your VPC. </p>
    pub fn set_security_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.security_group_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`NetworkSettings`](crate::types::NetworkSettings).
    pub fn build(self) -> crate::types::NetworkSettings {
        crate::types::NetworkSettings {
            network_settings_arn: self.network_settings_arn,
            associated_portal_arns: self.associated_portal_arns,
            vpc_id: self.vpc_id,
            subnet_ids: self.subnet_ids,
            security_group_ids: self.security_group_ids,
        }
    }
}
