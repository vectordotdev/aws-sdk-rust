// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The internet key exchange (IKE) version permitted for the VPN tunnel.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IkeVersionsListValue {
    /// <p>The IKE version.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl IkeVersionsListValue {
    /// <p>The IKE version.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl IkeVersionsListValue {
    /// Creates a new builder-style object to manufacture [`IkeVersionsListValue`](crate::types::IkeVersionsListValue).
    pub fn builder() -> crate::types::builders::IkeVersionsListValueBuilder {
        crate::types::builders::IkeVersionsListValueBuilder::default()
    }
}

/// A builder for [`IkeVersionsListValue`](crate::types::IkeVersionsListValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IkeVersionsListValueBuilder {
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl IkeVersionsListValueBuilder {
    /// <p>The IKE version.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IKE version.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`IkeVersionsListValue`](crate::types::IkeVersionsListValue).
    pub fn build(self) -> crate::types::IkeVersionsListValue {
        crate::types::IkeVersionsListValue { value: self.value }
    }
}
