// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemoveIpRoutesInput {
    /// <p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>
    #[doc(hidden)]
    pub directory_id: ::std::option::Option<::std::string::String>,
    /// <p>IP address blocks that you want to remove.</p>
    #[doc(hidden)]
    pub cidr_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RemoveIpRoutesInput {
    /// <p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
    /// <p>IP address blocks that you want to remove.</p>
    pub fn cidr_ips(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.cidr_ips.as_deref()
    }
}
impl RemoveIpRoutesInput {
    /// Creates a new builder-style object to manufacture [`RemoveIpRoutesInput`](crate::operation::remove_ip_routes::RemoveIpRoutesInput).
    pub fn builder() -> crate::operation::remove_ip_routes::builders::RemoveIpRoutesInputBuilder {
        crate::operation::remove_ip_routes::builders::RemoveIpRoutesInputBuilder::default()
    }
}

/// A builder for [`RemoveIpRoutesInput`](crate::operation::remove_ip_routes::RemoveIpRoutesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RemoveIpRoutesInputBuilder {
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
    pub(crate) cidr_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RemoveIpRoutesInputBuilder {
    /// <p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    /// Appends an item to `cidr_ips`.
    ///
    /// To override the contents of this collection use [`set_cidr_ips`](Self::set_cidr_ips).
    ///
    /// <p>IP address blocks that you want to remove.</p>
    pub fn cidr_ips(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.cidr_ips.unwrap_or_default();
        v.push(input.into());
        self.cidr_ips = ::std::option::Option::Some(v);
        self
    }
    /// <p>IP address blocks that you want to remove.</p>
    pub fn set_cidr_ips(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.cidr_ips = input;
        self
    }
    /// Consumes the builder and constructs a [`RemoveIpRoutesInput`](crate::operation::remove_ip_routes::RemoveIpRoutesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::remove_ip_routes::RemoveIpRoutesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::remove_ip_routes::RemoveIpRoutesInput {
            directory_id: self.directory_id,
            cidr_ips: self.cidr_ips,
        })
    }
}
