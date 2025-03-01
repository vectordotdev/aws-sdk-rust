// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a set of permissions for a security group rule.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IpPermission {
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    #[doc(hidden)]
    pub from_port: ::std::option::Option<i32>,
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>).</p>
    /// <p>[VPC only] Use <code>-1</code> to specify all protocols. When authorizing security group rules, specifying <code>-1</code> or a protocol number other than <code>tcp</code>, <code>udp</code>, <code>icmp</code>, or <code>icmpv6</code> allows traffic on all ports, regardless of any port range you specify. For <code>tcp</code>, <code>udp</code>, and <code>icmp</code>, you must specify a port range. For <code>icmpv6</code>, the port range is optional; if you omit the port range, traffic for all types and codes is allowed.</p>
    #[doc(hidden)]
    pub ip_protocol: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 ranges.</p>
    #[doc(hidden)]
    pub ip_ranges: ::std::option::Option<::std::vec::Vec<crate::types::IpRange>>,
    /// <p>[VPC only] The IPv6 ranges.</p>
    #[doc(hidden)]
    pub ipv6_ranges: ::std::option::Option<::std::vec::Vec<crate::types::Ipv6Range>>,
    /// <p>[VPC only] The prefix list IDs.</p>
    #[doc(hidden)]
    pub prefix_list_ids: ::std::option::Option<::std::vec::Vec<crate::types::PrefixListId>>,
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    #[doc(hidden)]
    pub to_port: ::std::option::Option<i32>,
    /// <p>The security group and Amazon Web Services account ID pairs.</p>
    #[doc(hidden)]
    pub user_id_group_pairs: ::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>>,
}
impl IpPermission {
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>).</p>
    /// <p>[VPC only] Use <code>-1</code> to specify all protocols. When authorizing security group rules, specifying <code>-1</code> or a protocol number other than <code>tcp</code>, <code>udp</code>, <code>icmp</code>, or <code>icmpv6</code> allows traffic on all ports, regardless of any port range you specify. For <code>tcp</code>, <code>udp</code>, and <code>icmp</code>, you must specify a port range. For <code>icmpv6</code>, the port range is optional; if you omit the port range, traffic for all types and codes is allowed.</p>
    pub fn ip_protocol(&self) -> ::std::option::Option<&str> {
        self.ip_protocol.as_deref()
    }
    /// <p>The IPv4 ranges.</p>
    pub fn ip_ranges(&self) -> ::std::option::Option<&[crate::types::IpRange]> {
        self.ip_ranges.as_deref()
    }
    /// <p>[VPC only] The IPv6 ranges.</p>
    pub fn ipv6_ranges(&self) -> ::std::option::Option<&[crate::types::Ipv6Range]> {
        self.ipv6_ranges.as_deref()
    }
    /// <p>[VPC only] The prefix list IDs.</p>
    pub fn prefix_list_ids(&self) -> ::std::option::Option<&[crate::types::PrefixListId]> {
        self.prefix_list_ids.as_deref()
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
    /// <p>The security group and Amazon Web Services account ID pairs.</p>
    pub fn user_id_group_pairs(&self) -> ::std::option::Option<&[crate::types::UserIdGroupPair]> {
        self.user_id_group_pairs.as_deref()
    }
}
impl IpPermission {
    /// Creates a new builder-style object to manufacture [`IpPermission`](crate::types::IpPermission).
    pub fn builder() -> crate::types::builders::IpPermissionBuilder {
        crate::types::builders::IpPermissionBuilder::default()
    }
}

/// A builder for [`IpPermission`](crate::types::IpPermission).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IpPermissionBuilder {
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) ip_protocol: ::std::option::Option<::std::string::String>,
    pub(crate) ip_ranges: ::std::option::Option<::std::vec::Vec<crate::types::IpRange>>,
    pub(crate) ipv6_ranges: ::std::option::Option<::std::vec::Vec<crate::types::Ipv6Range>>,
    pub(crate) prefix_list_ids: ::std::option::Option<::std::vec::Vec<crate::types::PrefixListId>>,
    pub(crate) to_port: ::std::option::Option<i32>,
    pub(crate) user_id_group_pairs:
        ::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>>,
}
impl IpPermissionBuilder {
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>).</p>
    /// <p>[VPC only] Use <code>-1</code> to specify all protocols. When authorizing security group rules, specifying <code>-1</code> or a protocol number other than <code>tcp</code>, <code>udp</code>, <code>icmp</code>, or <code>icmpv6</code> allows traffic on all ports, regardless of any port range you specify. For <code>tcp</code>, <code>udp</code>, and <code>icmp</code>, you must specify a port range. For <code>icmpv6</code>, the port range is optional; if you omit the port range, traffic for all types and codes is allowed.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>).</p>
    /// <p>[VPC only] Use <code>-1</code> to specify all protocols. When authorizing security group rules, specifying <code>-1</code> or a protocol number other than <code>tcp</code>, <code>udp</code>, <code>icmp</code>, or <code>icmpv6</code> allows traffic on all ports, regardless of any port range you specify. For <code>tcp</code>, <code>udp</code>, and <code>icmp</code>, you must specify a port range. For <code>icmpv6</code>, the port range is optional; if you omit the port range, traffic for all types and codes is allowed.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_protocol = input;
        self
    }
    /// Appends an item to `ip_ranges`.
    ///
    /// To override the contents of this collection use [`set_ip_ranges`](Self::set_ip_ranges).
    ///
    /// <p>The IPv4 ranges.</p>
    pub fn ip_ranges(mut self, input: crate::types::IpRange) -> Self {
        let mut v = self.ip_ranges.unwrap_or_default();
        v.push(input);
        self.ip_ranges = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IPv4 ranges.</p>
    pub fn set_ip_ranges(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IpRange>>,
    ) -> Self {
        self.ip_ranges = input;
        self
    }
    /// Appends an item to `ipv6_ranges`.
    ///
    /// To override the contents of this collection use [`set_ipv6_ranges`](Self::set_ipv6_ranges).
    ///
    /// <p>[VPC only] The IPv6 ranges.</p>
    pub fn ipv6_ranges(mut self, input: crate::types::Ipv6Range) -> Self {
        let mut v = self.ipv6_ranges.unwrap_or_default();
        v.push(input);
        self.ipv6_ranges = ::std::option::Option::Some(v);
        self
    }
    /// <p>[VPC only] The IPv6 ranges.</p>
    pub fn set_ipv6_ranges(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Ipv6Range>>,
    ) -> Self {
        self.ipv6_ranges = input;
        self
    }
    /// Appends an item to `prefix_list_ids`.
    ///
    /// To override the contents of this collection use [`set_prefix_list_ids`](Self::set_prefix_list_ids).
    ///
    /// <p>[VPC only] The prefix list IDs.</p>
    pub fn prefix_list_ids(mut self, input: crate::types::PrefixListId) -> Self {
        let mut v = self.prefix_list_ids.unwrap_or_default();
        v.push(input);
        self.prefix_list_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>[VPC only] The prefix list IDs.</p>
    pub fn set_prefix_list_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PrefixListId>>,
    ) -> Self {
        self.prefix_list_ids = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// Appends an item to `user_id_group_pairs`.
    ///
    /// To override the contents of this collection use [`set_user_id_group_pairs`](Self::set_user_id_group_pairs).
    ///
    /// <p>The security group and Amazon Web Services account ID pairs.</p>
    pub fn user_id_group_pairs(mut self, input: crate::types::UserIdGroupPair) -> Self {
        let mut v = self.user_id_group_pairs.unwrap_or_default();
        v.push(input);
        self.user_id_group_pairs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The security group and Amazon Web Services account ID pairs.</p>
    pub fn set_user_id_group_pairs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>>,
    ) -> Self {
        self.user_id_group_pairs = input;
        self
    }
    /// Consumes the builder and constructs a [`IpPermission`](crate::types::IpPermission).
    pub fn build(self) -> crate::types::IpPermission {
        crate::types::IpPermission {
            from_port: self.from_port,
            ip_protocol: self.ip_protocol,
            ip_ranges: self.ip_ranges,
            ipv6_ranges: self.ipv6_ranges,
            prefix_list_ids: self.prefix_list_ids,
            to_port: self.to_port,
            user_id_group_pairs: self.user_id_group_pairs,
        }
    }
}
