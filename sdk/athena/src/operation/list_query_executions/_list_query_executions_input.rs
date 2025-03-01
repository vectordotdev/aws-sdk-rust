// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListQueryExecutionsInput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of query executions to return in this request.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The name of the workgroup from which queries are being returned. If a workgroup is not specified, a list of available query execution IDs for the queries in the primary workgroup is returned.</p>
    #[doc(hidden)]
    pub work_group: ::std::option::Option<::std::string::String>,
}
impl ListQueryExecutionsInput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of query executions to return in this request.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The name of the workgroup from which queries are being returned. If a workgroup is not specified, a list of available query execution IDs for the queries in the primary workgroup is returned.</p>
    pub fn work_group(&self) -> ::std::option::Option<&str> {
        self.work_group.as_deref()
    }
}
impl ListQueryExecutionsInput {
    /// Creates a new builder-style object to manufacture [`ListQueryExecutionsInput`](crate::operation::list_query_executions::ListQueryExecutionsInput).
    pub fn builder(
    ) -> crate::operation::list_query_executions::builders::ListQueryExecutionsInputBuilder {
        crate::operation::list_query_executions::builders::ListQueryExecutionsInputBuilder::default(
        )
    }
}

/// A builder for [`ListQueryExecutionsInput`](crate::operation::list_query_executions::ListQueryExecutionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListQueryExecutionsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) work_group: ::std::option::Option<::std::string::String>,
}
impl ListQueryExecutionsInputBuilder {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of query executions to return in this request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of query executions to return in this request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The name of the workgroup from which queries are being returned. If a workgroup is not specified, a list of available query execution IDs for the queries in the primary workgroup is returned.</p>
    pub fn work_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.work_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the workgroup from which queries are being returned. If a workgroup is not specified, a list of available query execution IDs for the queries in the primary workgroup is returned.</p>
    pub fn set_work_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.work_group = input;
        self
    }
    /// Consumes the builder and constructs a [`ListQueryExecutionsInput`](crate::operation::list_query_executions::ListQueryExecutionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_query_executions::ListQueryExecutionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_query_executions::ListQueryExecutionsInput {
                next_token: self.next_token,
                max_results: self.max_results,
                work_group: self.work_group,
            },
        )
    }
}
