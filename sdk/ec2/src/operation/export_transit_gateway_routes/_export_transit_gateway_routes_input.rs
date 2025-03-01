// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportTransitGatewayRoutesInput {
    /// <p>The ID of the route table.</p>
    #[doc(hidden)]
    pub transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>attachment.transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>attachment.resource-id</code> - The resource id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify supernet-of-match as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route (<code>active</code> | <code>blackhole</code>).</p> </li>
    /// <li> <p> <code>transit-gateway-route-destination-cidr-block</code> - The CIDR range.</p> </li>
    /// <li> <p> <code>type</code> - The type of route (<code>propagated</code> | <code>static</code>).</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The name of the S3 bucket.</p>
    #[doc(hidden)]
    pub s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl ExportTransitGatewayRoutesInput {
    /// <p>The ID of the route table.</p>
    pub fn transit_gateway_route_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_id.as_deref()
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>attachment.transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>attachment.resource-id</code> - The resource id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify supernet-of-match as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route (<code>active</code> | <code>blackhole</code>).</p> </li>
    /// <li> <p> <code>transit-gateway-route-destination-cidr-block</code> - The CIDR range.</p> </li>
    /// <li> <p> <code>type</code> - The type of route (<code>propagated</code> | <code>static</code>).</p> </li>
    /// </ul>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The name of the S3 bucket.</p>
    pub fn s3_bucket(&self) -> ::std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl ExportTransitGatewayRoutesInput {
    /// Creates a new builder-style object to manufacture [`ExportTransitGatewayRoutesInput`](crate::operation::export_transit_gateway_routes::ExportTransitGatewayRoutesInput).
    pub fn builder() -> crate::operation::export_transit_gateway_routes::builders::ExportTransitGatewayRoutesInputBuilder{
        crate::operation::export_transit_gateway_routes::builders::ExportTransitGatewayRoutesInputBuilder::default()
    }
}

/// A builder for [`ExportTransitGatewayRoutesInput`](crate::operation::export_transit_gateway_routes::ExportTransitGatewayRoutesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportTransitGatewayRoutesInputBuilder {
    pub(crate) transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl ExportTransitGatewayRoutesInputBuilder {
    /// <p>The ID of the route table.</p>
    pub fn transit_gateway_route_table_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the route table.</p>
    pub fn set_transit_gateway_route_table_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_route_table_id = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>attachment.transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>attachment.resource-id</code> - The resource id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify supernet-of-match as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route (<code>active</code> | <code>blackhole</code>).</p> </li>
    /// <li> <p> <code>transit-gateway-route-destination-cidr-block</code> - The CIDR range.</p> </li>
    /// <li> <p> <code>type</code> - The type of route (<code>propagated</code> | <code>static</code>).</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>attachment.transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>attachment.resource-id</code> - The resource id of the transit gateway attachment.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify supernet-of-match as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route (<code>active</code> | <code>blackhole</code>).</p> </li>
    /// <li> <p> <code>transit-gateway-route-destination-cidr-block</code> - The CIDR range.</p> </li>
    /// <li> <p> <code>type</code> - The type of route (<code>propagated</code> | <code>static</code>).</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The name of the S3 bucket.</p>
    pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the S3 bucket.</p>
    pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_bucket = input;
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
    /// Consumes the builder and constructs a [`ExportTransitGatewayRoutesInput`](crate::operation::export_transit_gateway_routes::ExportTransitGatewayRoutesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_transit_gateway_routes::ExportTransitGatewayRoutesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::export_transit_gateway_routes::ExportTransitGatewayRoutesInput {
                transit_gateway_route_table_id: self.transit_gateway_route_table_id,
                filters: self.filters,
                s3_bucket: self.s3_bucket,
                dry_run: self.dry_run,
            },
        )
    }
}
