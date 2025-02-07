// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for the ListThingPrincipal operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListThingPrincipalsInput {
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in this operation.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The name of the thing.</p>
    #[doc(hidden)]
    pub thing_name: ::std::option::Option<::std::string::String>,
}
impl ListThingPrincipalsInput {
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The name of the thing.</p>
    pub fn thing_name(&self) -> ::std::option::Option<&str> {
        self.thing_name.as_deref()
    }
}
impl ListThingPrincipalsInput {
    /// Creates a new builder-style object to manufacture [`ListThingPrincipalsInput`](crate::operation::list_thing_principals::ListThingPrincipalsInput).
    pub fn builder(
    ) -> crate::operation::list_thing_principals::builders::ListThingPrincipalsInputBuilder {
        crate::operation::list_thing_principals::builders::ListThingPrincipalsInputBuilder::default(
        )
    }
}

/// A builder for [`ListThingPrincipalsInput`](crate::operation::list_thing_principals::ListThingPrincipalsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListThingPrincipalsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) thing_name: ::std::option::Option<::std::string::String>,
}
impl ListThingPrincipalsInputBuilder {
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The name of the thing.</p>
    pub fn thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.thing_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the thing.</p>
    pub fn set_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.thing_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ListThingPrincipalsInput`](crate::operation::list_thing_principals::ListThingPrincipalsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_thing_principals::ListThingPrincipalsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_thing_principals::ListThingPrincipalsInput {
                next_token: self.next_token,
                max_results: self.max_results,
                thing_name: self.thing_name,
            },
        )
    }
}
