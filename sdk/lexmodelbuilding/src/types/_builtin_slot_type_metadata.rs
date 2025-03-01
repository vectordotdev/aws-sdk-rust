// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information about a built in slot type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BuiltinSlotTypeMetadata {
    /// <p>A unique identifier for the built-in slot type. To find the signature for a slot type, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p>
    #[doc(hidden)]
    pub signature: ::std::option::Option<::std::string::String>,
    /// <p>A list of target locales for the slot. </p>
    #[doc(hidden)]
    pub supported_locales: ::std::option::Option<::std::vec::Vec<crate::types::Locale>>,
}
impl BuiltinSlotTypeMetadata {
    /// <p>A unique identifier for the built-in slot type. To find the signature for a slot type, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p>
    pub fn signature(&self) -> ::std::option::Option<&str> {
        self.signature.as_deref()
    }
    /// <p>A list of target locales for the slot. </p>
    pub fn supported_locales(&self) -> ::std::option::Option<&[crate::types::Locale]> {
        self.supported_locales.as_deref()
    }
}
impl BuiltinSlotTypeMetadata {
    /// Creates a new builder-style object to manufacture [`BuiltinSlotTypeMetadata`](crate::types::BuiltinSlotTypeMetadata).
    pub fn builder() -> crate::types::builders::BuiltinSlotTypeMetadataBuilder {
        crate::types::builders::BuiltinSlotTypeMetadataBuilder::default()
    }
}

/// A builder for [`BuiltinSlotTypeMetadata`](crate::types::BuiltinSlotTypeMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BuiltinSlotTypeMetadataBuilder {
    pub(crate) signature: ::std::option::Option<::std::string::String>,
    pub(crate) supported_locales: ::std::option::Option<::std::vec::Vec<crate::types::Locale>>,
}
impl BuiltinSlotTypeMetadataBuilder {
    /// <p>A unique identifier for the built-in slot type. To find the signature for a slot type, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p>
    pub fn signature(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.signature = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the built-in slot type. To find the signature for a slot type, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p>
    pub fn set_signature(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.signature = input;
        self
    }
    /// Appends an item to `supported_locales`.
    ///
    /// To override the contents of this collection use [`set_supported_locales`](Self::set_supported_locales).
    ///
    /// <p>A list of target locales for the slot. </p>
    pub fn supported_locales(mut self, input: crate::types::Locale) -> Self {
        let mut v = self.supported_locales.unwrap_or_default();
        v.push(input);
        self.supported_locales = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of target locales for the slot. </p>
    pub fn set_supported_locales(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Locale>>,
    ) -> Self {
        self.supported_locales = input;
        self
    }
    /// Consumes the builder and constructs a [`BuiltinSlotTypeMetadata`](crate::types::BuiltinSlotTypeMetadata).
    pub fn build(self) -> crate::types::BuiltinSlotTypeMetadata {
        crate::types::BuiltinSlotTypeMetadata {
            signature: self.signature,
            supported_locales: self.supported_locales,
        }
    }
}
