// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex data type that includes the field patterns to match for field-level encryption.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FieldPatterns {
    /// <p>The number of field-level encryption field patterns.</p>
    #[doc(hidden)]
    pub quantity: ::std::option::Option<i32>,
    /// <p>An array of the field-level encryption field patterns.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FieldPatterns {
    /// <p>The number of field-level encryption field patterns.</p>
    pub fn quantity(&self) -> ::std::option::Option<i32> {
        self.quantity
    }
    /// <p>An array of the field-level encryption field patterns.</p>
    pub fn items(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.items.as_deref()
    }
}
impl FieldPatterns {
    /// Creates a new builder-style object to manufacture [`FieldPatterns`](crate::types::FieldPatterns).
    pub fn builder() -> crate::types::builders::FieldPatternsBuilder {
        crate::types::builders::FieldPatternsBuilder::default()
    }
}

/// A builder for [`FieldPatterns`](crate::types::FieldPatterns).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FieldPatternsBuilder {
    pub(crate) quantity: ::std::option::Option<i32>,
    pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FieldPatternsBuilder {
    /// <p>The number of field-level encryption field patterns.</p>
    pub fn quantity(mut self, input: i32) -> Self {
        self.quantity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of field-level encryption field patterns.</p>
    pub fn set_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.quantity = input;
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>An array of the field-level encryption field patterns.</p>
    pub fn items(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input.into());
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of the field-level encryption field patterns.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// Consumes the builder and constructs a [`FieldPatterns`](crate::types::FieldPatterns).
    pub fn build(self) -> crate::types::FieldPatterns {
        crate::types::FieldPatterns {
            quantity: self.quantity,
            items: self.items,
        }
    }
}
