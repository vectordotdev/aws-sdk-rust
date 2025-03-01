// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Request to ListChangesetsRequest. It exposes minimal query filters.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListChangesetsInput {
    /// <p>The unique identifier for the FinSpace Dataset to which the Changeset belongs.</p>
    #[doc(hidden)]
    pub dataset_id: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results per page.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A token that indicates where a results page should begin.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListChangesetsInput {
    /// <p>The unique identifier for the FinSpace Dataset to which the Changeset belongs.</p>
    pub fn dataset_id(&self) -> ::std::option::Option<&str> {
        self.dataset_id.as_deref()
    }
    /// <p>The maximum number of results per page.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A token that indicates where a results page should begin.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListChangesetsInput {
    /// Creates a new builder-style object to manufacture [`ListChangesetsInput`](crate::operation::list_changesets::ListChangesetsInput).
    pub fn builder() -> crate::operation::list_changesets::builders::ListChangesetsInputBuilder {
        crate::operation::list_changesets::builders::ListChangesetsInputBuilder::default()
    }
}

/// A builder for [`ListChangesetsInput`](crate::operation::list_changesets::ListChangesetsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListChangesetsInputBuilder {
    pub(crate) dataset_id: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListChangesetsInputBuilder {
    /// <p>The unique identifier for the FinSpace Dataset to which the Changeset belongs.</p>
    pub fn dataset_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dataset_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the FinSpace Dataset to which the Changeset belongs.</p>
    pub fn set_dataset_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dataset_id = input;
        self
    }
    /// <p>The maximum number of results per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>A token that indicates where a results page should begin.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates where a results page should begin.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListChangesetsInput`](crate::operation::list_changesets::ListChangesetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_changesets::ListChangesetsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_changesets::ListChangesetsInput {
            dataset_id: self.dataset_id,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
