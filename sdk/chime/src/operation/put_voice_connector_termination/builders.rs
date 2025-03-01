// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_voice_connector_termination::_put_voice_connector_termination_output::PutVoiceConnectorTerminationOutputBuilder;

pub use crate::operation::put_voice_connector_termination::_put_voice_connector_termination_input::PutVoiceConnectorTerminationInputBuilder;

/// Fluent builder constructing a request to `PutVoiceConnectorTermination`.
///
/// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p> <note>
/// <p>If emergency calling is configured for the Amazon Chime Voice Connector, it must be deleted prior to turning off termination settings.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutVoiceConnectorTerminationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationInputBuilder,
}
impl PutVoiceConnectorTerminationFluentBuilder {
    /// Creates a new `PutVoiceConnectorTermination`.
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
            crate::operation::put_voice_connector_termination::PutVoiceConnectorTermination,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationError,
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
        crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationError,
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
        crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationError,
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
            crate::operation::put_voice_connector_termination::PutVoiceConnectorTermination,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn voice_connector_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.voice_connector_id(input.into());
        self
    }
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn set_voice_connector_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_voice_connector_id(input);
        self
    }
    /// <p>The termination setting details to add.</p>
    pub fn termination(mut self, input: crate::types::Termination) -> Self {
        self.inner = self.inner.termination(input);
        self
    }
    /// <p>The termination setting details to add.</p>
    pub fn set_termination(
        mut self,
        input: ::std::option::Option<crate::types::Termination>,
    ) -> Self {
        self.inner = self.inner.set_termination(input);
        self
    }
}
