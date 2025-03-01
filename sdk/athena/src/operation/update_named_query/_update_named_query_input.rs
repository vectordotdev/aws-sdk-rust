// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateNamedQueryInput {
    /// <p>The unique identifier (UUID) of the query.</p>
    #[doc(hidden)]
    pub named_query_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the query.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The query description.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The contents of the query with all query statements.</p>
    #[doc(hidden)]
    pub query_string: ::std::option::Option<::std::string::String>,
}
impl UpdateNamedQueryInput {
    /// <p>The unique identifier (UUID) of the query.</p>
    pub fn named_query_id(&self) -> ::std::option::Option<&str> {
        self.named_query_id.as_deref()
    }
    /// <p>The name of the query.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The query description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The contents of the query with all query statements.</p>
    pub fn query_string(&self) -> ::std::option::Option<&str> {
        self.query_string.as_deref()
    }
}
impl UpdateNamedQueryInput {
    /// Creates a new builder-style object to manufacture [`UpdateNamedQueryInput`](crate::operation::update_named_query::UpdateNamedQueryInput).
    pub fn builder() -> crate::operation::update_named_query::builders::UpdateNamedQueryInputBuilder
    {
        crate::operation::update_named_query::builders::UpdateNamedQueryInputBuilder::default()
    }
}

/// A builder for [`UpdateNamedQueryInput`](crate::operation::update_named_query::UpdateNamedQueryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateNamedQueryInputBuilder {
    pub(crate) named_query_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) query_string: ::std::option::Option<::std::string::String>,
}
impl UpdateNamedQueryInputBuilder {
    /// <p>The unique identifier (UUID) of the query.</p>
    pub fn named_query_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.named_query_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier (UUID) of the query.</p>
    pub fn set_named_query_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.named_query_id = input;
        self
    }
    /// <p>The name of the query.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the query.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The query description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The query description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The contents of the query with all query statements.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.query_string = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The contents of the query with all query statements.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.query_string = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateNamedQueryInput`](crate::operation::update_named_query::UpdateNamedQueryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_named_query::UpdateNamedQueryInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_named_query::UpdateNamedQueryInput {
                named_query_id: self.named_query_id,
                name: self.name,
                description: self.description,
                query_string: self.query_string,
            },
        )
    }
}
