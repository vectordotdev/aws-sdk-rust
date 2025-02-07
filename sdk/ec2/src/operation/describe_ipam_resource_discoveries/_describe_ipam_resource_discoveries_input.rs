// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeIpamResourceDiscoveriesInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IPAM resource discovery IDs.</p>
    #[doc(hidden)]
    pub ipam_resource_discovery_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of resource discoveries to return in one page of results.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The resource discovery filters.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
}
impl DescribeIpamResourceDiscoveriesInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IPAM resource discovery IDs.</p>
    pub fn ipam_resource_discovery_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.ipam_resource_discovery_ids.as_deref()
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of resource discoveries to return in one page of results.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The resource discovery filters.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
}
impl DescribeIpamResourceDiscoveriesInput {
    /// Creates a new builder-style object to manufacture [`DescribeIpamResourceDiscoveriesInput`](crate::operation::describe_ipam_resource_discoveries::DescribeIpamResourceDiscoveriesInput).
    pub fn builder() -> crate::operation::describe_ipam_resource_discoveries::builders::DescribeIpamResourceDiscoveriesInputBuilder{
        crate::operation::describe_ipam_resource_discoveries::builders::DescribeIpamResourceDiscoveriesInputBuilder::default()
    }
}

/// A builder for [`DescribeIpamResourceDiscoveriesInput`](crate::operation::describe_ipam_resource_discoveries::DescribeIpamResourceDiscoveriesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeIpamResourceDiscoveriesInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_resource_discovery_ids:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
}
impl DescribeIpamResourceDiscoveriesInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Appends an item to `ipam_resource_discovery_ids`.
    ///
    /// To override the contents of this collection use [`set_ipam_resource_discovery_ids`](Self::set_ipam_resource_discovery_ids).
    ///
    /// <p>The IPAM resource discovery IDs.</p>
    pub fn ipam_resource_discovery_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.ipam_resource_discovery_ids.unwrap_or_default();
        v.push(input.into());
        self.ipam_resource_discovery_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IPAM resource discovery IDs.</p>
    pub fn set_ipam_resource_discovery_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.ipam_resource_discovery_ids = input;
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of resource discoveries to return in one page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of resource discoveries to return in one page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The resource discovery filters.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The resource discovery filters.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeIpamResourceDiscoveriesInput`](crate::operation::describe_ipam_resource_discoveries::DescribeIpamResourceDiscoveriesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_ipam_resource_discoveries::DescribeIpamResourceDiscoveriesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_ipam_resource_discoveries::DescribeIpamResourceDiscoveriesInput {
                dry_run: self.dry_run
                ,
                ipam_resource_discovery_ids: self.ipam_resource_discovery_ids
                ,
                next_token: self.next_token
                ,
                max_results: self.max_results
                ,
                filters: self.filters
                ,
            }
        )
    }
}
