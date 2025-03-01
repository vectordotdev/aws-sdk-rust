// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSourceLocationsOutput {
    /// <p>A list of source locations.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<crate::types::SourceLocation>>,
    /// <p>Pagination token returned by the list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSourceLocationsOutput {
    /// <p>A list of source locations.</p>
    pub fn items(&self) -> ::std::option::Option<&[crate::types::SourceLocation]> {
        self.items.as_deref()
    }
    /// <p>Pagination token returned by the list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListSourceLocationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListSourceLocationsOutput {
    /// Creates a new builder-style object to manufacture [`ListSourceLocationsOutput`](crate::operation::list_source_locations::ListSourceLocationsOutput).
    pub fn builder(
    ) -> crate::operation::list_source_locations::builders::ListSourceLocationsOutputBuilder {
        crate::operation::list_source_locations::builders::ListSourceLocationsOutputBuilder::default(
        )
    }
}

/// A builder for [`ListSourceLocationsOutput`](crate::operation::list_source_locations::ListSourceLocationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSourceLocationsOutputBuilder {
    pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::types::SourceLocation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSourceLocationsOutputBuilder {
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>A list of source locations.</p>
    pub fn items(mut self, input: crate::types::SourceLocation) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of source locations.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SourceLocation>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// <p>Pagination token returned by the list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Pagination token returned by the list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
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
    /// Consumes the builder and constructs a [`ListSourceLocationsOutput`](crate::operation::list_source_locations::ListSourceLocationsOutput).
    pub fn build(self) -> crate::operation::list_source_locations::ListSourceLocationsOutput {
        crate::operation::list_source_locations::ListSourceLocationsOutput {
            items: self.items,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
