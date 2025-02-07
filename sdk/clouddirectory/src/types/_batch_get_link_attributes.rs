// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Retrieves attributes that are associated with a typed link inside a <code>BatchRead</code> operation. For more information, see <code>GetLinkAttributes</code> and <code>BatchReadRequest$Operations</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetLinkAttributes {
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[doc(hidden)]
    pub typed_link_specifier: ::std::option::Option<crate::types::TypedLinkSpecifier>,
    /// <p>A list of attribute names whose values will be retrieved.</p>
    #[doc(hidden)]
    pub attribute_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchGetLinkAttributes {
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn typed_link_specifier(&self) -> ::std::option::Option<&crate::types::TypedLinkSpecifier> {
        self.typed_link_specifier.as_ref()
    }
    /// <p>A list of attribute names whose values will be retrieved.</p>
    pub fn attribute_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.attribute_names.as_deref()
    }
}
impl BatchGetLinkAttributes {
    /// Creates a new builder-style object to manufacture [`BatchGetLinkAttributes`](crate::types::BatchGetLinkAttributes).
    pub fn builder() -> crate::types::builders::BatchGetLinkAttributesBuilder {
        crate::types::builders::BatchGetLinkAttributesBuilder::default()
    }
}

/// A builder for [`BatchGetLinkAttributes`](crate::types::BatchGetLinkAttributes).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetLinkAttributesBuilder {
    pub(crate) typed_link_specifier: ::std::option::Option<crate::types::TypedLinkSpecifier>,
    pub(crate) attribute_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchGetLinkAttributesBuilder {
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn typed_link_specifier(mut self, input: crate::types::TypedLinkSpecifier) -> Self {
        self.typed_link_specifier = ::std::option::Option::Some(input);
        self
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn set_typed_link_specifier(
        mut self,
        input: ::std::option::Option<crate::types::TypedLinkSpecifier>,
    ) -> Self {
        self.typed_link_specifier = input;
        self
    }
    /// Appends an item to `attribute_names`.
    ///
    /// To override the contents of this collection use [`set_attribute_names`](Self::set_attribute_names).
    ///
    /// <p>A list of attribute names whose values will be retrieved.</p>
    pub fn attribute_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.attribute_names.unwrap_or_default();
        v.push(input.into());
        self.attribute_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of attribute names whose values will be retrieved.</p>
    pub fn set_attribute_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.attribute_names = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetLinkAttributes`](crate::types::BatchGetLinkAttributes).
    pub fn build(self) -> crate::types::BatchGetLinkAttributes {
        crate::types::BatchGetLinkAttributes {
            typed_link_specifier: self.typed_link_specifier,
            attribute_names: self.attribute_names,
        }
    }
}
