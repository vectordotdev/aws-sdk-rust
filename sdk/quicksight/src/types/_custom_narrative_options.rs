// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The custom narrative options.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomNarrativeOptions {
    /// <p>The string input of custom narrative.</p>
    #[doc(hidden)]
    pub narrative: ::std::option::Option<::std::string::String>,
}
impl CustomNarrativeOptions {
    /// <p>The string input of custom narrative.</p>
    pub fn narrative(&self) -> ::std::option::Option<&str> {
        self.narrative.as_deref()
    }
}
impl CustomNarrativeOptions {
    /// Creates a new builder-style object to manufacture [`CustomNarrativeOptions`](crate::types::CustomNarrativeOptions).
    pub fn builder() -> crate::types::builders::CustomNarrativeOptionsBuilder {
        crate::types::builders::CustomNarrativeOptionsBuilder::default()
    }
}

/// A builder for [`CustomNarrativeOptions`](crate::types::CustomNarrativeOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CustomNarrativeOptionsBuilder {
    pub(crate) narrative: ::std::option::Option<::std::string::String>,
}
impl CustomNarrativeOptionsBuilder {
    /// <p>The string input of custom narrative.</p>
    pub fn narrative(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.narrative = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The string input of custom narrative.</p>
    pub fn set_narrative(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.narrative = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomNarrativeOptions`](crate::types::CustomNarrativeOptions).
    pub fn build(self) -> crate::types::CustomNarrativeOptions {
        crate::types::CustomNarrativeOptions {
            narrative: self.narrative,
        }
    }
}
