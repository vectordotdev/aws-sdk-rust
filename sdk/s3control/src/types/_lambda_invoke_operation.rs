// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the configuration parameters for a <code>Lambda Invoke</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LambdaInvokeOperation {
    /// <p>The Amazon Resource Name (ARN) for the Lambda function that the specified job will invoke on every object in the manifest.</p>
    #[doc(hidden)]
    pub function_arn: ::std::option::Option<::std::string::String>,
}
impl LambdaInvokeOperation {
    /// <p>The Amazon Resource Name (ARN) for the Lambda function that the specified job will invoke on every object in the manifest.</p>
    pub fn function_arn(&self) -> ::std::option::Option<&str> {
        self.function_arn.as_deref()
    }
}
impl LambdaInvokeOperation {
    /// Creates a new builder-style object to manufacture [`LambdaInvokeOperation`](crate::types::LambdaInvokeOperation).
    pub fn builder() -> crate::types::builders::LambdaInvokeOperationBuilder {
        crate::types::builders::LambdaInvokeOperationBuilder::default()
    }
}

/// A builder for [`LambdaInvokeOperation`](crate::types::LambdaInvokeOperation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LambdaInvokeOperationBuilder {
    pub(crate) function_arn: ::std::option::Option<::std::string::String>,
}
impl LambdaInvokeOperationBuilder {
    /// <p>The Amazon Resource Name (ARN) for the Lambda function that the specified job will invoke on every object in the manifest.</p>
    pub fn function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the Lambda function that the specified job will invoke on every object in the manifest.</p>
    pub fn set_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`LambdaInvokeOperation`](crate::types::LambdaInvokeOperation).
    pub fn build(self) -> crate::types::LambdaInvokeOperation {
        crate::types::LambdaInvokeOperation {
            function_arn: self.function_arn,
        }
    }
}
