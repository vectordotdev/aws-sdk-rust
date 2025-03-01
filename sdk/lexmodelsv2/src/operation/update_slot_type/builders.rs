// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_slot_type::_update_slot_type_output::UpdateSlotTypeOutputBuilder;

pub use crate::operation::update_slot_type::_update_slot_type_input::UpdateSlotTypeInputBuilder;

/// Fluent builder constructing a request to `UpdateSlotType`.
///
/// <p>Updates the configuration of an existing slot type.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSlotTypeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_slot_type::builders::UpdateSlotTypeInputBuilder,
}
impl UpdateSlotTypeFluentBuilder {
    /// Creates a new `UpdateSlotType`.
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
            crate::operation::update_slot_type::UpdateSlotType,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_slot_type::UpdateSlotTypeError,
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
        crate::operation::update_slot_type::UpdateSlotTypeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_slot_type::UpdateSlotTypeError,
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
        crate::operation::update_slot_type::UpdateSlotTypeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_slot_type::UpdateSlotTypeError,
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
            crate::operation::update_slot_type::UpdateSlotType,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_slot_type::UpdateSlotTypeError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the slot type to update.</p>
    pub fn slot_type_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.slot_type_id(input.into());
        self
    }
    /// <p>The unique identifier of the slot type to update.</p>
    pub fn set_slot_type_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_slot_type_id(input);
        self
    }
    /// <p>The new name of the slot type.</p>
    pub fn slot_type_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.slot_type_name(input.into());
        self
    }
    /// <p>The new name of the slot type.</p>
    pub fn set_slot_type_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_slot_type_name(input);
        self
    }
    /// <p>The new description of the slot type.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The new description of the slot type.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `slotTypeValues`.
    ///
    /// To override the contents of this collection use [`set_slot_type_values`](Self::set_slot_type_values).
    ///
    /// <p>A new list of values and their optional synonyms that define the values that the slot type can take.</p>
    pub fn slot_type_values(mut self, input: crate::types::SlotTypeValue) -> Self {
        self.inner = self.inner.slot_type_values(input);
        self
    }
    /// <p>A new list of values and their optional synonyms that define the values that the slot type can take.</p>
    pub fn set_slot_type_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SlotTypeValue>>,
    ) -> Self {
        self.inner = self.inner.set_slot_type_values(input);
        self
    }
    /// <p>The strategy that Amazon Lex should use when deciding on a value from the list of slot type values.</p>
    pub fn value_selection_setting(
        mut self,
        input: crate::types::SlotValueSelectionSetting,
    ) -> Self {
        self.inner = self.inner.value_selection_setting(input);
        self
    }
    /// <p>The strategy that Amazon Lex should use when deciding on a value from the list of slot type values.</p>
    pub fn set_value_selection_setting(
        mut self,
        input: ::std::option::Option<crate::types::SlotValueSelectionSetting>,
    ) -> Self {
        self.inner = self.inner.set_value_selection_setting(input);
        self
    }
    /// <p>The new built-in slot type that should be used as the parent of this slot type.</p>
    pub fn parent_slot_type_signature(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.parent_slot_type_signature(input.into());
        self
    }
    /// <p>The new built-in slot type that should be used as the parent of this slot type.</p>
    pub fn set_parent_slot_type_signature(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_parent_slot_type_signature(input);
        self
    }
    /// <p>The identifier of the bot that contains the slot type.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier of the bot that contains the slot type.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The version of the bot that contains the slot type. Must be <code>DRAFT</code>.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot that contains the slot type. Must be <code>DRAFT</code>.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The identifier of the language and locale that contains the slot type. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale that contains the slot type. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>Provides information about the external source of the slot type's definition.</p>
    pub fn external_source_setting(mut self, input: crate::types::ExternalSourceSetting) -> Self {
        self.inner = self.inner.external_source_setting(input);
        self
    }
    /// <p>Provides information about the external source of the slot type's definition.</p>
    pub fn set_external_source_setting(
        mut self,
        input: ::std::option::Option<crate::types::ExternalSourceSetting>,
    ) -> Self {
        self.inner = self.inner.set_external_source_setting(input);
        self
    }
    /// <p>Specifications for a composite slot type.</p>
    pub fn composite_slot_type_setting(
        mut self,
        input: crate::types::CompositeSlotTypeSetting,
    ) -> Self {
        self.inner = self.inner.composite_slot_type_setting(input);
        self
    }
    /// <p>Specifications for a composite slot type.</p>
    pub fn set_composite_slot_type_setting(
        mut self,
        input: ::std::option::Option<crate::types::CompositeSlotTypeSetting>,
    ) -> Self {
        self.inner = self.inner.set_composite_slot_type_setting(input);
        self
    }
}
