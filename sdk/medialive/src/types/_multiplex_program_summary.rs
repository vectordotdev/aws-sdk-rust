// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Placeholder documentation for MultiplexProgramSummary
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MultiplexProgramSummary {
    /// The MediaLive Channel associated with the program.
    #[doc(hidden)]
    pub channel_id: ::std::option::Option<::std::string::String>,
    /// The name of the multiplex program.
    #[doc(hidden)]
    pub program_name: ::std::option::Option<::std::string::String>,
}
impl MultiplexProgramSummary {
    /// The MediaLive Channel associated with the program.
    pub fn channel_id(&self) -> ::std::option::Option<&str> {
        self.channel_id.as_deref()
    }
    /// The name of the multiplex program.
    pub fn program_name(&self) -> ::std::option::Option<&str> {
        self.program_name.as_deref()
    }
}
impl MultiplexProgramSummary {
    /// Creates a new builder-style object to manufacture [`MultiplexProgramSummary`](crate::types::MultiplexProgramSummary).
    pub fn builder() -> crate::types::builders::MultiplexProgramSummaryBuilder {
        crate::types::builders::MultiplexProgramSummaryBuilder::default()
    }
}

/// A builder for [`MultiplexProgramSummary`](crate::types::MultiplexProgramSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MultiplexProgramSummaryBuilder {
    pub(crate) channel_id: ::std::option::Option<::std::string::String>,
    pub(crate) program_name: ::std::option::Option<::std::string::String>,
}
impl MultiplexProgramSummaryBuilder {
    /// The MediaLive Channel associated with the program.
    pub fn channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The MediaLive Channel associated with the program.
    pub fn set_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_id = input;
        self
    }
    /// The name of the multiplex program.
    pub fn program_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.program_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the multiplex program.
    pub fn set_program_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.program_name = input;
        self
    }
    /// Consumes the builder and constructs a [`MultiplexProgramSummary`](crate::types::MultiplexProgramSummary).
    pub fn build(self) -> crate::types::MultiplexProgramSummary {
        crate::types::MultiplexProgramSummary {
            channel_id: self.channel_id,
            program_name: self.program_name,
        }
    }
}
