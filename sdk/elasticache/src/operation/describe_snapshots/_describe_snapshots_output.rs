// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>DescribeSnapshots</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSnapshotsOutput {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p>
    #[doc(hidden)]
    pub snapshots: ::std::option::Option<::std::vec::Vec<crate::types::Snapshot>>,
    _request_id: Option<String>,
}
impl DescribeSnapshotsOutput {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p>
    pub fn snapshots(&self) -> ::std::option::Option<&[crate::types::Snapshot]> {
        self.snapshots.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeSnapshotsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSnapshotsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput).
    pub fn builder(
    ) -> crate::operation::describe_snapshots::builders::DescribeSnapshotsOutputBuilder {
        crate::operation::describe_snapshots::builders::DescribeSnapshotsOutputBuilder::default()
    }
}

/// A builder for [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeSnapshotsOutputBuilder {
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) snapshots: ::std::option::Option<::std::vec::Vec<crate::types::Snapshot>>,
    _request_id: Option<String>,
}
impl DescribeSnapshotsOutputBuilder {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Appends an item to `snapshots`.
    ///
    /// To override the contents of this collection use [`set_snapshots`](Self::set_snapshots).
    ///
    /// <p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p>
    pub fn snapshots(mut self, input: crate::types::Snapshot) -> Self {
        let mut v = self.snapshots.unwrap_or_default();
        v.push(input);
        self.snapshots = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p>
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
    /// Consumes the builder and constructs a [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput).
    pub fn build(self) -> crate::operation::describe_snapshots::DescribeSnapshotsOutput {
        crate::operation::describe_snapshots::DescribeSnapshotsOutput {
            marker: self.marker,
            snapshots: self.snapshots,
            _request_id: self._request_id,
        }
    }
}
