// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeInstanceInformationInput {
    /// <p>This is a legacy method. We recommend that you don't use this method. Instead, use the <code>Filters</code> data type. <code>Filters</code> enables you to return node information by filtering based on tags applied to managed nodes.</p> <note>
    /// <p>Attempting to use <code>InstanceInformationFilterList</code> and <code>Filters</code> leads to an exception error. </p>
    /// </note>
    #[doc(hidden)]
    pub instance_information_filter_list:
        ::std::option::Option<::std::vec::Vec<crate::types::InstanceInformationFilter>>,
    /// <p>One or more filters. Use a filter to return a more specific list of managed nodes. You can filter based on tags applied to your managed nodes. Use this <code>Filters</code> data type instead of <code>InstanceInformationFilterList</code>, which is deprecated.</p>
    #[doc(hidden)]
    pub filters:
        ::std::option::Option<::std::vec::Vec<crate::types::InstanceInformationStringFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeInstanceInformationInput {
    /// <p>This is a legacy method. We recommend that you don't use this method. Instead, use the <code>Filters</code> data type. <code>Filters</code> enables you to return node information by filtering based on tags applied to managed nodes.</p> <note>
    /// <p>Attempting to use <code>InstanceInformationFilterList</code> and <code>Filters</code> leads to an exception error. </p>
    /// </note>
    pub fn instance_information_filter_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::InstanceInformationFilter]> {
        self.instance_information_filter_list.as_deref()
    }
    /// <p>One or more filters. Use a filter to return a more specific list of managed nodes. You can filter based on tags applied to your managed nodes. Use this <code>Filters</code> data type instead of <code>InstanceInformationFilterList</code>, which is deprecated.</p>
    pub fn filters(
        &self,
    ) -> ::std::option::Option<&[crate::types::InstanceInformationStringFilter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeInstanceInformationInput {
    /// Creates a new builder-style object to manufacture [`DescribeInstanceInformationInput`](crate::operation::describe_instance_information::DescribeInstanceInformationInput).
    pub fn builder() -> crate::operation::describe_instance_information::builders::DescribeInstanceInformationInputBuilder{
        crate::operation::describe_instance_information::builders::DescribeInstanceInformationInputBuilder::default()
    }
}

/// A builder for [`DescribeInstanceInformationInput`](crate::operation::describe_instance_information::DescribeInstanceInformationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeInstanceInformationInputBuilder {
    pub(crate) instance_information_filter_list:
        ::std::option::Option<::std::vec::Vec<crate::types::InstanceInformationFilter>>,
    pub(crate) filters:
        ::std::option::Option<::std::vec::Vec<crate::types::InstanceInformationStringFilter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeInstanceInformationInputBuilder {
    /// Appends an item to `instance_information_filter_list`.
    ///
    /// To override the contents of this collection use [`set_instance_information_filter_list`](Self::set_instance_information_filter_list).
    ///
    /// <p>This is a legacy method. We recommend that you don't use this method. Instead, use the <code>Filters</code> data type. <code>Filters</code> enables you to return node information by filtering based on tags applied to managed nodes.</p> <note>
    /// <p>Attempting to use <code>InstanceInformationFilterList</code> and <code>Filters</code> leads to an exception error. </p>
    /// </note>
    pub fn instance_information_filter_list(
        mut self,
        input: crate::types::InstanceInformationFilter,
    ) -> Self {
        let mut v = self.instance_information_filter_list.unwrap_or_default();
        v.push(input);
        self.instance_information_filter_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>This is a legacy method. We recommend that you don't use this method. Instead, use the <code>Filters</code> data type. <code>Filters</code> enables you to return node information by filtering based on tags applied to managed nodes.</p> <note>
    /// <p>Attempting to use <code>InstanceInformationFilterList</code> and <code>Filters</code> leads to an exception error. </p>
    /// </note>
    pub fn set_instance_information_filter_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceInformationFilter>>,
    ) -> Self {
        self.instance_information_filter_list = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. Use a filter to return a more specific list of managed nodes. You can filter based on tags applied to your managed nodes. Use this <code>Filters</code> data type instead of <code>InstanceInformationFilterList</code>, which is deprecated.</p>
    pub fn filters(mut self, input: crate::types::InstanceInformationStringFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters. Use a filter to return a more specific list of managed nodes. You can filter based on tags applied to your managed nodes. Use this <code>Filters</code> data type instead of <code>InstanceInformationFilterList</code>, which is deprecated.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::InstanceInformationStringFilter>,
        >,
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
    /// Consumes the builder and constructs a [`DescribeInstanceInformationInput`](crate::operation::describe_instance_information::DescribeInstanceInformationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_instance_information::DescribeInstanceInformationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_instance_information::DescribeInstanceInformationInput {
                instance_information_filter_list: self.instance_information_filter_list,
                filters: self.filters,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
