// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableFastSnapshotRestoresInput {
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    #[doc(hidden)]
    pub availability_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    #[doc(hidden)]
    pub source_snapshot_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl EnableFastSnapshotRestoresInput {
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn availability_zones(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.availability_zones.as_deref()
    }
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    pub fn source_snapshot_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.source_snapshot_ids.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableFastSnapshotRestoresInput {
    /// Creates a new builder-style object to manufacture [`EnableFastSnapshotRestoresInput`](crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresInput).
    pub fn builder() -> crate::operation::enable_fast_snapshot_restores::builders::EnableFastSnapshotRestoresInputBuilder{
        crate::operation::enable_fast_snapshot_restores::builders::EnableFastSnapshotRestoresInputBuilder::default()
    }
}

/// A builder for [`EnableFastSnapshotRestoresInput`](crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnableFastSnapshotRestoresInputBuilder {
    pub(crate) availability_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) source_snapshot_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl EnableFastSnapshotRestoresInputBuilder {
    /// Appends an item to `availability_zones`.
    ///
    /// To override the contents of this collection use [`set_availability_zones`](Self::set_availability_zones).
    ///
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn availability_zones(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.availability_zones.unwrap_or_default();
        v.push(input.into());
        self.availability_zones = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn set_availability_zones(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.availability_zones = input;
        self
    }
    /// Appends an item to `source_snapshot_ids`.
    ///
    /// To override the contents of this collection use [`set_source_snapshot_ids`](Self::set_source_snapshot_ids).
    ///
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    pub fn source_snapshot_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.source_snapshot_ids.unwrap_or_default();
        v.push(input.into());
        self.source_snapshot_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    pub fn set_source_snapshot_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.source_snapshot_ids = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`EnableFastSnapshotRestoresInput`](crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresInput {
                availability_zones: self.availability_zones,
                source_snapshot_ids: self.source_snapshot_ids,
                dry_run: self.dry_run,
            },
        )
    }
}
