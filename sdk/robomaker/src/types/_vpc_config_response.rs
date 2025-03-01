// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>VPC configuration associated with your simulation job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcConfigResponse {
    /// <p>A list of subnet IDs associated with the simulation job.</p>
    #[doc(hidden)]
    pub subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of security group IDs associated with the simulation job.</p>
    #[doc(hidden)]
    pub security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The VPC ID associated with your simulation job.</p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>A boolean indicating if a public IP was assigned.</p>
    #[doc(hidden)]
    pub assign_public_ip: bool,
}
impl VpcConfigResponse {
    /// <p>A list of subnet IDs associated with the simulation job.</p>
    pub fn subnets(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnets.as_deref()
    }
    /// <p>A list of security group IDs associated with the simulation job.</p>
    pub fn security_groups(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.security_groups.as_deref()
    }
    /// <p>The VPC ID associated with your simulation job.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>A boolean indicating if a public IP was assigned.</p>
    pub fn assign_public_ip(&self) -> bool {
        self.assign_public_ip
    }
}
impl VpcConfigResponse {
    /// Creates a new builder-style object to manufacture [`VpcConfigResponse`](crate::types::VpcConfigResponse).
    pub fn builder() -> crate::types::builders::VpcConfigResponseBuilder {
        crate::types::builders::VpcConfigResponseBuilder::default()
    }
}

/// A builder for [`VpcConfigResponse`](crate::types::VpcConfigResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VpcConfigResponseBuilder {
    pub(crate) subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) assign_public_ip: ::std::option::Option<bool>,
}
impl VpcConfigResponseBuilder {
    /// Appends an item to `subnets`.
    ///
    /// To override the contents of this collection use [`set_subnets`](Self::set_subnets).
    ///
    /// <p>A list of subnet IDs associated with the simulation job.</p>
    pub fn subnets(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnets.unwrap_or_default();
        v.push(input.into());
        self.subnets = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of subnet IDs associated with the simulation job.</p>
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
    /// <p>A list of security group IDs associated with the simulation job.</p>
    pub fn security_groups(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.security_groups.unwrap_or_default();
        v.push(input.into());
        self.security_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of security group IDs associated with the simulation job.</p>
    pub fn set_security_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.security_groups = input;
        self
    }
    /// <p>The VPC ID associated with your simulation job.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The VPC ID associated with your simulation job.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>A boolean indicating if a public IP was assigned.</p>
    pub fn assign_public_ip(mut self, input: bool) -> Self {
        self.assign_public_ip = ::std::option::Option::Some(input);
        self
    }
    /// <p>A boolean indicating if a public IP was assigned.</p>
    pub fn set_assign_public_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.assign_public_ip = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcConfigResponse`](crate::types::VpcConfigResponse).
    pub fn build(self) -> crate::types::VpcConfigResponse {
        crate::types::VpcConfigResponse {
            subnets: self.subnets,
            security_groups: self.security_groups,
            vpc_id: self.vpc_id,
            assign_public_ip: self.assign_public_ip.unwrap_or_default(),
        }
    }
}
