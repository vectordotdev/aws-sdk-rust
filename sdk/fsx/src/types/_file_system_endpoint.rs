// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An Amazon FSx for NetApp ONTAP file system has two endpoints that are used to access data or to manage the file system using the NetApp ONTAP CLI, REST API, or NetApp SnapMirror. They are the <code>Management</code> and <code>Intercluster</code> endpoints.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FileSystemEndpoint {
    /// <p>The Domain Name Service (DNS) name for the file system. You can mount your file system using its DNS name.</p>
    #[doc(hidden)]
    pub dns_name: ::std::option::Option<::std::string::String>,
    /// <p>IP addresses of the file system endpoint.</p>
    #[doc(hidden)]
    pub ip_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FileSystemEndpoint {
    /// <p>The Domain Name Service (DNS) name for the file system. You can mount your file system using its DNS name.</p>
    pub fn dns_name(&self) -> ::std::option::Option<&str> {
        self.dns_name.as_deref()
    }
    /// <p>IP addresses of the file system endpoint.</p>
    pub fn ip_addresses(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.ip_addresses.as_deref()
    }
}
impl FileSystemEndpoint {
    /// Creates a new builder-style object to manufacture [`FileSystemEndpoint`](crate::types::FileSystemEndpoint).
    pub fn builder() -> crate::types::builders::FileSystemEndpointBuilder {
        crate::types::builders::FileSystemEndpointBuilder::default()
    }
}

/// A builder for [`FileSystemEndpoint`](crate::types::FileSystemEndpoint).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FileSystemEndpointBuilder {
    pub(crate) dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) ip_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FileSystemEndpointBuilder {
    /// <p>The Domain Name Service (DNS) name for the file system. You can mount your file system using its DNS name.</p>
    pub fn dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Domain Name Service (DNS) name for the file system. You can mount your file system using its DNS name.</p>
    pub fn set_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dns_name = input;
        self
    }
    /// Appends an item to `ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_ip_addresses`](Self::set_ip_addresses).
    ///
    /// <p>IP addresses of the file system endpoint.</p>
    pub fn ip_addresses(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.ip_addresses.unwrap_or_default();
        v.push(input.into());
        self.ip_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>IP addresses of the file system endpoint.</p>
    pub fn set_ip_addresses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.ip_addresses = input;
        self
    }
    /// Consumes the builder and constructs a [`FileSystemEndpoint`](crate::types::FileSystemEndpoint).
    pub fn build(self) -> crate::types::FileSystemEndpoint {
        crate::types::FileSystemEndpoint {
            dns_name: self.dns_name,
            ip_addresses: self.ip_addresses,
        }
    }
}
