// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSchemaVersionsInput {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub limit: ::std::option::Option<i32>,
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The name of the registry.</p>
    #[doc(hidden)]
    pub registry_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the schema.</p>
    #[doc(hidden)]
    pub schema_name: ::std::option::Option<::std::string::String>,
}
impl ListSchemaVersionsInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The name of the registry.</p>
    pub fn registry_name(&self) -> ::std::option::Option<&str> {
        self.registry_name.as_deref()
    }
    /// <p>The name of the schema.</p>
    pub fn schema_name(&self) -> ::std::option::Option<&str> {
        self.schema_name.as_deref()
    }
}
impl ListSchemaVersionsInput {
    /// Creates a new builder-style object to manufacture [`ListSchemaVersionsInput`](crate::operation::list_schema_versions::ListSchemaVersionsInput).
    pub fn builder(
    ) -> crate::operation::list_schema_versions::builders::ListSchemaVersionsInputBuilder {
        crate::operation::list_schema_versions::builders::ListSchemaVersionsInputBuilder::default()
    }
}

/// A builder for [`ListSchemaVersionsInput`](crate::operation::list_schema_versions::ListSchemaVersionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSchemaVersionsInputBuilder {
    pub(crate) limit: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) registry_name: ::std::option::Option<::std::string::String>,
    pub(crate) schema_name: ::std::option::Option<::std::string::String>,
}
impl ListSchemaVersionsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that specifies the next page of results to return. To request the first page, leave NextToken empty. The token will expire in 24 hours, and cannot be shared with other accounts.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The name of the registry.</p>
    pub fn registry_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.registry_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the registry.</p>
    pub fn set_registry_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.registry_name = input;
        self
    }
    /// <p>The name of the schema.</p>
    pub fn schema_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schema_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the schema.</p>
    pub fn set_schema_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schema_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ListSchemaVersionsInput`](crate::operation::list_schema_versions::ListSchemaVersionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_schema_versions::ListSchemaVersionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_schema_versions::ListSchemaVersionsInput {
                limit: self.limit,
                next_token: self.next_token,
                registry_name: self.registry_name,
                schema_name: self.schema_name,
            },
        )
    }
}
