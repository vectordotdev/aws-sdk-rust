// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The source of the trial.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TrialSource {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[doc(hidden)]
    pub source_arn: ::std::option::Option<::std::string::String>,
    /// <p>The source job type.</p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<::std::string::String>,
}
impl TrialSource {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    pub fn source_arn(&self) -> ::std::option::Option<&str> {
        self.source_arn.as_deref()
    }
    /// <p>The source job type.</p>
    pub fn source_type(&self) -> ::std::option::Option<&str> {
        self.source_type.as_deref()
    }
}
impl TrialSource {
    /// Creates a new builder-style object to manufacture [`TrialSource`](crate::types::TrialSource).
    pub fn builder() -> crate::types::builders::TrialSourceBuilder {
        crate::types::builders::TrialSourceBuilder::default()
    }
}

/// A builder for [`TrialSource`](crate::types::TrialSource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TrialSourceBuilder {
    pub(crate) source_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source_type: ::std::option::Option<::std::string::String>,
}
impl TrialSourceBuilder {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_arn = input;
        self
    }
    /// <p>The source job type.</p>
    pub fn source_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source job type.</p>
    pub fn set_source_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_type = input;
        self
    }
    /// Consumes the builder and constructs a [`TrialSource`](crate::types::TrialSource).
    pub fn build(self) -> crate::types::TrialSource {
        crate::types::TrialSource {
            source_arn: self.source_arn,
            source_type: self.source_type,
        }
    }
}
