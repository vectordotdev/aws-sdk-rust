// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_policy_generation::_cancel_policy_generation_output::CancelPolicyGenerationOutputBuilder;

pub use crate::operation::cancel_policy_generation::_cancel_policy_generation_input::CancelPolicyGenerationInputBuilder;

/// Fluent builder constructing a request to `CancelPolicyGeneration`.
///
/// <p>Cancels the requested policy generation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelPolicyGenerationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_policy_generation::builders::CancelPolicyGenerationInputBuilder,
}
impl CancelPolicyGenerationFluentBuilder {
    /// Creates a new `CancelPolicyGeneration`.
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
            crate::operation::cancel_policy_generation::CancelPolicyGeneration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_policy_generation::CancelPolicyGenerationError,
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
        crate::operation::cancel_policy_generation::CancelPolicyGenerationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_policy_generation::CancelPolicyGenerationError,
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
        crate::operation::cancel_policy_generation::CancelPolicyGenerationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_policy_generation::CancelPolicyGenerationError,
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
            crate::operation::cancel_policy_generation::CancelPolicyGeneration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_policy_generation::CancelPolicyGenerationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The <code>JobId</code> that is returned by the <code>StartPolicyGeneration</code> operation. The <code>JobId</code> can be used with <code>GetGeneratedPolicy</code> to retrieve the generated policies or used with <code>CancelPolicyGeneration</code> to cancel the policy generation request.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The <code>JobId</code> that is returned by the <code>StartPolicyGeneration</code> operation. The <code>JobId</code> can be used with <code>GetGeneratedPolicy</code> to retrieve the generated policies or used with <code>CancelPolicyGeneration</code> to cancel the policy generation request.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
}
