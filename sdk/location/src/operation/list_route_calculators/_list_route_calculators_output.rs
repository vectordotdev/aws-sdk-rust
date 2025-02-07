// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListRouteCalculatorsOutput {
    /// <p>Lists the route calculator resources that exist in your Amazon Web Services account</p>
    #[doc(hidden)]
    pub entries:
        ::std::option::Option<::std::vec::Vec<crate::types::ListRouteCalculatorsResponseEntry>>,
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a subsequent request to fetch the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListRouteCalculatorsOutput {
    /// <p>Lists the route calculator resources that exist in your Amazon Web Services account</p>
    pub fn entries(
        &self,
    ) -> ::std::option::Option<&[crate::types::ListRouteCalculatorsResponseEntry]> {
        self.entries.as_deref()
    }
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a subsequent request to fetch the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListRouteCalculatorsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListRouteCalculatorsOutput {
    /// Creates a new builder-style object to manufacture [`ListRouteCalculatorsOutput`](crate::operation::list_route_calculators::ListRouteCalculatorsOutput).
    pub fn builder(
    ) -> crate::operation::list_route_calculators::builders::ListRouteCalculatorsOutputBuilder {
        crate::operation::list_route_calculators::builders::ListRouteCalculatorsOutputBuilder::default()
    }
}

/// A builder for [`ListRouteCalculatorsOutput`](crate::operation::list_route_calculators::ListRouteCalculatorsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListRouteCalculatorsOutputBuilder {
    pub(crate) entries:
        ::std::option::Option<::std::vec::Vec<crate::types::ListRouteCalculatorsResponseEntry>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListRouteCalculatorsOutputBuilder {
    /// Appends an item to `entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>Lists the route calculator resources that exist in your Amazon Web Services account</p>
    pub fn entries(mut self, input: crate::types::ListRouteCalculatorsResponseEntry) -> Self {
        let mut v = self.entries.unwrap_or_default();
        v.push(input);
        self.entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>Lists the route calculator resources that exist in your Amazon Web Services account</p>
    pub fn set_entries(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::ListRouteCalculatorsResponseEntry>,
        >,
    ) -> Self {
        self.entries = input;
        self
    }
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a subsequent request to fetch the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a subsequent request to fetch the next set of results.</p>
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
    /// Consumes the builder and constructs a [`ListRouteCalculatorsOutput`](crate::operation::list_route_calculators::ListRouteCalculatorsOutput).
    pub fn build(self) -> crate::operation::list_route_calculators::ListRouteCalculatorsOutput {
        crate::operation::list_route_calculators::ListRouteCalculatorsOutput {
            entries: self.entries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
