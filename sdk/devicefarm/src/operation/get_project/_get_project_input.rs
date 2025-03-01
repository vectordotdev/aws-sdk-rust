// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to the get project operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetProjectInput {
    /// <p>The project's ARN.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl GetProjectInput {
    /// <p>The project's ARN.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl GetProjectInput {
    /// Creates a new builder-style object to manufacture [`GetProjectInput`](crate::operation::get_project::GetProjectInput).
    pub fn builder() -> crate::operation::get_project::builders::GetProjectInputBuilder {
        crate::operation::get_project::builders::GetProjectInputBuilder::default()
    }
}

/// A builder for [`GetProjectInput`](crate::operation::get_project::GetProjectInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetProjectInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl GetProjectInputBuilder {
    /// <p>The project's ARN.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The project's ARN.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetProjectInput`](crate::operation::get_project::GetProjectInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_project::GetProjectInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_project::GetProjectInput { arn: self.arn })
    }
}
