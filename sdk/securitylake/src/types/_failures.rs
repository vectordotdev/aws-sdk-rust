// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List of all failures.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Failures {
    /// <p>List of all exception messages.</p>
    #[doc(hidden)]
    pub exception_message: ::std::option::Option<::std::string::String>,
    /// <p>List of all remediation steps for failures.</p>
    #[doc(hidden)]
    pub remediation: ::std::option::Option<::std::string::String>,
    /// <p>This error can occur if you configure the wrong timestamp format, or if the subset of entries used for validation had errors or missing values.</p>
    #[doc(hidden)]
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl Failures {
    /// <p>List of all exception messages.</p>
    pub fn exception_message(&self) -> ::std::option::Option<&str> {
        self.exception_message.as_deref()
    }
    /// <p>List of all remediation steps for failures.</p>
    pub fn remediation(&self) -> ::std::option::Option<&str> {
        self.remediation.as_deref()
    }
    /// <p>This error can occur if you configure the wrong timestamp format, or if the subset of entries used for validation had errors or missing values.</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
}
impl Failures {
    /// Creates a new builder-style object to manufacture [`Failures`](crate::types::Failures).
    pub fn builder() -> crate::types::builders::FailuresBuilder {
        crate::types::builders::FailuresBuilder::default()
    }
}

/// A builder for [`Failures`](crate::types::Failures).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailuresBuilder {
    pub(crate) exception_message: ::std::option::Option<::std::string::String>,
    pub(crate) remediation: ::std::option::Option<::std::string::String>,
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl FailuresBuilder {
    /// <p>List of all exception messages.</p>
    pub fn exception_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.exception_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>List of all exception messages.</p>
    pub fn set_exception_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.exception_message = input;
        self
    }
    /// <p>List of all remediation steps for failures.</p>
    pub fn remediation(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.remediation = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>List of all remediation steps for failures.</p>
    pub fn set_remediation(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.remediation = input;
        self
    }
    /// <p>This error can occur if you configure the wrong timestamp format, or if the subset of entries used for validation had errors or missing values.</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>This error can occur if you configure the wrong timestamp format, or if the subset of entries used for validation had errors or missing values.</p>
    pub fn set_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`Failures`](crate::types::Failures).
    pub fn build(self) -> crate::types::Failures {
        crate::types::Failures {
            exception_message: self.exception_message,
            remediation: self.remediation,
            timestamp: self.timestamp,
        }
    }
}
