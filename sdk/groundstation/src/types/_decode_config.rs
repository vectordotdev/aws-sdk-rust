// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the decode <code>Config</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecodeConfig {
    /// <p>Unvalidated JSON of a decode <code>Config</code>.</p>
    #[doc(hidden)]
    pub unvalidated_json: ::std::option::Option<::std::string::String>,
}
impl DecodeConfig {
    /// <p>Unvalidated JSON of a decode <code>Config</code>.</p>
    pub fn unvalidated_json(&self) -> ::std::option::Option<&str> {
        self.unvalidated_json.as_deref()
    }
}
impl DecodeConfig {
    /// Creates a new builder-style object to manufacture [`DecodeConfig`](crate::types::DecodeConfig).
    pub fn builder() -> crate::types::builders::DecodeConfigBuilder {
        crate::types::builders::DecodeConfigBuilder::default()
    }
}

/// A builder for [`DecodeConfig`](crate::types::DecodeConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecodeConfigBuilder {
    pub(crate) unvalidated_json: ::std::option::Option<::std::string::String>,
}
impl DecodeConfigBuilder {
    /// <p>Unvalidated JSON of a decode <code>Config</code>.</p>
    pub fn unvalidated_json(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.unvalidated_json = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unvalidated JSON of a decode <code>Config</code>.</p>
    pub fn set_unvalidated_json(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.unvalidated_json = input;
        self
    }
    /// Consumes the builder and constructs a [`DecodeConfig`](crate::types::DecodeConfig).
    pub fn build(self) -> crate::types::DecodeConfig {
        crate::types::DecodeConfig {
            unvalidated_json: self.unvalidated_json,
        }
    }
}
