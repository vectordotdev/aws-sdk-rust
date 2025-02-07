// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The name of a context that must be active for an intent to be selected by Amazon Lex.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputContext {
    /// <p>The name of the context.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl InputContext {
    /// <p>The name of the context.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl InputContext {
    /// Creates a new builder-style object to manufacture [`InputContext`](crate::types::InputContext).
    pub fn builder() -> crate::types::builders::InputContextBuilder {
        crate::types::builders::InputContextBuilder::default()
    }
}

/// A builder for [`InputContext`](crate::types::InputContext).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InputContextBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl InputContextBuilder {
    /// <p>The name of the context.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the context.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`InputContext`](crate::types::InputContext).
    pub fn build(self) -> crate::types::InputContext {
        crate::types::InputContext { name: self.name }
    }
}
