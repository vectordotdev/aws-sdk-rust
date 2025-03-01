// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListOpsItemEventsInput {
    /// <p>One or more OpsItem filters. Use a filter to return a more specific list of results. </p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::OpsItemEventFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListOpsItemEventsInput {
    /// <p>One or more OpsItem filters. Use a filter to return a more specific list of results. </p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::OpsItemEventFilter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListOpsItemEventsInput {
    /// Creates a new builder-style object to manufacture [`ListOpsItemEventsInput`](crate::operation::list_ops_item_events::ListOpsItemEventsInput).
    pub fn builder(
    ) -> crate::operation::list_ops_item_events::builders::ListOpsItemEventsInputBuilder {
        crate::operation::list_ops_item_events::builders::ListOpsItemEventsInputBuilder::default()
    }
}

/// A builder for [`ListOpsItemEventsInput`](crate::operation::list_ops_item_events::ListOpsItemEventsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListOpsItemEventsInputBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::OpsItemEventFilter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListOpsItemEventsInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more OpsItem filters. Use a filter to return a more specific list of results. </p>
    pub fn filters(mut self, input: crate::types::OpsItemEventFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more OpsItem filters. Use a filter to return a more specific list of results. </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OpsItemEventFilter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
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
    /// Consumes the builder and constructs a [`ListOpsItemEventsInput`](crate::operation::list_ops_item_events::ListOpsItemEventsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_ops_item_events::ListOpsItemEventsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_ops_item_events::ListOpsItemEventsInput {
                filters: self.filters,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
