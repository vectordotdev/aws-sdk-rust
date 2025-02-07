// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTransitGatewayPolicyTableAssociationsInput {
    /// <p>The ID of the transit gateway policy table.</p>
    #[doc(hidden)]
    pub transit_gateway_policy_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The filters associated with the transit gateway policy table.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl GetTransitGatewayPolicyTableAssociationsInput {
    /// <p>The ID of the transit gateway policy table.</p>
    pub fn transit_gateway_policy_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_policy_table_id.as_deref()
    }
    /// <p>The filters associated with the transit gateway policy table.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl GetTransitGatewayPolicyTableAssociationsInput {
    /// Creates a new builder-style object to manufacture [`GetTransitGatewayPolicyTableAssociationsInput`](crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsInput).
    pub fn builder() -> crate::operation::get_transit_gateway_policy_table_associations::builders::GetTransitGatewayPolicyTableAssociationsInputBuilder{
        crate::operation::get_transit_gateway_policy_table_associations::builders::GetTransitGatewayPolicyTableAssociationsInputBuilder::default()
    }
}

/// A builder for [`GetTransitGatewayPolicyTableAssociationsInput`](crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTransitGatewayPolicyTableAssociationsInputBuilder {
    pub(crate) transit_gateway_policy_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl GetTransitGatewayPolicyTableAssociationsInputBuilder {
    /// <p>The ID of the transit gateway policy table.</p>
    pub fn transit_gateway_policy_table_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_policy_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway policy table.</p>
    pub fn set_transit_gateway_policy_table_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_policy_table_id = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters associated with the transit gateway policy table.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters associated with the transit gateway policy table.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`GetTransitGatewayPolicyTableAssociationsInput`](crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsInput {
                transit_gateway_policy_table_id: self.transit_gateway_policy_table_id
                ,
                filters: self.filters
                ,
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
