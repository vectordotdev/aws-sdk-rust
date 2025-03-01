// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An entry for a prefix list.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemovePrefixListEntry {
    /// <p>The CIDR block.</p>
    #[doc(hidden)]
    pub cidr: ::std::option::Option<::std::string::String>,
}
impl RemovePrefixListEntry {
    /// <p>The CIDR block.</p>
    pub fn cidr(&self) -> ::std::option::Option<&str> {
        self.cidr.as_deref()
    }
}
impl RemovePrefixListEntry {
    /// Creates a new builder-style object to manufacture [`RemovePrefixListEntry`](crate::types::RemovePrefixListEntry).
    pub fn builder() -> crate::types::builders::RemovePrefixListEntryBuilder {
        crate::types::builders::RemovePrefixListEntryBuilder::default()
    }
}

/// A builder for [`RemovePrefixListEntry`](crate::types::RemovePrefixListEntry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RemovePrefixListEntryBuilder {
    pub(crate) cidr: ::std::option::Option<::std::string::String>,
}
impl RemovePrefixListEntryBuilder {
    /// <p>The CIDR block.</p>
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The CIDR block.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// Consumes the builder and constructs a [`RemovePrefixListEntry`](crate::types::RemovePrefixListEntry).
    pub fn build(self) -> crate::types::RemovePrefixListEntry {
        crate::types::RemovePrefixListEntry { cidr: self.cidr }
    }
}
