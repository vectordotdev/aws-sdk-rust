// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes how to bind a component property to form data.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FormBindingElement {
    /// <p>The name of the component to retrieve a value from.</p>
    #[doc(hidden)]
    pub element: ::std::option::Option<::std::string::String>,
    /// <p>The property to retrieve a value from.</p>
    #[doc(hidden)]
    pub property: ::std::option::Option<::std::string::String>,
}
impl FormBindingElement {
    /// <p>The name of the component to retrieve a value from.</p>
    pub fn element(&self) -> ::std::option::Option<&str> {
        self.element.as_deref()
    }
    /// <p>The property to retrieve a value from.</p>
    pub fn property(&self) -> ::std::option::Option<&str> {
        self.property.as_deref()
    }
}
impl FormBindingElement {
    /// Creates a new builder-style object to manufacture [`FormBindingElement`](crate::types::FormBindingElement).
    pub fn builder() -> crate::types::builders::FormBindingElementBuilder {
        crate::types::builders::FormBindingElementBuilder::default()
    }
}

/// A builder for [`FormBindingElement`](crate::types::FormBindingElement).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FormBindingElementBuilder {
    pub(crate) element: ::std::option::Option<::std::string::String>,
    pub(crate) property: ::std::option::Option<::std::string::String>,
}
impl FormBindingElementBuilder {
    /// <p>The name of the component to retrieve a value from.</p>
    pub fn element(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.element = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the component to retrieve a value from.</p>
    pub fn set_element(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.element = input;
        self
    }
    /// <p>The property to retrieve a value from.</p>
    pub fn property(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.property = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The property to retrieve a value from.</p>
    pub fn set_property(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.property = input;
        self
    }
    /// Consumes the builder and constructs a [`FormBindingElement`](crate::types::FormBindingElement).
    pub fn build(self) -> crate::types::FormBindingElement {
        crate::types::FormBindingElement {
            element: self.element,
            property: self.property,
        }
    }
}
