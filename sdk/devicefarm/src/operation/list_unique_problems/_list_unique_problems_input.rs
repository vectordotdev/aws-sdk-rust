// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to the list unique problems operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListUniqueProblemsInput {
    /// <p>The unique problems' ARNs.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListUniqueProblemsInput {
    /// <p>The unique problems' ARNs.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListUniqueProblemsInput {
    /// Creates a new builder-style object to manufacture [`ListUniqueProblemsInput`](crate::operation::list_unique_problems::ListUniqueProblemsInput).
    pub fn builder(
    ) -> crate::operation::list_unique_problems::builders::ListUniqueProblemsInputBuilder {
        crate::operation::list_unique_problems::builders::ListUniqueProblemsInputBuilder::default()
    }
}

/// A builder for [`ListUniqueProblemsInput`](crate::operation::list_unique_problems::ListUniqueProblemsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListUniqueProblemsInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListUniqueProblemsInputBuilder {
    /// <p>The unique problems' ARNs.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique problems' ARNs.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListUniqueProblemsInput`](crate::operation::list_unique_problems::ListUniqueProblemsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_unique_problems::ListUniqueProblemsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_unique_problems::ListUniqueProblemsInput {
                arn: self.arn,
                next_token: self.next_token,
            },
        )
    }
}
