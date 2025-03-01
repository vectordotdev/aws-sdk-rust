// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains configuration information about the Amazon Virtual Private Cloud (VPC).</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcConfiguration {
    /// <p>An array of strings containing the Amazon VPC subnet IDs (e.g., <code>subnet-0bb1c79de3EXAMPLE</code>.</p>
    #[doc(hidden)]
    pub subnet_id_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An array of strings containing the list of security groups.</p>
    #[doc(hidden)]
    pub security_group_id_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl VpcConfiguration {
    /// <p>An array of strings containing the Amazon VPC subnet IDs (e.g., <code>subnet-0bb1c79de3EXAMPLE</code>.</p>
    pub fn subnet_id_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnet_id_list.as_deref()
    }
    /// <p>An array of strings containing the list of security groups.</p>
    pub fn security_group_id_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.security_group_id_list.as_deref()
    }
}
impl VpcConfiguration {
    /// Creates a new builder-style object to manufacture [`VpcConfiguration`](crate::types::VpcConfiguration).
    pub fn builder() -> crate::types::builders::VpcConfigurationBuilder {
        crate::types::builders::VpcConfigurationBuilder::default()
    }
}

/// A builder for [`VpcConfiguration`](crate::types::VpcConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VpcConfigurationBuilder {
    pub(crate) subnet_id_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_group_id_list:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl VpcConfigurationBuilder {
    /// Appends an item to `subnet_id_list`.
    ///
    /// To override the contents of this collection use [`set_subnet_id_list`](Self::set_subnet_id_list).
    ///
    /// <p>An array of strings containing the Amazon VPC subnet IDs (e.g., <code>subnet-0bb1c79de3EXAMPLE</code>.</p>
    pub fn subnet_id_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.subnet_id_list.unwrap_or_default();
        v.push(input.into());
        self.subnet_id_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of strings containing the Amazon VPC subnet IDs (e.g., <code>subnet-0bb1c79de3EXAMPLE</code>.</p>
    pub fn set_subnet_id_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnet_id_list = input;
        self
    }
    /// Appends an item to `security_group_id_list`.
    ///
    /// To override the contents of this collection use [`set_security_group_id_list`](Self::set_security_group_id_list).
    ///
    /// <p>An array of strings containing the list of security groups.</p>
    pub fn security_group_id_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.security_group_id_list.unwrap_or_default();
        v.push(input.into());
        self.security_group_id_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of strings containing the list of security groups.</p>
    pub fn set_security_group_id_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.security_group_id_list = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcConfiguration`](crate::types::VpcConfiguration).
    pub fn build(self) -> crate::types::VpcConfiguration {
        crate::types::VpcConfiguration {
            subnet_id_list: self.subnet_id_list,
            security_group_id_list: self.security_group_id_list,
        }
    }
}
