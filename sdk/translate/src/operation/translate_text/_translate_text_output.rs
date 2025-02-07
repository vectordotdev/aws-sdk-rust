// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TranslateTextOutput {
    /// <p>The translated text.</p>
    #[doc(hidden)]
    pub translated_text: ::std::option::Option<::std::string::String>,
    /// <p>The language code for the language of the source text.</p>
    #[doc(hidden)]
    pub source_language_code: ::std::option::Option<::std::string::String>,
    /// <p>The language code for the language of the target text. </p>
    #[doc(hidden)]
    pub target_language_code: ::std::option::Option<::std::string::String>,
    /// <p>The names of the custom terminologies applied to the input text by Amazon Translate for the translated text response.</p>
    #[doc(hidden)]
    pub applied_terminologies:
        ::std::option::Option<::std::vec::Vec<crate::types::AppliedTerminology>>,
    /// <p>Settings that configure the translation output.</p>
    #[doc(hidden)]
    pub applied_settings: ::std::option::Option<crate::types::TranslationSettings>,
    _request_id: Option<String>,
}
impl TranslateTextOutput {
    /// <p>The translated text.</p>
    pub fn translated_text(&self) -> ::std::option::Option<&str> {
        self.translated_text.as_deref()
    }
    /// <p>The language code for the language of the source text.</p>
    pub fn source_language_code(&self) -> ::std::option::Option<&str> {
        self.source_language_code.as_deref()
    }
    /// <p>The language code for the language of the target text. </p>
    pub fn target_language_code(&self) -> ::std::option::Option<&str> {
        self.target_language_code.as_deref()
    }
    /// <p>The names of the custom terminologies applied to the input text by Amazon Translate for the translated text response.</p>
    pub fn applied_terminologies(
        &self,
    ) -> ::std::option::Option<&[crate::types::AppliedTerminology]> {
        self.applied_terminologies.as_deref()
    }
    /// <p>Settings that configure the translation output.</p>
    pub fn applied_settings(&self) -> ::std::option::Option<&crate::types::TranslationSettings> {
        self.applied_settings.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for TranslateTextOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl TranslateTextOutput {
    /// Creates a new builder-style object to manufacture [`TranslateTextOutput`](crate::operation::translate_text::TranslateTextOutput).
    pub fn builder() -> crate::operation::translate_text::builders::TranslateTextOutputBuilder {
        crate::operation::translate_text::builders::TranslateTextOutputBuilder::default()
    }
}

/// A builder for [`TranslateTextOutput`](crate::operation::translate_text::TranslateTextOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TranslateTextOutputBuilder {
    pub(crate) translated_text: ::std::option::Option<::std::string::String>,
    pub(crate) source_language_code: ::std::option::Option<::std::string::String>,
    pub(crate) target_language_code: ::std::option::Option<::std::string::String>,
    pub(crate) applied_terminologies:
        ::std::option::Option<::std::vec::Vec<crate::types::AppliedTerminology>>,
    pub(crate) applied_settings: ::std::option::Option<crate::types::TranslationSettings>,
    _request_id: Option<String>,
}
impl TranslateTextOutputBuilder {
    /// <p>The translated text.</p>
    pub fn translated_text(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.translated_text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The translated text.</p>
    pub fn set_translated_text(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.translated_text = input;
        self
    }
    /// <p>The language code for the language of the source text.</p>
    pub fn source_language_code(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_language_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The language code for the language of the source text.</p>
    pub fn set_source_language_code(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_language_code = input;
        self
    }
    /// <p>The language code for the language of the target text. </p>
    pub fn target_language_code(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_language_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The language code for the language of the target text. </p>
    pub fn set_target_language_code(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_language_code = input;
        self
    }
    /// Appends an item to `applied_terminologies`.
    ///
    /// To override the contents of this collection use [`set_applied_terminologies`](Self::set_applied_terminologies).
    ///
    /// <p>The names of the custom terminologies applied to the input text by Amazon Translate for the translated text response.</p>
    pub fn applied_terminologies(mut self, input: crate::types::AppliedTerminology) -> Self {
        let mut v = self.applied_terminologies.unwrap_or_default();
        v.push(input);
        self.applied_terminologies = ::std::option::Option::Some(v);
        self
    }
    /// <p>The names of the custom terminologies applied to the input text by Amazon Translate for the translated text response.</p>
    pub fn set_applied_terminologies(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AppliedTerminology>>,
    ) -> Self {
        self.applied_terminologies = input;
        self
    }
    /// <p>Settings that configure the translation output.</p>
    pub fn applied_settings(mut self, input: crate::types::TranslationSettings) -> Self {
        self.applied_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings that configure the translation output.</p>
    pub fn set_applied_settings(
        mut self,
        input: ::std::option::Option<crate::types::TranslationSettings>,
    ) -> Self {
        self.applied_settings = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`TranslateTextOutput`](crate::operation::translate_text::TranslateTextOutput).
    pub fn build(self) -> crate::operation::translate_text::TranslateTextOutput {
        crate::operation::translate_text::TranslateTextOutput {
            translated_text: self.translated_text,
            source_language_code: self.source_language_code,
            target_language_code: self.target_language_code,
            applied_terminologies: self.applied_terminologies,
            applied_settings: self.applied_settings,
            _request_id: self._request_id,
        }
    }
}
