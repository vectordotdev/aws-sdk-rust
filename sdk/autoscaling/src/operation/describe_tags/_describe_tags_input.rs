// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTagsInput {
    /// <p>One or more filters to scope the tags to return. The maximum number of filters per filter type (for example, <code>auto-scaling-group</code>) is 1000.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    #[doc(hidden)]
    pub max_records: ::std::option::Option<i32>,
}
impl DescribeTagsInput {
    /// <p>One or more filters to scope the tags to return. The maximum number of filters per filter type (for example, <code>auto-scaling-group</code>) is 1000.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    pub fn max_records(&self) -> ::std::option::Option<i32> {
        self.max_records
    }
}
impl DescribeTagsInput {
    /// Creates a new builder-style object to manufacture [`DescribeTagsInput`](crate::operation::describe_tags::DescribeTagsInput).
    pub fn builder() -> crate::operation::describe_tags::builders::DescribeTagsInputBuilder {
        crate::operation::describe_tags::builders::DescribeTagsInputBuilder::default()
    }
}

/// A builder for [`DescribeTagsInput`](crate::operation::describe_tags::DescribeTagsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTagsInputBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_records: ::std::option::Option<i32>,
}
impl DescribeTagsInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters to scope the tags to return. The maximum number of filters per filter type (for example, <code>auto-scaling-group</code>) is 1000.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters to scope the tags to return. The maximum number of filters per filter type (for example, <code>auto-scaling-group</code>) is 1000.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.max_records = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_records = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeTagsInput`](crate::operation::describe_tags::DescribeTagsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_tags::DescribeTagsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_tags::DescribeTagsInput {
            filters: self.filters,
            next_token: self.next_token,
            max_records: self.max_records,
        })
    }
}
