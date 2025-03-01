// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListChangedBlocksInput {
    /// <p>The ID of the first snapshot to use for the comparison.</p> <important>
    /// <p>The <code>FirstSnapshotID</code> parameter must be specified with a <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    #[doc(hidden)]
    pub first_snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the second snapshot to use for the comparison.</p> <important>
    /// <p>The <code>SecondSnapshotId</code> parameter must be specified with a <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    #[doc(hidden)]
    pub second_snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>The token to request the next page of results.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of blocks to be returned by the request.</p>
    /// <p>Even if additional blocks can be retrieved from the snapshot, the request can return less blocks than <b>MaxResults</b> or an empty array of blocks.</p>
    /// <p>To retrieve the next set of blocks from the snapshot, make another request with the returned <b>NextToken</b> value. The value of <b>NextToken</b> is <code>null</code> when there are no more blocks to return.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The block index from which the comparison should start.</p>
    /// <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    #[doc(hidden)]
    pub starting_block_index: ::std::option::Option<i32>,
}
impl ListChangedBlocksInput {
    /// <p>The ID of the first snapshot to use for the comparison.</p> <important>
    /// <p>The <code>FirstSnapshotID</code> parameter must be specified with a <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    pub fn first_snapshot_id(&self) -> ::std::option::Option<&str> {
        self.first_snapshot_id.as_deref()
    }
    /// <p>The ID of the second snapshot to use for the comparison.</p> <important>
    /// <p>The <code>SecondSnapshotId</code> parameter must be specified with a <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    pub fn second_snapshot_id(&self) -> ::std::option::Option<&str> {
        self.second_snapshot_id.as_deref()
    }
    /// <p>The token to request the next page of results.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of blocks to be returned by the request.</p>
    /// <p>Even if additional blocks can be retrieved from the snapshot, the request can return less blocks than <b>MaxResults</b> or an empty array of blocks.</p>
    /// <p>To retrieve the next set of blocks from the snapshot, make another request with the returned <b>NextToken</b> value. The value of <b>NextToken</b> is <code>null</code> when there are no more blocks to return.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The block index from which the comparison should start.</p>
    /// <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    pub fn starting_block_index(&self) -> ::std::option::Option<i32> {
        self.starting_block_index
    }
}
impl ListChangedBlocksInput {
    /// Creates a new builder-style object to manufacture [`ListChangedBlocksInput`](crate::operation::list_changed_blocks::ListChangedBlocksInput).
    pub fn builder(
    ) -> crate::operation::list_changed_blocks::builders::ListChangedBlocksInputBuilder {
        crate::operation::list_changed_blocks::builders::ListChangedBlocksInputBuilder::default()
    }
}

/// A builder for [`ListChangedBlocksInput`](crate::operation::list_changed_blocks::ListChangedBlocksInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListChangedBlocksInputBuilder {
    pub(crate) first_snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) second_snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) starting_block_index: ::std::option::Option<i32>,
}
impl ListChangedBlocksInputBuilder {
    /// <p>The ID of the first snapshot to use for the comparison.</p> <important>
    /// <p>The <code>FirstSnapshotID</code> parameter must be specified with a <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    pub fn first_snapshot_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.first_snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the first snapshot to use for the comparison.</p> <important>
    /// <p>The <code>FirstSnapshotID</code> parameter must be specified with a <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    pub fn set_first_snapshot_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.first_snapshot_id = input;
        self
    }
    /// <p>The ID of the second snapshot to use for the comparison.</p> <important>
    /// <p>The <code>SecondSnapshotId</code> parameter must be specified with a <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    pub fn second_snapshot_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.second_snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the second snapshot to use for the comparison.</p> <important>
    /// <p>The <code>SecondSnapshotId</code> parameter must be specified with a <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p>
    /// </important>
    pub fn set_second_snapshot_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.second_snapshot_id = input;
        self
    }
    /// <p>The token to request the next page of results.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to request the next page of results.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of blocks to be returned by the request.</p>
    /// <p>Even if additional blocks can be retrieved from the snapshot, the request can return less blocks than <b>MaxResults</b> or an empty array of blocks.</p>
    /// <p>To retrieve the next set of blocks from the snapshot, make another request with the returned <b>NextToken</b> value. The value of <b>NextToken</b> is <code>null</code> when there are no more blocks to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of blocks to be returned by the request.</p>
    /// <p>Even if additional blocks can be retrieved from the snapshot, the request can return less blocks than <b>MaxResults</b> or an empty array of blocks.</p>
    /// <p>To retrieve the next set of blocks from the snapshot, make another request with the returned <b>NextToken</b> value. The value of <b>NextToken</b> is <code>null</code> when there are no more blocks to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The block index from which the comparison should start.</p>
    /// <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    pub fn starting_block_index(mut self, input: i32) -> Self {
        self.starting_block_index = ::std::option::Option::Some(input);
        self
    }
    /// <p>The block index from which the comparison should start.</p>
    /// <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>
    /// <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    pub fn set_starting_block_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.starting_block_index = input;
        self
    }
    /// Consumes the builder and constructs a [`ListChangedBlocksInput`](crate::operation::list_changed_blocks::ListChangedBlocksInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_changed_blocks::ListChangedBlocksInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_changed_blocks::ListChangedBlocksInput {
                first_snapshot_id: self.first_snapshot_id,
                second_snapshot_id: self.second_snapshot_id,
                next_token: self.next_token,
                max_results: self.max_results,
                starting_block_index: self.starting_block_index,
            },
        )
    }
}
