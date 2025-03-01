// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVerifiedAccessEndpointsOutput {
    /// <p>The ID of the Verified Access endpoint.</p>
    #[doc(hidden)]
    pub verified_access_endpoints:
        ::std::option::Option<::std::vec::Vec<crate::types::VerifiedAccessEndpoint>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVerifiedAccessEndpointsOutput {
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn verified_access_endpoints(
        &self,
    ) -> ::std::option::Option<&[crate::types::VerifiedAccessEndpoint]> {
        self.verified_access_endpoints.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeVerifiedAccessEndpointsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVerifiedAccessEndpointsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVerifiedAccessEndpointsOutput`](crate::operation::describe_verified_access_endpoints::DescribeVerifiedAccessEndpointsOutput).
    pub fn builder() -> crate::operation::describe_verified_access_endpoints::builders::DescribeVerifiedAccessEndpointsOutputBuilder{
        crate::operation::describe_verified_access_endpoints::builders::DescribeVerifiedAccessEndpointsOutputBuilder::default()
    }
}

/// A builder for [`DescribeVerifiedAccessEndpointsOutput`](crate::operation::describe_verified_access_endpoints::DescribeVerifiedAccessEndpointsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVerifiedAccessEndpointsOutputBuilder {
    pub(crate) verified_access_endpoints:
        ::std::option::Option<::std::vec::Vec<crate::types::VerifiedAccessEndpoint>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVerifiedAccessEndpointsOutputBuilder {
    /// Appends an item to `verified_access_endpoints`.
    ///
    /// To override the contents of this collection use [`set_verified_access_endpoints`](Self::set_verified_access_endpoints).
    ///
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn verified_access_endpoints(
        mut self,
        input: crate::types::VerifiedAccessEndpoint,
    ) -> Self {
        let mut v = self.verified_access_endpoints.unwrap_or_default();
        v.push(input);
        self.verified_access_endpoints = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn set_verified_access_endpoints(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VerifiedAccessEndpoint>>,
    ) -> Self {
        self.verified_access_endpoints = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeVerifiedAccessEndpointsOutput`](crate::operation::describe_verified_access_endpoints::DescribeVerifiedAccessEndpointsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_verified_access_endpoints::DescribeVerifiedAccessEndpointsOutput
    {
        crate::operation::describe_verified_access_endpoints::DescribeVerifiedAccessEndpointsOutput {
            verified_access_endpoints: self.verified_access_endpoints
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
