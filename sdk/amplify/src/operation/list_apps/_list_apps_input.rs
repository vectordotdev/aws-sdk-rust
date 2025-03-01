// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The request structure for the list apps request. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAppsInput {
    /// <p> A pagination token. If non-null, the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p> The maximum number of records to list in a single response. </p>
    #[doc(hidden)]
    pub max_results: i32,
}
impl ListAppsInput {
    /// <p> A pagination token. If non-null, the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p> The maximum number of records to list in a single response. </p>
    pub fn max_results(&self) -> i32 {
        self.max_results
    }
}
impl ListAppsInput {
    /// Creates a new builder-style object to manufacture [`ListAppsInput`](crate::operation::list_apps::ListAppsInput).
    pub fn builder() -> crate::operation::list_apps::builders::ListAppsInputBuilder {
        crate::operation::list_apps::builders::ListAppsInputBuilder::default()
    }
}

/// A builder for [`ListAppsInput`](crate::operation::list_apps::ListAppsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAppsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListAppsInputBuilder {
    /// <p> A pagination token. If non-null, the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> A pagination token. If non-null, the pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p> The maximum number of records to list in a single response. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p> The maximum number of records to list in a single response. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListAppsInput`](crate::operation::list_apps::ListAppsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_apps::ListAppsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_apps::ListAppsInput {
            next_token: self.next_token,
            max_results: self.max_results.unwrap_or_default(),
        })
    }
}
