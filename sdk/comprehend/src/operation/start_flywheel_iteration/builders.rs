// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_flywheel_iteration::_start_flywheel_iteration_output::StartFlywheelIterationOutputBuilder;

pub use crate::operation::start_flywheel_iteration::_start_flywheel_iteration_input::StartFlywheelIterationInputBuilder;

/// Fluent builder constructing a request to `StartFlywheelIteration`.
///
/// <p>Start the flywheel iteration.This operation uses any new datasets to train a new model version. For more information about flywheels, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/flywheels-about.html"> Flywheel overview</a> in the <i>Amazon Comprehend Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartFlywheelIterationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_flywheel_iteration::builders::StartFlywheelIterationInputBuilder,
}
impl StartFlywheelIterationFluentBuilder {
    /// Creates a new `StartFlywheelIteration`.
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
            crate::operation::start_flywheel_iteration::StartFlywheelIteration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_flywheel_iteration::StartFlywheelIterationError,
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
        crate::operation::start_flywheel_iteration::StartFlywheelIterationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_flywheel_iteration::StartFlywheelIterationError,
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
        crate::operation::start_flywheel_iteration::StartFlywheelIterationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_flywheel_iteration::StartFlywheelIterationError,
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
            crate::operation::start_flywheel_iteration::StartFlywheelIteration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_flywheel_iteration::StartFlywheelIterationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN of the flywheel.</p>
    pub fn flywheel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flywheel_arn(input.into());
        self
    }
    /// <p>The ARN of the flywheel.</p>
    pub fn set_flywheel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flywheel_arn(input);
        self
    }
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
}
