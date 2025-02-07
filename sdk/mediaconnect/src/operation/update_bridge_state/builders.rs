// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bridge_state::_update_bridge_state_output::UpdateBridgeStateOutputBuilder;

pub use crate::operation::update_bridge_state::_update_bridge_state_input::UpdateBridgeStateInputBuilder;

/// Fluent builder constructing a request to `UpdateBridgeState`.
///
/// Updates the bridge state
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBridgeStateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bridge_state::builders::UpdateBridgeStateInputBuilder,
}
impl UpdateBridgeStateFluentBuilder {
    /// Creates a new `UpdateBridgeState`.
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
            crate::operation::update_bridge_state::UpdateBridgeState,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bridge_state::UpdateBridgeStateError,
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
        crate::operation::update_bridge_state::UpdateBridgeStateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bridge_state::UpdateBridgeStateError,
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
        crate::operation::update_bridge_state::UpdateBridgeStateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bridge_state::UpdateBridgeStateError,
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
            crate::operation::update_bridge_state::UpdateBridgeState,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bridge_state::UpdateBridgeStateError,
        >,
    > {
        self.customize_middleware().await
    }
    /// The ARN of the bridge that you want to update.
    pub fn bridge_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bridge_arn(input.into());
        self
    }
    /// The ARN of the bridge that you want to update.
    pub fn set_bridge_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bridge_arn(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn desired_state(mut self, input: crate::types::DesiredState) -> Self {
        self.inner = self.inner.desired_state(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_desired_state(
        mut self,
        input: ::std::option::Option<crate::types::DesiredState>,
    ) -> Self {
        self.inner = self.inner.set_desired_state(input);
        self
    }
}
