// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DeleteVpnConnectionRoute.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVpnConnectionRouteInput {
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    #[doc(hidden)]
    pub destination_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPN connection.</p>
    #[doc(hidden)]
    pub vpn_connection_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVpnConnectionRouteInput {
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn destination_cidr_block(&self) -> ::std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpn_connection_id.as_deref()
    }
}
impl DeleteVpnConnectionRouteInput {
    /// Creates a new builder-style object to manufacture [`DeleteVpnConnectionRouteInput`](crate::operation::delete_vpn_connection_route::DeleteVpnConnectionRouteInput).
    pub fn builder(
    ) -> crate::operation::delete_vpn_connection_route::builders::DeleteVpnConnectionRouteInputBuilder
    {
        crate::operation::delete_vpn_connection_route::builders::DeleteVpnConnectionRouteInputBuilder::default()
    }
}

/// A builder for [`DeleteVpnConnectionRouteInput`](crate::operation::delete_vpn_connection_route::DeleteVpnConnectionRouteInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteVpnConnectionRouteInputBuilder {
    pub(crate) destination_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) vpn_connection_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVpnConnectionRouteInputBuilder {
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn destination_cidr_block(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.destination_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.vpn_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn set_vpn_connection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.vpn_connection_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVpnConnectionRouteInput`](crate::operation::delete_vpn_connection_route::DeleteVpnConnectionRouteInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_vpn_connection_route::DeleteVpnConnectionRouteInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_vpn_connection_route::DeleteVpnConnectionRouteInput {
                destination_cidr_block: self.destination_cidr_block,
                vpn_connection_id: self.vpn_connection_id,
            },
        )
    }
}
