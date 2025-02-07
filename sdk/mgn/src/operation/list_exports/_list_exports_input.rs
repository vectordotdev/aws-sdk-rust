// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List export request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListExportsInput {
    /// <p>List exports request filters.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<crate::types::ListExportsRequestFilters>,
    /// <p>List export request max results.</p>
    #[doc(hidden)]
    pub max_results: i32,
    /// <p>List export request next token.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListExportsInput {
    /// <p>List exports request filters.</p>
    pub fn filters(&self) -> ::std::option::Option<&crate::types::ListExportsRequestFilters> {
        self.filters.as_ref()
    }
    /// <p>List export request max results.</p>
    pub fn max_results(&self) -> i32 {
        self.max_results
    }
    /// <p>List export request next token.</p>
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
    pub(crate) filters: ::std::option::Option<crate::types::ListExportsRequestFilters>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListExportsInputBuilder {
    /// <p>List exports request filters.</p>
    pub fn filters(mut self, input: crate::types::ListExportsRequestFilters) -> Self {
        self.filters = ::std::option::Option::Some(input);
        self
    }
    /// <p>List exports request filters.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<crate::types::ListExportsRequestFilters>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>List export request max results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>List export request max results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>List export request next token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>List export request next token.</p>
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
            filters: self.filters,
            max_results: self.max_results.unwrap_or_default(),
            next_token: self.next_token,
        })
    }
}
