// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A parent label for a label. A label can have 0, 1, or more parents. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Parent {
    /// <p>The name of the parent label.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl Parent {
    /// <p>The name of the parent label.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl Parent {
    /// Creates a new builder-style object to manufacture [`Parent`](crate::types::Parent).
    pub fn builder() -> crate::types::builders::ParentBuilder {
        crate::types::builders::ParentBuilder::default()
    }
}

/// A builder for [`Parent`](crate::types::Parent).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ParentBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl ParentBuilder {
    /// <p>The name of the parent label.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parent label.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`Parent`](crate::types::Parent).
    pub fn build(self) -> crate::types::Parent {
        crate::types::Parent { name: self.name }
    }
}
