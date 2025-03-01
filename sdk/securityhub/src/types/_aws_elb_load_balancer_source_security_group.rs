// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the security group for the load balancer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsElbLoadBalancerSourceSecurityGroup {
    /// <p>The name of the security group.</p>
    #[doc(hidden)]
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The owner of the security group.</p>
    #[doc(hidden)]
    pub owner_alias: ::std::option::Option<::std::string::String>,
}
impl AwsElbLoadBalancerSourceSecurityGroup {
    /// <p>The name of the security group.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The owner of the security group.</p>
    pub fn owner_alias(&self) -> ::std::option::Option<&str> {
        self.owner_alias.as_deref()
    }
}
impl AwsElbLoadBalancerSourceSecurityGroup {
    /// Creates a new builder-style object to manufacture [`AwsElbLoadBalancerSourceSecurityGroup`](crate::types::AwsElbLoadBalancerSourceSecurityGroup).
    pub fn builder() -> crate::types::builders::AwsElbLoadBalancerSourceSecurityGroupBuilder {
        crate::types::builders::AwsElbLoadBalancerSourceSecurityGroupBuilder::default()
    }
}

/// A builder for [`AwsElbLoadBalancerSourceSecurityGroup`](crate::types::AwsElbLoadBalancerSourceSecurityGroup).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsElbLoadBalancerSourceSecurityGroupBuilder {
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) owner_alias: ::std::option::Option<::std::string::String>,
}
impl AwsElbLoadBalancerSourceSecurityGroupBuilder {
    /// <p>The name of the security group.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the security group.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The owner of the security group.</p>
    pub fn owner_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The owner of the security group.</p>
    pub fn set_owner_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_alias = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsElbLoadBalancerSourceSecurityGroup`](crate::types::AwsElbLoadBalancerSourceSecurityGroup).
    pub fn build(self) -> crate::types::AwsElbLoadBalancerSourceSecurityGroup {
        crate::types::AwsElbLoadBalancerSourceSecurityGroup {
            group_name: self.group_name,
            owner_alias: self.owner_alias,
        }
    }
}
