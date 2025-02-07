// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayPrefixListReferenceInput {
    /// <p>The ID of the transit gateway route table.</p>
    #[doc(hidden)]
    pub transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the prefix list that is used for destination matches.</p>
    #[doc(hidden)]
    pub prefix_list_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the attachment to which traffic is routed.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether to drop traffic that matches this route.</p>
    #[doc(hidden)]
    pub blackhole: ::std::option::Option<bool>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateTransitGatewayPrefixListReferenceInput {
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_id.as_deref()
    }
    /// <p>The ID of the prefix list that is used for destination matches.</p>
    pub fn prefix_list_id(&self) -> ::std::option::Option<&str> {
        self.prefix_list_id.as_deref()
    }
    /// <p>The ID of the attachment to which traffic is routed.</p>
    pub fn transit_gateway_attachment_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>Indicates whether to drop traffic that matches this route.</p>
    pub fn blackhole(&self) -> ::std::option::Option<bool> {
        self.blackhole
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateTransitGatewayPrefixListReferenceInput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayPrefixListReferenceInput`](crate::operation::create_transit_gateway_prefix_list_reference::CreateTransitGatewayPrefixListReferenceInput).
    pub fn builder() -> crate::operation::create_transit_gateway_prefix_list_reference::builders::CreateTransitGatewayPrefixListReferenceInputBuilder{
        crate::operation::create_transit_gateway_prefix_list_reference::builders::CreateTransitGatewayPrefixListReferenceInputBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayPrefixListReferenceInput`](crate::operation::create_transit_gateway_prefix_list_reference::CreateTransitGatewayPrefixListReferenceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateTransitGatewayPrefixListReferenceInputBuilder {
    pub(crate) transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) prefix_list_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) blackhole: ::std::option::Option<bool>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateTransitGatewayPrefixListReferenceInputBuilder {
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn set_transit_gateway_route_table_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_route_table_id = input;
        self
    }
    /// <p>The ID of the prefix list that is used for destination matches.</p>
    pub fn prefix_list_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.prefix_list_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the prefix list that is used for destination matches.</p>
    pub fn set_prefix_list_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.prefix_list_id = input;
        self
    }
    /// <p>The ID of the attachment to which traffic is routed.</p>
    pub fn transit_gateway_attachment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the attachment to which traffic is routed.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>Indicates whether to drop traffic that matches this route.</p>
    pub fn blackhole(mut self, input: bool) -> Self {
        self.blackhole = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to drop traffic that matches this route.</p>
    pub fn set_blackhole(mut self, input: ::std::option::Option<bool>) -> Self {
        self.blackhole = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayPrefixListReferenceInput`](crate::operation::create_transit_gateway_prefix_list_reference::CreateTransitGatewayPrefixListReferenceInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_transit_gateway_prefix_list_reference::CreateTransitGatewayPrefixListReferenceInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::create_transit_gateway_prefix_list_reference::CreateTransitGatewayPrefixListReferenceInput {
                transit_gateway_route_table_id: self.transit_gateway_route_table_id
                ,
                prefix_list_id: self.prefix_list_id
                ,
                transit_gateway_attachment_id: self.transit_gateway_attachment_id
                ,
                blackhole: self.blackhole
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
