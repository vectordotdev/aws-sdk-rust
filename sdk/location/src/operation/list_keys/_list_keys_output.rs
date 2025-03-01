// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListKeysOutput {
    /// <p>Contains API key resources in your Amazon Web Services account. Details include API key name, allowed referers and timestamp for when the API key will expire.</p>
    #[doc(hidden)]
    pub entries: ::std::option::Option<::std::vec::Vec<crate::types::ListKeysResponseEntry>>,
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a following request to fetch the next set of results. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListKeysOutput {
    /// <p>Contains API key resources in your Amazon Web Services account. Details include API key name, allowed referers and timestamp for when the API key will expire.</p>
    pub fn entries(&self) -> ::std::option::Option<&[crate::types::ListKeysResponseEntry]> {
        self.entries.as_deref()
    }
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a following request to fetch the next set of results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListKeysOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListKeysOutput {
    /// Creates a new builder-style object to manufacture [`ListKeysOutput`](crate::operation::list_keys::ListKeysOutput).
    pub fn builder() -> crate::operation::list_keys::builders::ListKeysOutputBuilder {
        crate::operation::list_keys::builders::ListKeysOutputBuilder::default()
    }
}

/// A builder for [`ListKeysOutput`](crate::operation::list_keys::ListKeysOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListKeysOutputBuilder {
    pub(crate) entries: ::std::option::Option<::std::vec::Vec<crate::types::ListKeysResponseEntry>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListKeysOutputBuilder {
    /// Appends an item to `entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>Contains API key resources in your Amazon Web Services account. Details include API key name, allowed referers and timestamp for when the API key will expire.</p>
    pub fn entries(mut self, input: crate::types::ListKeysResponseEntry) -> Self {
        let mut v = self.entries.unwrap_or_default();
        v.push(input);
        self.entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains API key resources in your Amazon Web Services account. Details include API key name, allowed referers and timestamp for when the API key will expire.</p>
    pub fn set_entries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ListKeysResponseEntry>>,
    ) -> Self {
        self.entries = input;
        self
    }
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a following request to fetch the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token indicating there are additional pages available. You can use the token in a following request to fetch the next set of results. </p>
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
    /// Consumes the builder and constructs a [`ListKeysOutput`](crate::operation::list_keys::ListKeysOutput).
    pub fn build(self) -> crate::operation::list_keys::ListKeysOutput {
        crate::operation::list_keys::ListKeysOutput {
            entries: self.entries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
