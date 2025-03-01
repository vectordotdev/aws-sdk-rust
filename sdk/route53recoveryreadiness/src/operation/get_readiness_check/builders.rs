// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_readiness_check::_get_readiness_check_output::GetReadinessCheckOutputBuilder;

pub use crate::operation::get_readiness_check::_get_readiness_check_input::GetReadinessCheckInputBuilder;

/// Fluent builder constructing a request to `GetReadinessCheck`.
///
/// <p>Gets details about a readiness check.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetReadinessCheckFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_readiness_check::builders::GetReadinessCheckInputBuilder,
}
impl GetReadinessCheckFluentBuilder {
    /// Creates a new `GetReadinessCheck`.
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
            crate::operation::get_readiness_check::GetReadinessCheck,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_readiness_check::GetReadinessCheckError,
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
        crate::operation::get_readiness_check::GetReadinessCheckOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_readiness_check::GetReadinessCheckError,
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
        crate::operation::get_readiness_check::GetReadinessCheckOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_readiness_check::GetReadinessCheckError,
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
            crate::operation::get_readiness_check::GetReadinessCheck,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_readiness_check::GetReadinessCheckError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Name of a readiness check.</p>
    pub fn readiness_check_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.readiness_check_name(input.into());
        self
    }
    /// <p>Name of a readiness check.</p>
    pub fn set_readiness_check_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_readiness_check_name(input);
        self
    }
}
