// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeDefaultAuthorizerInput {}
impl DescribeDefaultAuthorizerInput {
    /// Creates a new builder-style object to manufacture [`DescribeDefaultAuthorizerInput`](crate::operation::describe_default_authorizer::DescribeDefaultAuthorizerInput).
    pub fn builder() -> crate::operation::describe_default_authorizer::builders::DescribeDefaultAuthorizerInputBuilder{
        crate::operation::describe_default_authorizer::builders::DescribeDefaultAuthorizerInputBuilder::default()
    }
}

/// A builder for [`DescribeDefaultAuthorizerInput`](crate::operation::describe_default_authorizer::DescribeDefaultAuthorizerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeDefaultAuthorizerInputBuilder {}
impl DescribeDefaultAuthorizerInputBuilder {
    /// Consumes the builder and constructs a [`DescribeDefaultAuthorizerInput`](crate::operation::describe_default_authorizer::DescribeDefaultAuthorizerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_default_authorizer::DescribeDefaultAuthorizerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_default_authorizer::DescribeDefaultAuthorizerInput {},
        )
    }
}
