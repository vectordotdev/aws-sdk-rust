// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeImportSnapshotTasksInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The filters.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>A list of import snapshot task IDs.</p>
    #[doc(hidden)]
    pub import_task_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A token that indicates the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeImportSnapshotTasksInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The filters.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>A list of import snapshot task IDs.</p>
    pub fn import_task_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.import_task_ids.as_deref()
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeImportSnapshotTasksInput {
    /// Creates a new builder-style object to manufacture [`DescribeImportSnapshotTasksInput`](crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksInput).
    pub fn builder() -> crate::operation::describe_import_snapshot_tasks::builders::DescribeImportSnapshotTasksInputBuilder{
        crate::operation::describe_import_snapshot_tasks::builders::DescribeImportSnapshotTasksInputBuilder::default()
    }
}

/// A builder for [`DescribeImportSnapshotTasksInput`](crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeImportSnapshotTasksInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) import_task_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeImportSnapshotTasksInputBuilder {
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
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Appends an item to `import_task_ids`.
    ///
    /// To override the contents of this collection use [`set_import_task_ids`](Self::set_import_task_ids).
    ///
    /// <p>A list of import snapshot task IDs.</p>
    pub fn import_task_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.import_task_ids.unwrap_or_default();
        v.push(input.into());
        self.import_task_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of import snapshot task IDs.</p>
    pub fn set_import_task_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.import_task_ids = input;
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeImportSnapshotTasksInput`](crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksInput {
                dry_run: self.dry_run,
                filters: self.filters,
                import_task_ids: self.import_task_ids,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
