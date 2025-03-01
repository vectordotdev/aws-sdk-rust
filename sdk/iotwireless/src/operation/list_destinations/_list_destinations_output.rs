// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDestinationsOutput {
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The list of destinations.</p>
    #[doc(hidden)]
    pub destination_list: ::std::option::Option<::std::vec::Vec<crate::types::Destinations>>,
    _request_id: Option<String>,
}
impl ListDestinationsOutput {
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The list of destinations.</p>
    pub fn destination_list(&self) -> ::std::option::Option<&[crate::types::Destinations]> {
        self.destination_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListDestinationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListDestinationsOutput {
    /// Creates a new builder-style object to manufacture [`ListDestinationsOutput`](crate::operation::list_destinations::ListDestinationsOutput).
    pub fn builder() -> crate::operation::list_destinations::builders::ListDestinationsOutputBuilder
    {
        crate::operation::list_destinations::builders::ListDestinationsOutputBuilder::default()
    }
}

/// A builder for [`ListDestinationsOutput`](crate::operation::list_destinations::ListDestinationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListDestinationsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) destination_list: ::std::option::Option<::std::vec::Vec<crate::types::Destinations>>,
    _request_id: Option<String>,
}
impl ListDestinationsOutputBuilder {
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `destination_list`.
    ///
    /// To override the contents of this collection use [`set_destination_list`](Self::set_destination_list).
    ///
    /// <p>The list of destinations.</p>
    pub fn destination_list(mut self, input: crate::types::Destinations) -> Self {
        let mut v = self.destination_list.unwrap_or_default();
        v.push(input);
        self.destination_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of destinations.</p>
    pub fn set_destination_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Destinations>>,
    ) -> Self {
        self.destination_list = input;
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
    /// Consumes the builder and constructs a [`ListDestinationsOutput`](crate::operation::list_destinations::ListDestinationsOutput).
    pub fn build(self) -> crate::operation::list_destinations::ListDestinationsOutput {
        crate::operation::list_destinations::ListDestinationsOutput {
            next_token: self.next_token,
            destination_list: self.destination_list,
            _request_id: self._request_id,
        }
    }
}
