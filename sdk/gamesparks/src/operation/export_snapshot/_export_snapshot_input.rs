// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportSnapshotInput {
    /// <p>The name of the game.</p>
    #[doc(hidden)]
    pub game_name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the snapshot to export.</p>
    #[doc(hidden)]
    pub snapshot_id: ::std::option::Option<::std::string::String>,
}
impl ExportSnapshotInput {
    /// <p>The name of the game.</p>
    pub fn game_name(&self) -> ::std::option::Option<&str> {
        self.game_name.as_deref()
    }
    /// <p>The identifier of the snapshot to export.</p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
}
impl ExportSnapshotInput {
    /// Creates a new builder-style object to manufacture [`ExportSnapshotInput`](crate::operation::export_snapshot::ExportSnapshotInput).
    pub fn builder() -> crate::operation::export_snapshot::builders::ExportSnapshotInputBuilder {
        crate::operation::export_snapshot::builders::ExportSnapshotInputBuilder::default()
    }
}

/// A builder for [`ExportSnapshotInput`](crate::operation::export_snapshot::ExportSnapshotInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportSnapshotInputBuilder {
    pub(crate) game_name: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
}
impl ExportSnapshotInputBuilder {
    /// <p>The name of the game.</p>
    pub fn game_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.game_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the game.</p>
    pub fn set_game_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.game_name = input;
        self
    }
    /// <p>The identifier of the snapshot to export.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the snapshot to export.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ExportSnapshotInput`](crate::operation::export_snapshot::ExportSnapshotInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_snapshot::ExportSnapshotInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::export_snapshot::ExportSnapshotInput {
            game_name: self.game_name,
            snapshot_id: self.snapshot_id,
        })
    }
}
