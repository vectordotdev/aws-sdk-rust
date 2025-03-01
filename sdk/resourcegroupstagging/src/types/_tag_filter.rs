// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of tags (keys and values) that are used to specify the associated resources.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagFilter {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TagFilter {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl TagFilter {
    /// Creates a new builder-style object to manufacture [`TagFilter`](crate::types::TagFilter).
    pub fn builder() -> crate::types::builders::TagFilterBuilder {
        crate::types::builders::TagFilterBuilder::default()
    }
}

/// A builder for [`TagFilter`](crate::types::TagFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagFilterBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TagFilterBuilder {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`TagFilter`](crate::types::TagFilter).
    pub fn build(self) -> crate::types::TagFilter {
        crate::types::TagFilter {
            key: self.key,
            values: self.values,
        }
    }
}
