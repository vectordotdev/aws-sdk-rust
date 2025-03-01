// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAliasesInput {
    /// <p>The identifier for the organization under which the entity exists.</p>
    #[doc(hidden)]
    pub organization_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for the entity for which to list the aliases.</p>
    #[doc(hidden)]
    pub entity_id: ::std::option::Option<::std::string::String>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in a single call.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListAliasesInput {
    /// <p>The identifier for the organization under which the entity exists.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
    /// <p>The identifier for the entity for which to list the aliases.</p>
    pub fn entity_id(&self) -> ::std::option::Option<&str> {
        self.entity_id.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListAliasesInput {
    /// Creates a new builder-style object to manufacture [`ListAliasesInput`](crate::operation::list_aliases::ListAliasesInput).
    pub fn builder() -> crate::operation::list_aliases::builders::ListAliasesInputBuilder {
        crate::operation::list_aliases::builders::ListAliasesInputBuilder::default()
    }
}

/// A builder for [`ListAliasesInput`](crate::operation::list_aliases::ListAliasesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAliasesInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
    pub(crate) entity_id: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListAliasesInputBuilder {
    /// <p>The identifier for the organization under which the entity exists.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the organization under which the entity exists.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.organization_id = input;
        self
    }
    /// <p>The identifier for the entity for which to list the aliases.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the entity for which to list the aliases.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_id = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListAliasesInput`](crate::operation::list_aliases::ListAliasesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_aliases::ListAliasesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_aliases::ListAliasesInput {
            organization_id: self.organization_id,
            entity_id: self.entity_id,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
