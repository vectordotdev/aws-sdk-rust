// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListExportsInput {
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    #[doc(hidden)]
    pub table_arn: ::std::option::Option<::std::string::String>,
    /// <p>Maximum number of results to return per page.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListExportsInput {
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    pub fn table_arn(&self) -> ::std::option::Option<&str> {
        self.table_arn.as_deref()
    }
    /// <p>Maximum number of results to return per page.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListExportsInput {
    /// Creates a new builder-style object to manufacture [`ListExportsInput`](crate::operation::list_exports::ListExportsInput).
    pub fn builder() -> crate::operation::list_exports::builders::ListExportsInputBuilder {
        crate::operation::list_exports::builders::ListExportsInputBuilder::default()
    }
}

/// A builder for [`ListExportsInput`](crate::operation::list_exports::ListExportsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListExportsInputBuilder {
    pub(crate) table_arn: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListExportsInputBuilder {
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_arn = input;
        self
    }
    /// <p>Maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListExportsInput`](crate::operation::list_exports::ListExportsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_exports::ListExportsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_exports::ListExportsInput {
            table_arn: self.table_arn,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
