// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the key value pairs for the JSON.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonFormatRef {
    /// <p>The specified key for the JSON.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>The specified value for the JSON.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl JsonFormatRef {
    /// <p>The specified key for the JSON.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The specified value for the JSON.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl JsonFormatRef {
    /// Creates a new builder-style object to manufacture [`JsonFormatRef`](crate::types::JsonFormatRef).
    pub fn builder() -> crate::types::builders::JsonFormatRefBuilder {
        crate::types::builders::JsonFormatRefBuilder::default()
    }
}

/// A builder for [`JsonFormatRef`](crate::types::JsonFormatRef).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JsonFormatRefBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl JsonFormatRefBuilder {
    /// <p>The specified key for the JSON.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The specified key for the JSON.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The specified value for the JSON.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The specified value for the JSON.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`JsonFormatRef`](crate::types::JsonFormatRef).
    pub fn build(self) -> crate::types::JsonFormatRef {
        crate::types::JsonFormatRef {
            key: self.key,
            value: self.value,
        }
    }
}
