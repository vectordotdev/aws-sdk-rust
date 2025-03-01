// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object representing the networking details for a task or service.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsVpcConfiguration {
    /// <p>The IDs of the subnets associated with the task or service. There's a limit of 16 subnets that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified subnets must be from the same VPC.</p>
    /// </note>
    #[doc(hidden)]
    pub subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IDs of the security groups associated with the task or service. If you don't specify a security group, the default security group for the VPC is used. There's a limit of 5 security groups that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified security groups must be from the same VPC.</p>
    /// </note>
    #[doc(hidden)]
    pub security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Whether the task's elastic network interface receives a public IP address. The default value is <code>DISABLED</code>.</p>
    #[doc(hidden)]
    pub assign_public_ip: ::std::option::Option<crate::types::AssignPublicIp>,
}
impl AwsVpcConfiguration {
    /// <p>The IDs of the subnets associated with the task or service. There's a limit of 16 subnets that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified subnets must be from the same VPC.</p>
    /// </note>
    pub fn subnets(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnets.as_deref()
    }
    /// <p>The IDs of the security groups associated with the task or service. If you don't specify a security group, the default security group for the VPC is used. There's a limit of 5 security groups that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified security groups must be from the same VPC.</p>
    /// </note>
    pub fn security_groups(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.security_groups.as_deref()
    }
    /// <p>Whether the task's elastic network interface receives a public IP address. The default value is <code>DISABLED</code>.</p>
    pub fn assign_public_ip(&self) -> ::std::option::Option<&crate::types::AssignPublicIp> {
        self.assign_public_ip.as_ref()
    }
}
impl AwsVpcConfiguration {
    /// Creates a new builder-style object to manufacture [`AwsVpcConfiguration`](crate::types::AwsVpcConfiguration).
    pub fn builder() -> crate::types::builders::AwsVpcConfigurationBuilder {
        crate::types::builders::AwsVpcConfigurationBuilder::default()
    }
}

/// A builder for [`AwsVpcConfiguration`](crate::types::AwsVpcConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsVpcConfigurationBuilder {
    pub(crate) subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) assign_public_ip: ::std::option::Option<crate::types::AssignPublicIp>,
}
impl AwsVpcConfigurationBuilder {
    /// Appends an item to `subnets`.
    ///
    /// To override the contents of this collection use [`set_subnets`](Self::set_subnets).
    ///
    /// <p>The IDs of the subnets associated with the task or service. There's a limit of 16 subnets that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified subnets must be from the same VPC.</p>
    /// </note>
    pub fn subnets(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnets.unwrap_or_default();
        v.push(input.into());
        self.subnets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the subnets associated with the task or service. There's a limit of 16 subnets that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified subnets must be from the same VPC.</p>
    /// </note>
    pub fn set_subnets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnets = input;
        self
    }
    /// Appends an item to `security_groups`.
    ///
    /// To override the contents of this collection use [`set_security_groups`](Self::set_security_groups).
    ///
    /// <p>The IDs of the security groups associated with the task or service. If you don't specify a security group, the default security group for the VPC is used. There's a limit of 5 security groups that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified security groups must be from the same VPC.</p>
    /// </note>
    pub fn security_groups(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.security_groups.unwrap_or_default();
        v.push(input.into());
        self.security_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the security groups associated with the task or service. If you don't specify a security group, the default security group for the VPC is used. There's a limit of 5 security groups that can be specified per <code>AwsVpcConfiguration</code>.</p> <note>
    /// <p>All specified security groups must be from the same VPC.</p>
    /// </note>
    pub fn set_security_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.security_groups = input;
        self
    }
    /// <p>Whether the task's elastic network interface receives a public IP address. The default value is <code>DISABLED</code>.</p>
    pub fn assign_public_ip(mut self, input: crate::types::AssignPublicIp) -> Self {
        self.assign_public_ip = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether the task's elastic network interface receives a public IP address. The default value is <code>DISABLED</code>.</p>
    pub fn set_assign_public_ip(
        mut self,
        input: ::std::option::Option<crate::types::AssignPublicIp>,
    ) -> Self {
        self.assign_public_ip = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsVpcConfiguration`](crate::types::AwsVpcConfiguration).
    pub fn build(self) -> crate::types::AwsVpcConfiguration {
        crate::types::AwsVpcConfiguration {
            subnets: self.subnets,
            security_groups: self.security_groups,
            assign_public_ip: self.assign_public_ip,
        }
    }
}
