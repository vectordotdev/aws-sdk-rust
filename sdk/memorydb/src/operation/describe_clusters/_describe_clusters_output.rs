// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeClustersOutput {
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>A list of clusters</p>
    #[doc(hidden)]
    pub clusters: ::std::option::Option<::std::vec::Vec<crate::types::Cluster>>,
    _request_id: Option<String>,
}
impl DescribeClustersOutput {
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A list of clusters</p>
    pub fn clusters(&self) -> ::std::option::Option<&[crate::types::Cluster]> {
        self.clusters.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeClustersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeClustersOutput {
    /// Creates a new builder-style object to manufacture [`DescribeClustersOutput`](crate::operation::describe_clusters::DescribeClustersOutput).
    pub fn builder() -> crate::operation::describe_clusters::builders::DescribeClustersOutputBuilder
    {
        crate::operation::describe_clusters::builders::DescribeClustersOutputBuilder::default()
    }
}

/// A builder for [`DescribeClustersOutput`](crate::operation::describe_clusters::DescribeClustersOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeClustersOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) clusters: ::std::option::Option<::std::vec::Vec<crate::types::Cluster>>,
    _request_id: Option<String>,
}
impl DescribeClustersOutputBuilder {
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `clusters`.
    ///
    /// To override the contents of this collection use [`set_clusters`](Self::set_clusters).
    ///
    /// <p>A list of clusters</p>
    pub fn clusters(mut self, input: crate::types::Cluster) -> Self {
        let mut v = self.clusters.unwrap_or_default();
        v.push(input);
        self.clusters = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of clusters</p>
    pub fn set_clusters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Cluster>>,
    ) -> Self {
        self.clusters = input;
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
    /// Consumes the builder and constructs a [`DescribeClustersOutput`](crate::operation::describe_clusters::DescribeClustersOutput).
    pub fn build(self) -> crate::operation::describe_clusters::DescribeClustersOutput {
        crate::operation::describe_clusters::DescribeClustersOutput {
            next_token: self.next_token,
            clusters: self.clusters,
            _request_id: self._request_id,
        }
    }
}
