// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The unique custom vocabulary item from the custom vocabulary list.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomVocabularyItem {
    /// <p>The unique item identifer for the custom vocabulary item from the custom vocabulary list.</p>
    #[doc(hidden)]
    pub item_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique phrase for the custom vocabulary item from the custom vocabulary list.</p>
    #[doc(hidden)]
    pub phrase: ::std::option::Option<::std::string::String>,
    /// <p>The weight assigned for the custom vocabulary item from the custom vocabulary list.</p>
    #[doc(hidden)]
    pub weight: ::std::option::Option<i32>,
    /// <p>The DisplayAs value for the custom vocabulary item from the custom vocabulary list.</p>
    #[doc(hidden)]
    pub display_as: ::std::option::Option<::std::string::String>,
}
impl CustomVocabularyItem {
    /// <p>The unique item identifer for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn item_id(&self) -> ::std::option::Option<&str> {
        self.item_id.as_deref()
    }
    /// <p>The unique phrase for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn phrase(&self) -> ::std::option::Option<&str> {
        self.phrase.as_deref()
    }
    /// <p>The weight assigned for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn weight(&self) -> ::std::option::Option<i32> {
        self.weight
    }
    /// <p>The DisplayAs value for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn display_as(&self) -> ::std::option::Option<&str> {
        self.display_as.as_deref()
    }
}
impl CustomVocabularyItem {
    /// Creates a new builder-style object to manufacture [`CustomVocabularyItem`](crate::types::CustomVocabularyItem).
    pub fn builder() -> crate::types::builders::CustomVocabularyItemBuilder {
        crate::types::builders::CustomVocabularyItemBuilder::default()
    }
}

/// A builder for [`CustomVocabularyItem`](crate::types::CustomVocabularyItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CustomVocabularyItemBuilder {
    pub(crate) item_id: ::std::option::Option<::std::string::String>,
    pub(crate) phrase: ::std::option::Option<::std::string::String>,
    pub(crate) weight: ::std::option::Option<i32>,
    pub(crate) display_as: ::std::option::Option<::std::string::String>,
}
impl CustomVocabularyItemBuilder {
    /// <p>The unique item identifer for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn item_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.item_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique item identifer for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn set_item_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.item_id = input;
        self
    }
    /// <p>The unique phrase for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn phrase(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.phrase = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique phrase for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn set_phrase(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.phrase = input;
        self
    }
    /// <p>The weight assigned for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn weight(mut self, input: i32) -> Self {
        self.weight = ::std::option::Option::Some(input);
        self
    }
    /// <p>The weight assigned for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn set_weight(mut self, input: ::std::option::Option<i32>) -> Self {
        self.weight = input;
        self
    }
    /// <p>The DisplayAs value for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn display_as(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_as = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The DisplayAs value for the custom vocabulary item from the custom vocabulary list.</p>
    pub fn set_display_as(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_as = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomVocabularyItem`](crate::types::CustomVocabularyItem).
    pub fn build(self) -> crate::types::CustomVocabularyItem {
        crate::types::CustomVocabularyItem {
            item_id: self.item_id,
            phrase: self.phrase,
            weight: self.weight,
            display_as: self.display_as,
        }
    }
}
