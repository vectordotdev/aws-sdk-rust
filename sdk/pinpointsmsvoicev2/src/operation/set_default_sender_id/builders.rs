// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_default_sender_id::_set_default_sender_id_output::SetDefaultSenderIdOutputBuilder;

pub use crate::operation::set_default_sender_id::_set_default_sender_id_input::SetDefaultSenderIdInputBuilder;

/// Fluent builder constructing a request to `SetDefaultSenderId`.
///
/// <p>Sets default sender ID on a configuration set.</p>
/// <p>When sending a text message to a destination country that supports sender IDs, the default sender ID on the configuration set specified will be used if no dedicated origination phone numbers or registered sender IDs are available in your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetDefaultSenderIdFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_default_sender_id::builders::SetDefaultSenderIdInputBuilder,
}
impl SetDefaultSenderIdFluentBuilder {
    /// Creates a new `SetDefaultSenderId`.
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
            crate::operation::set_default_sender_id::SetDefaultSenderId,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_default_sender_id::SetDefaultSenderIdError,
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
        crate::operation::set_default_sender_id::SetDefaultSenderIdOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_default_sender_id::SetDefaultSenderIdError,
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
        crate::operation::set_default_sender_id::SetDefaultSenderIdOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_default_sender_id::SetDefaultSenderIdError,
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
            crate::operation::set_default_sender_id::SetDefaultSenderId,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_default_sender_id::SetDefaultSenderIdError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The configuration set to updated with a new default SenderId. This field can be the ConsigurationSetName or ConfigurationSetArn.</p>
    pub fn configuration_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_set_name(input.into());
        self
    }
    /// <p>The configuration set to updated with a new default SenderId. This field can be the ConsigurationSetName or ConfigurationSetArn.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_set_name(input);
        self
    }
    /// <p>The current sender ID for the configuration set. When sending a text message to a destination country which supports SenderIds, the default sender ID on the configuration set specified on <code>SendTextMessage</code> will be used if no dedicated origination phone numbers or registered SenderIds are available in your account, instead of a generic sender ID, such as 'NOTICE'.</p>
    pub fn sender_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sender_id(input.into());
        self
    }
    /// <p>The current sender ID for the configuration set. When sending a text message to a destination country which supports SenderIds, the default sender ID on the configuration set specified on <code>SendTextMessage</code> will be used if no dedicated origination phone numbers or registered SenderIds are available in your account, instead of a generic sender ID, such as 'NOTICE'.</p>
    pub fn set_sender_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sender_id(input);
        self
    }
}
