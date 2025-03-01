// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListStudiosInput {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListStudiosInput {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListStudiosInput {
    /// Creates a new builder-style object to manufacture [`ListStudiosInput`](crate::operation::list_studios::ListStudiosInput).
    pub fn builder() -> crate::operation::list_studios::builders::ListStudiosInputBuilder {
        crate::operation::list_studios::builders::ListStudiosInputBuilder::default()
    }
}

/// A builder for [`ListStudiosInput`](crate::operation::list_studios::ListStudiosInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListStudiosInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListStudiosInputBuilder {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListStudiosInput`](crate::operation::list_studios::ListStudiosInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_studios::ListStudiosInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_studios::ListStudiosInput {
            next_token: self.next_token,
        })
    }
}
