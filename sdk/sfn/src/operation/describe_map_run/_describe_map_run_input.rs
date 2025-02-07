// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeMapRunInput {
    /// <p>The Amazon Resource Name (ARN) that identifies a Map Run.</p>
    #[doc(hidden)]
    pub map_run_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeMapRunInput {
    /// <p>The Amazon Resource Name (ARN) that identifies a Map Run.</p>
    pub fn map_run_arn(&self) -> ::std::option::Option<&str> {
        self.map_run_arn.as_deref()
    }
}
impl DescribeMapRunInput {
    /// Creates a new builder-style object to manufacture [`DescribeMapRunInput`](crate::operation::describe_map_run::DescribeMapRunInput).
    pub fn builder() -> crate::operation::describe_map_run::builders::DescribeMapRunInputBuilder {
        crate::operation::describe_map_run::builders::DescribeMapRunInputBuilder::default()
    }
}

/// A builder for [`DescribeMapRunInput`](crate::operation::describe_map_run::DescribeMapRunInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeMapRunInputBuilder {
    pub(crate) map_run_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeMapRunInputBuilder {
    /// <p>The Amazon Resource Name (ARN) that identifies a Map Run.</p>
    pub fn map_run_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.map_run_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies a Map Run.</p>
    pub fn set_map_run_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.map_run_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeMapRunInput`](crate::operation::describe_map_run::DescribeMapRunInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_map_run::DescribeMapRunInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_map_run::DescribeMapRunInput {
            map_run_arn: self.map_run_arn,
        })
    }
}
