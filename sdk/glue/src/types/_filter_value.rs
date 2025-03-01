// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a single entry in the list of values for a <code>FilterExpression</code>. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FilterValue {
    /// <p>The type of filter value.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::FilterValueType>,
    /// <p>The value to be associated.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FilterValue {
    /// <p>The type of filter value.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::FilterValueType> {
        self.r#type.as_ref()
    }
    /// <p>The value to be associated.</p>
    pub fn value(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.value.as_deref()
    }
}
impl FilterValue {
    /// Creates a new builder-style object to manufacture [`FilterValue`](crate::types::FilterValue).
    pub fn builder() -> crate::types::builders::FilterValueBuilder {
        crate::types::builders::FilterValueBuilder::default()
    }
}

/// A builder for [`FilterValue`](crate::types::FilterValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FilterValueBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::FilterValueType>,
    pub(crate) value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FilterValueBuilder {
    /// <p>The type of filter value.</p>
    pub fn r#type(mut self, input: crate::types::FilterValueType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of filter value.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::FilterValueType>) -> Self {
        self.r#type = input;
        self
    }
    /// Appends an item to `value`.
    ///
    /// To override the contents of this collection use [`set_value`](Self::set_value).
    ///
    /// <p>The value to be associated.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.value.unwrap_or_default();
        v.push(input.into());
        self.value = ::std::option::Option::Some(v);
        self
    }
    /// <p>The value to be associated.</p>
    pub fn set_value(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`FilterValue`](crate::types::FilterValue).
    pub fn build(self) -> crate::types::FilterValue {
        crate::types::FilterValue {
            r#type: self.r#type,
            value: self.value,
        }
    }
}
