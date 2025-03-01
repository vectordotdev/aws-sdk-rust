// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeQueryDefinitionsInput {
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    #[doc(hidden)]
    pub query_definition_name_prefix: ::std::option::Option<::std::string::String>,
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeQueryDefinitionsInput {
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    pub fn query_definition_name_prefix(&self) -> ::std::option::Option<&str> {
        self.query_definition_name_prefix.as_deref()
    }
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeQueryDefinitionsInput {
    /// Creates a new builder-style object to manufacture [`DescribeQueryDefinitionsInput`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsInput).
    pub fn builder(
    ) -> crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsInputBuilder
    {
        crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsInputBuilder::default()
    }
}

/// A builder for [`DescribeQueryDefinitionsInput`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeQueryDefinitionsInputBuilder {
    pub(crate) query_definition_name_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeQueryDefinitionsInputBuilder {
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    pub fn query_definition_name_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.query_definition_name_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    pub fn set_query_definition_name_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.query_definition_name_prefix = input;
        self
    }
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeQueryDefinitionsInput`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_query_definitions::DescribeQueryDefinitionsInput {
                query_definition_name_prefix: self.query_definition_name_prefix,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
