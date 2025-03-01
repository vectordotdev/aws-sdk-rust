// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSnapshotsOutput {
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>All of the returned snapshot objects.</p>
    #[doc(hidden)]
    pub snapshots: ::std::option::Option<::std::vec::Vec<crate::types::Snapshot>>,
    _request_id: Option<String>,
}
impl ListSnapshotsOutput {
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>All of the returned snapshot objects.</p>
    pub fn snapshots(&self) -> ::std::option::Option<&[crate::types::Snapshot]> {
        self.snapshots.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListSnapshotsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListSnapshotsOutput {
    /// Creates a new builder-style object to manufacture [`ListSnapshotsOutput`](crate::operation::list_snapshots::ListSnapshotsOutput).
    pub fn builder() -> crate::operation::list_snapshots::builders::ListSnapshotsOutputBuilder {
        crate::operation::list_snapshots::builders::ListSnapshotsOutputBuilder::default()
    }
}

/// A builder for [`ListSnapshotsOutput`](crate::operation::list_snapshots::ListSnapshotsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSnapshotsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) snapshots: ::std::option::Option<::std::vec::Vec<crate::types::Snapshot>>,
    _request_id: Option<String>,
}
impl ListSnapshotsOutputBuilder {
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `snapshots`.
    ///
    /// To override the contents of this collection use [`set_snapshots`](Self::set_snapshots).
    ///
    /// <p>All of the returned snapshot objects.</p>
    pub fn snapshots(mut self, input: crate::types::Snapshot) -> Self {
        let mut v = self.snapshots.unwrap_or_default();
        v.push(input);
        self.snapshots = ::std::option::Option::Some(v);
        self
    }
    /// <p>All of the returned snapshot objects.</p>
    pub fn set_snapshots(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Snapshot>>,
    ) -> Self {
        self.snapshots = input;
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
    /// Consumes the builder and constructs a [`ListSnapshotsOutput`](crate::operation::list_snapshots::ListSnapshotsOutput).
    pub fn build(self) -> crate::operation::list_snapshots::ListSnapshotsOutput {
        crate::operation::list_snapshots::ListSnapshotsOutput {
            next_token: self.next_token,
            snapshots: self.snapshots,
            _request_id: self._request_id,
        }
    }
}
