// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a default vocabulary.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DefaultVocabulary {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a> </p>
    #[doc(hidden)]
    pub language_code: ::std::option::Option<crate::types::VocabularyLanguageCode>,
    /// <p>The identifier of the custom vocabulary.</p>
    #[doc(hidden)]
    pub vocabulary_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique name of the custom vocabulary.</p>
    #[doc(hidden)]
    pub vocabulary_name: ::std::option::Option<::std::string::String>,
}
impl DefaultVocabulary {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a> </p>
    pub fn language_code(&self) -> ::std::option::Option<&crate::types::VocabularyLanguageCode> {
        self.language_code.as_ref()
    }
    /// <p>The identifier of the custom vocabulary.</p>
    pub fn vocabulary_id(&self) -> ::std::option::Option<&str> {
        self.vocabulary_id.as_deref()
    }
    /// <p>A unique name of the custom vocabulary.</p>
    pub fn vocabulary_name(&self) -> ::std::option::Option<&str> {
        self.vocabulary_name.as_deref()
    }
}
impl DefaultVocabulary {
    /// Creates a new builder-style object to manufacture [`DefaultVocabulary`](crate::types::DefaultVocabulary).
    pub fn builder() -> crate::types::builders::DefaultVocabularyBuilder {
        crate::types::builders::DefaultVocabularyBuilder::default()
    }
}

/// A builder for [`DefaultVocabulary`](crate::types::DefaultVocabulary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DefaultVocabularyBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) language_code: ::std::option::Option<crate::types::VocabularyLanguageCode>,
    pub(crate) vocabulary_id: ::std::option::Option<::std::string::String>,
    pub(crate) vocabulary_name: ::std::option::Option<::std::string::String>,
}
impl DefaultVocabularyBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a> </p>
    pub fn language_code(mut self, input: crate::types::VocabularyLanguageCode) -> Self {
        self.language_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a> </p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::VocabularyLanguageCode>,
    ) -> Self {
        self.language_code = input;
        self
    }
    /// <p>The identifier of the custom vocabulary.</p>
    pub fn vocabulary_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.vocabulary_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the custom vocabulary.</p>
    pub fn set_vocabulary_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.vocabulary_id = input;
        self
    }
    /// <p>A unique name of the custom vocabulary.</p>
    pub fn vocabulary_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.vocabulary_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique name of the custom vocabulary.</p>
    pub fn set_vocabulary_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.vocabulary_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DefaultVocabulary`](crate::types::DefaultVocabulary).
    pub fn build(self) -> crate::types::DefaultVocabulary {
        crate::types::DefaultVocabulary {
            instance_id: self.instance_id,
            language_code: self.language_code,
            vocabulary_id: self.vocabulary_id,
            vocabulary_name: self.vocabulary_name,
        }
    }
}
