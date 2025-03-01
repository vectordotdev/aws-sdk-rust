// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSequenceStoresOutput {
    /// <p>A pagination token that's included if more results are available.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>A list of sequence stores.</p>
    #[doc(hidden)]
    pub sequence_stores: ::std::option::Option<::std::vec::Vec<crate::types::SequenceStoreDetail>>,
    _request_id: Option<String>,
}
impl ListSequenceStoresOutput {
    /// <p>A pagination token that's included if more results are available.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A list of sequence stores.</p>
    pub fn sequence_stores(&self) -> ::std::option::Option<&[crate::types::SequenceStoreDetail]> {
        self.sequence_stores.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListSequenceStoresOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListSequenceStoresOutput {
    /// Creates a new builder-style object to manufacture [`ListSequenceStoresOutput`](crate::operation::list_sequence_stores::ListSequenceStoresOutput).
    pub fn builder(
    ) -> crate::operation::list_sequence_stores::builders::ListSequenceStoresOutputBuilder {
        crate::operation::list_sequence_stores::builders::ListSequenceStoresOutputBuilder::default()
    }
}

/// A builder for [`ListSequenceStoresOutput`](crate::operation::list_sequence_stores::ListSequenceStoresOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSequenceStoresOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) sequence_stores:
        ::std::option::Option<::std::vec::Vec<crate::types::SequenceStoreDetail>>,
    _request_id: Option<String>,
}
impl ListSequenceStoresOutputBuilder {
    /// <p>A pagination token that's included if more results are available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token that's included if more results are available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `sequence_stores`.
    ///
    /// To override the contents of this collection use [`set_sequence_stores`](Self::set_sequence_stores).
    ///
    /// <p>A list of sequence stores.</p>
    pub fn sequence_stores(mut self, input: crate::types::SequenceStoreDetail) -> Self {
        let mut v = self.sequence_stores.unwrap_or_default();
        v.push(input);
        self.sequence_stores = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of sequence stores.</p>
    pub fn set_sequence_stores(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SequenceStoreDetail>>,
    ) -> Self {
        self.sequence_stores = input;
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
    /// Consumes the builder and constructs a [`ListSequenceStoresOutput`](crate::operation::list_sequence_stores::ListSequenceStoresOutput).
    pub fn build(self) -> crate::operation::list_sequence_stores::ListSequenceStoresOutput {
        crate::operation::list_sequence_stores::ListSequenceStoresOutput {
            next_token: self.next_token,
            sequence_stores: self.sequence_stores,
            _request_id: self._request_id,
        }
    }
}
