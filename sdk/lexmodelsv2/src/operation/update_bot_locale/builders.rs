// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bot_locale::_update_bot_locale_output::UpdateBotLocaleOutputBuilder;

pub use crate::operation::update_bot_locale::_update_bot_locale_input::UpdateBotLocaleInputBuilder;

/// Fluent builder constructing a request to `UpdateBotLocale`.
///
/// <p>Updates the settings that a bot has for a specific locale.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBotLocaleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bot_locale::builders::UpdateBotLocaleInputBuilder,
}
impl UpdateBotLocaleFluentBuilder {
    /// Creates a new `UpdateBotLocale`.
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
            crate::operation::update_bot_locale::UpdateBotLocale,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bot_locale::UpdateBotLocaleError,
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
        crate::operation::update_bot_locale::UpdateBotLocaleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bot_locale::UpdateBotLocaleError,
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
        crate::operation::update_bot_locale::UpdateBotLocaleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bot_locale::UpdateBotLocaleError,
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
            crate::operation::update_bot_locale::UpdateBotLocale,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bot_locale::UpdateBotLocaleError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the bot that contains the locale.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot that contains the locale.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The version of the bot that contains the locale to be updated. The version can only be the <code>DRAFT</code> version.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot that contains the locale to be updated. The version can only be the <code>DRAFT</code> version.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The identifier of the language and locale to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The new description of the locale.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The new description of the locale.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The new confidence threshold where Amazon Lex inserts the <code>AMAZON.FallbackIntent</code> and <code>AMAZON.KendraSearchIntent</code> intents in the list of possible intents for an utterance.</p>
    pub fn nlu_intent_confidence_threshold(mut self, input: f64) -> Self {
        self.inner = self.inner.nlu_intent_confidence_threshold(input);
        self
    }
    /// <p>The new confidence threshold where Amazon Lex inserts the <code>AMAZON.FallbackIntent</code> and <code>AMAZON.KendraSearchIntent</code> intents in the list of possible intents for an utterance.</p>
    pub fn set_nlu_intent_confidence_threshold(
        mut self,
        input: ::std::option::Option<f64>,
    ) -> Self {
        self.inner = self.inner.set_nlu_intent_confidence_threshold(input);
        self
    }
    /// <p>The new Amazon Polly voice Amazon Lex should use for voice interaction with the user.</p>
    pub fn voice_settings(mut self, input: crate::types::VoiceSettings) -> Self {
        self.inner = self.inner.voice_settings(input);
        self
    }
    /// <p>The new Amazon Polly voice Amazon Lex should use for voice interaction with the user.</p>
    pub fn set_voice_settings(
        mut self,
        input: ::std::option::Option<crate::types::VoiceSettings>,
    ) -> Self {
        self.inner = self.inner.set_voice_settings(input);
        self
    }
}
