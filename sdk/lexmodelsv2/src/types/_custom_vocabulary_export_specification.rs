// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the parameters required for exporting a custom vocabulary.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomVocabularyExportSpecification {
    /// <p>The identifier of the bot that contains the custom vocabulary to export.</p>
    #[doc(hidden)]
    pub bot_id: ::std::option::Option<::std::string::String>,
    /// <p>The version of the bot that contains the custom vocabulary to export.</p>
    #[doc(hidden)]
    pub bot_version: ::std::option::Option<::std::string::String>,
    /// <p>The locale of the bot that contains the custom vocabulary to export.</p>
    #[doc(hidden)]
    pub locale_id: ::std::option::Option<::std::string::String>,
}
impl CustomVocabularyExportSpecification {
    /// <p>The identifier of the bot that contains the custom vocabulary to export.</p>
    pub fn bot_id(&self) -> ::std::option::Option<&str> {
        self.bot_id.as_deref()
    }
    /// <p>The version of the bot that contains the custom vocabulary to export.</p>
    pub fn bot_version(&self) -> ::std::option::Option<&str> {
        self.bot_version.as_deref()
    }
    /// <p>The locale of the bot that contains the custom vocabulary to export.</p>
    pub fn locale_id(&self) -> ::std::option::Option<&str> {
        self.locale_id.as_deref()
    }
}
impl CustomVocabularyExportSpecification {
    /// Creates a new builder-style object to manufacture [`CustomVocabularyExportSpecification`](crate::types::CustomVocabularyExportSpecification).
    pub fn builder() -> crate::types::builders::CustomVocabularyExportSpecificationBuilder {
        crate::types::builders::CustomVocabularyExportSpecificationBuilder::default()
    }
}

/// A builder for [`CustomVocabularyExportSpecification`](crate::types::CustomVocabularyExportSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CustomVocabularyExportSpecificationBuilder {
    pub(crate) bot_id: ::std::option::Option<::std::string::String>,
    pub(crate) bot_version: ::std::option::Option<::std::string::String>,
    pub(crate) locale_id: ::std::option::Option<::std::string::String>,
}
impl CustomVocabularyExportSpecificationBuilder {
    /// <p>The identifier of the bot that contains the custom vocabulary to export.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the bot that contains the custom vocabulary to export.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_id = input;
        self
    }
    /// <p>The version of the bot that contains the custom vocabulary to export.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the bot that contains the custom vocabulary to export.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_version = input;
        self
    }
    /// <p>The locale of the bot that contains the custom vocabulary to export.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.locale_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The locale of the bot that contains the custom vocabulary to export.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.locale_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomVocabularyExportSpecification`](crate::types::CustomVocabularyExportSpecification).
    pub fn build(self) -> crate::types::CustomVocabularyExportSpecification {
        crate::types::CustomVocabularyExportSpecification {
            bot_id: self.bot_id,
            bot_version: self.bot_version,
            locale_id: self.locale_id,
        }
    }
}
