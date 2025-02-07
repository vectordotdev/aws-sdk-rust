// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_function::_describe_function_output::DescribeFunctionOutputBuilder;

pub use crate::operation::describe_function::_describe_function_input::DescribeFunctionInputBuilder;

/// Fluent builder constructing a request to `DescribeFunction`.
///
/// <p>Gets configuration information and metadata about a CloudFront function, but not the function's code. To get a function's code, use <code>GetFunction</code>.</p>
/// <p>To get configuration information and metadata about a function, you must provide the function's name and stage. To get these values, you can use <code>ListFunctions</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeFunctionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_function::builders::DescribeFunctionInputBuilder,
}
impl DescribeFunctionFluentBuilder {
    /// Creates a new `DescribeFunction`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_function::DescribeFunction,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_function::DescribeFunctionError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_function::DescribeFunctionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_function::DescribeFunctionError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_function::DescribeFunctionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_function::DescribeFunctionError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_function::DescribeFunction,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_function::DescribeFunctionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the function that you are getting information about.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the function that you are getting information about.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The function's stage, either <code>DEVELOPMENT</code> or <code>LIVE</code>.</p>
    pub fn stage(mut self, input: crate::types::FunctionStage) -> Self {
        self.inner = self.inner.stage(input);
        self
    }
    /// <p>The function's stage, either <code>DEVELOPMENT</code> or <code>LIVE</code>.</p>
    pub fn set_stage(mut self, input: ::std::option::Option<crate::types::FunctionStage>) -> Self {
        self.inner = self.inner.set_stage(input);
        self
    }
}
