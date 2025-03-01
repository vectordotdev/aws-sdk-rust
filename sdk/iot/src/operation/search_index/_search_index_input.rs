// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchIndexInput {
    /// <p>The search index name.</p>
    #[doc(hidden)]
    pub index_name: ::std::option::Option<::std::string::String>,
    /// <p>The search query string. For more information about the search query syntax, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query syntax</a>.</p>
    #[doc(hidden)]
    pub query_string: ::std::option::Option<::std::string::String>,
    /// <p>The token used to get the next set of results, or <code>null</code> if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return at one time.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The query version.</p>
    #[doc(hidden)]
    pub query_version: ::std::option::Option<::std::string::String>,
}
impl SearchIndexInput {
    /// <p>The search index name.</p>
    pub fn index_name(&self) -> ::std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p>The search query string. For more information about the search query syntax, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query syntax</a>.</p>
    pub fn query_string(&self) -> ::std::option::Option<&str> {
        self.query_string.as_deref()
    }
    /// <p>The token used to get the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The query version.</p>
    pub fn query_version(&self) -> ::std::option::Option<&str> {
        self.query_version.as_deref()
    }
}
impl SearchIndexInput {
    /// Creates a new builder-style object to manufacture [`SearchIndexInput`](crate::operation::search_index::SearchIndexInput).
    pub fn builder() -> crate::operation::search_index::builders::SearchIndexInputBuilder {
        crate::operation::search_index::builders::SearchIndexInputBuilder::default()
    }
}

/// A builder for [`SearchIndexInput`](crate::operation::search_index::SearchIndexInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SearchIndexInputBuilder {
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) query_string: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) query_version: ::std::option::Option<::std::string::String>,
}
impl SearchIndexInputBuilder {
    /// <p>The search index name.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The search index name.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The search query string. For more information about the search query syntax, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query syntax</a>.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.query_string = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The search query string. For more information about the search query syntax, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query syntax</a>.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.query_string = input;
        self
    }
    /// <p>The token used to get the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token used to get the next set of results, or <code>null</code> if there are no additional results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The query version.</p>
    pub fn query_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.query_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The query version.</p>
    pub fn set_query_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.query_version = input;
        self
    }
    /// Consumes the builder and constructs a [`SearchIndexInput`](crate::operation::search_index::SearchIndexInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::search_index::SearchIndexInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::search_index::SearchIndexInput {
            index_name: self.index_name,
            query_string: self.query_string,
            next_token: self.next_token,
            max_results: self.max_results,
            query_version: self.query_version,
        })
    }
}
