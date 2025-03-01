// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVocabularyFilterOutput {
    /// <p>The name you chose for your custom vocabulary filter.</p>
    #[doc(hidden)]
    pub vocabulary_filter_name: ::std::option::Option<::std::string::String>,
    /// <p>The language code you selected for your custom vocabulary filter.</p>
    #[doc(hidden)]
    pub language_code: ::std::option::Option<crate::types::LanguageCode>,
    /// <p>The date and time you created your custom vocabulary filter.</p>
    /// <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May 4, 2022.</p>
    #[doc(hidden)]
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl CreateVocabularyFilterOutput {
    /// <p>The name you chose for your custom vocabulary filter.</p>
    pub fn vocabulary_filter_name(&self) -> ::std::option::Option<&str> {
        self.vocabulary_filter_name.as_deref()
    }
    /// <p>The language code you selected for your custom vocabulary filter.</p>
    pub fn language_code(&self) -> ::std::option::Option<&crate::types::LanguageCode> {
        self.language_code.as_ref()
    }
    /// <p>The date and time you created your custom vocabulary filter.</p>
    /// <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May 4, 2022.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for CreateVocabularyFilterOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateVocabularyFilterOutput {
    /// Creates a new builder-style object to manufacture [`CreateVocabularyFilterOutput`](crate::operation::create_vocabulary_filter::CreateVocabularyFilterOutput).
    pub fn builder(
    ) -> crate::operation::create_vocabulary_filter::builders::CreateVocabularyFilterOutputBuilder
    {
        crate::operation::create_vocabulary_filter::builders::CreateVocabularyFilterOutputBuilder::default()
    }
}

/// A builder for [`CreateVocabularyFilterOutput`](crate::operation::create_vocabulary_filter::CreateVocabularyFilterOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVocabularyFilterOutputBuilder {
    pub(crate) vocabulary_filter_name: ::std::option::Option<::std::string::String>,
    pub(crate) language_code: ::std::option::Option<crate::types::LanguageCode>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl CreateVocabularyFilterOutputBuilder {
    /// <p>The name you chose for your custom vocabulary filter.</p>
    pub fn vocabulary_filter_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.vocabulary_filter_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name you chose for your custom vocabulary filter.</p>
    pub fn set_vocabulary_filter_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.vocabulary_filter_name = input;
        self
    }
    /// <p>The language code you selected for your custom vocabulary filter.</p>
    pub fn language_code(mut self, input: crate::types::LanguageCode) -> Self {
        self.language_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The language code you selected for your custom vocabulary filter.</p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::LanguageCode>,
    ) -> Self {
        self.language_code = input;
        self
    }
    /// <p>The date and time you created your custom vocabulary filter.</p>
    /// <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May 4, 2022.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time you created your custom vocabulary filter.</p>
    /// <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May 4, 2022.</p>
    pub fn set_last_modified_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_time = input;
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
    /// Consumes the builder and constructs a [`CreateVocabularyFilterOutput`](crate::operation::create_vocabulary_filter::CreateVocabularyFilterOutput).
    pub fn build(self) -> crate::operation::create_vocabulary_filter::CreateVocabularyFilterOutput {
        crate::operation::create_vocabulary_filter::CreateVocabularyFilterOutput {
            vocabulary_filter_name: self.vocabulary_filter_name,
            language_code: self.language_code,
            last_modified_time: self.last_modified_time,
            _request_id: self._request_id,
        }
    }
}
