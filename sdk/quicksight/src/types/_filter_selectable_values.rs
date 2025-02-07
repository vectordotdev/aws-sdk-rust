// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of selectable values that are used in a control.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FilterSelectableValues {
    /// <p>The values that are used in the <code>FilterSelectableValues</code>.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FilterSelectableValues {
    /// <p>The values that are used in the <code>FilterSelectableValues</code>.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl FilterSelectableValues {
    /// Creates a new builder-style object to manufacture [`FilterSelectableValues`](crate::types::FilterSelectableValues).
    pub fn builder() -> crate::types::builders::FilterSelectableValuesBuilder {
        crate::types::builders::FilterSelectableValuesBuilder::default()
    }
}

/// A builder for [`FilterSelectableValues`](crate::types::FilterSelectableValues).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FilterSelectableValuesBuilder {
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FilterSelectableValuesBuilder {
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The values that are used in the <code>FilterSelectableValues</code>.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The values that are used in the <code>FilterSelectableValues</code>.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`FilterSelectableValues`](crate::types::FilterSelectableValues).
    pub fn build(self) -> crate::types::FilterSelectableValues {
        crate::types::FilterSelectableValues {
            values: self.values,
        }
    }
}
