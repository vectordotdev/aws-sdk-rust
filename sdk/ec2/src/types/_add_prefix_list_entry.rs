// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An entry for a prefix list.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AddPrefixListEntry {
    /// <p>The CIDR block.</p>
    #[doc(hidden)]
    pub cidr: ::std::option::Option<::std::string::String>,
    /// <p>A description for the entry.</p>
    /// <p>Constraints: Up to 255 characters in length.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl AddPrefixListEntry {
    /// <p>The CIDR block.</p>
    pub fn cidr(&self) -> ::std::option::Option<&str> {
        self.cidr.as_deref()
    }
    /// <p>A description for the entry.</p>
    /// <p>Constraints: Up to 255 characters in length.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl AddPrefixListEntry {
    /// Creates a new builder-style object to manufacture [`AddPrefixListEntry`](crate::types::AddPrefixListEntry).
    pub fn builder() -> crate::types::builders::AddPrefixListEntryBuilder {
        crate::types::builders::AddPrefixListEntryBuilder::default()
    }
}

/// A builder for [`AddPrefixListEntry`](crate::types::AddPrefixListEntry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AddPrefixListEntryBuilder {
    pub(crate) cidr: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl AddPrefixListEntryBuilder {
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
    /// <p>A description for the entry.</p>
    /// <p>Constraints: Up to 255 characters in length.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the entry.</p>
    /// <p>Constraints: Up to 255 characters in length.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`AddPrefixListEntry`](crate::types::AddPrefixListEntry).
    pub fn build(self) -> crate::types::AddPrefixListEntry {
        crate::types::AddPrefixListEntry {
            cidr: self.cidr,
            description: self.description,
        }
    }
}
