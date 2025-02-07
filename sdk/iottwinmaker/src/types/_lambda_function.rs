// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Lambda function.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LambdaFunction {
    /// <p>The ARN of the Lambda function.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl LambdaFunction {
    /// <p>The ARN of the Lambda function.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl LambdaFunction {
    /// Creates a new builder-style object to manufacture [`LambdaFunction`](crate::types::LambdaFunction).
    pub fn builder() -> crate::types::builders::LambdaFunctionBuilder {
        crate::types::builders::LambdaFunctionBuilder::default()
    }
}

/// A builder for [`LambdaFunction`](crate::types::LambdaFunction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LambdaFunctionBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl LambdaFunctionBuilder {
    /// <p>The ARN of the Lambda function.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Lambda function.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`LambdaFunction`](crate::types::LambdaFunction).
    pub fn build(self) -> crate::types::LambdaFunction {
        crate::types::LambdaFunction { arn: self.arn }
    }
}
