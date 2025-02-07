// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes VPC configuration information for fleets and image builders.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcConfig {
    /// <p>The identifiers of the subnets to which a network interface is attached from the fleet instance or image builder instance. Fleet instances use one or more subnets. Image builder instances use one subnet.</p>
    #[doc(hidden)]
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The identifiers of the security groups for the fleet or image builder.</p>
    #[doc(hidden)]
    pub security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl VpcConfig {
    /// <p>The identifiers of the subnets to which a network interface is attached from the fleet instance or image builder instance. Fleet instances use one or more subnets. Image builder instances use one subnet.</p>
    pub fn subnet_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>The identifiers of the security groups for the fleet or image builder.</p>
    pub fn security_group_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.security_group_ids.as_deref()
    }
}
impl VpcConfig {
    /// Creates a new builder-style object to manufacture [`VpcConfig`](crate::types::VpcConfig).
    pub fn builder() -> crate::types::builders::VpcConfigBuilder {
        crate::types::builders::VpcConfigBuilder::default()
    }
}

/// A builder for [`VpcConfig`](crate::types::VpcConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VpcConfigBuilder {
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl VpcConfigBuilder {
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The identifiers of the subnets to which a network interface is attached from the fleet instance or image builder instance. Fleet instances use one or more subnets. Image builder instances use one subnet.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The identifiers of the subnets to which a network interface is attached from the fleet instance or image builder instance. Fleet instances use one or more subnets. Image builder instances use one subnet.</p>
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
    /// <p>The identifiers of the security groups for the fleet or image builder.</p>
    pub fn security_group_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The identifiers of the security groups for the fleet or image builder.</p>
    pub fn set_security_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.security_group_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcConfig`](crate::types::VpcConfig).
    pub fn build(self) -> crate::types::VpcConfig {
        crate::types::VpcConfig {
            subnet_ids: self.subnet_ids,
            security_group_ids: self.security_group_ids,
        }
    }
}
