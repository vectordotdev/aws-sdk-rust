// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetOpsSummaryInput {
    /// <p>Specify the name of a resource data sync to get.</p>
    #[doc(hidden)]
    pub sync_name: ::std::option::Option<::std::string::String>,
    /// <p>Optional filters used to scope down the returned OpsData. </p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::OpsFilter>>,
    /// <p>Optional aggregators that return counts of OpsData based on one or more expressions.</p>
    #[doc(hidden)]
    pub aggregators: ::std::option::Option<::std::vec::Vec<crate::types::OpsAggregator>>,
    /// <p>The OpsData data type to return.</p>
    #[doc(hidden)]
    pub result_attributes: ::std::option::Option<::std::vec::Vec<crate::types::OpsResultAttribute>>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl GetOpsSummaryInput {
    /// <p>Specify the name of a resource data sync to get.</p>
    pub fn sync_name(&self) -> ::std::option::Option<&str> {
        self.sync_name.as_deref()
    }
    /// <p>Optional filters used to scope down the returned OpsData. </p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::OpsFilter]> {
        self.filters.as_deref()
    }
    /// <p>Optional aggregators that return counts of OpsData based on one or more expressions.</p>
    pub fn aggregators(&self) -> ::std::option::Option<&[crate::types::OpsAggregator]> {
        self.aggregators.as_deref()
    }
    /// <p>The OpsData data type to return.</p>
    pub fn result_attributes(&self) -> ::std::option::Option<&[crate::types::OpsResultAttribute]> {
        self.result_attributes.as_deref()
    }
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl GetOpsSummaryInput {
    /// Creates a new builder-style object to manufacture [`GetOpsSummaryInput`](crate::operation::get_ops_summary::GetOpsSummaryInput).
    pub fn builder() -> crate::operation::get_ops_summary::builders::GetOpsSummaryInputBuilder {
        crate::operation::get_ops_summary::builders::GetOpsSummaryInputBuilder::default()
    }
}

/// A builder for [`GetOpsSummaryInput`](crate::operation::get_ops_summary::GetOpsSummaryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetOpsSummaryInputBuilder {
    pub(crate) sync_name: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::OpsFilter>>,
    pub(crate) aggregators: ::std::option::Option<::std::vec::Vec<crate::types::OpsAggregator>>,
    pub(crate) result_attributes:
        ::std::option::Option<::std::vec::Vec<crate::types::OpsResultAttribute>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl GetOpsSummaryInputBuilder {
    /// <p>Specify the name of a resource data sync to get.</p>
    pub fn sync_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sync_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the name of a resource data sync to get.</p>
    pub fn set_sync_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sync_name = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Optional filters used to scope down the returned OpsData. </p>
    pub fn filters(mut self, input: crate::types::OpsFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Optional filters used to scope down the returned OpsData. </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OpsFilter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Appends an item to `aggregators`.
    ///
    /// To override the contents of this collection use [`set_aggregators`](Self::set_aggregators).
    ///
    /// <p>Optional aggregators that return counts of OpsData based on one or more expressions.</p>
    pub fn aggregators(mut self, input: crate::types::OpsAggregator) -> Self {
        let mut v = self.aggregators.unwrap_or_default();
        v.push(input);
        self.aggregators = ::std::option::Option::Some(v);
        self
    }
    /// <p>Optional aggregators that return counts of OpsData based on one or more expressions.</p>
    pub fn set_aggregators(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OpsAggregator>>,
    ) -> Self {
        self.aggregators = input;
        self
    }
    /// Appends an item to `result_attributes`.
    ///
    /// To override the contents of this collection use [`set_result_attributes`](Self::set_result_attributes).
    ///
    /// <p>The OpsData data type to return.</p>
    pub fn result_attributes(mut self, input: crate::types::OpsResultAttribute) -> Self {
        let mut v = self.result_attributes.unwrap_or_default();
        v.push(input);
        self.result_attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The OpsData data type to return.</p>
    pub fn set_result_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OpsResultAttribute>>,
    ) -> Self {
        self.result_attributes = input;
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`GetOpsSummaryInput`](crate::operation::get_ops_summary::GetOpsSummaryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_ops_summary::GetOpsSummaryInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_ops_summary::GetOpsSummaryInput {
            sync_name: self.sync_name,
            filters: self.filters,
            aggregators: self.aggregators,
            result_attributes: self.result_attributes,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
