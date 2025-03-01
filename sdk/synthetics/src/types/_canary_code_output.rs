// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure contains information about the canary's Lambda handler and where its code is stored by CloudWatch Synthetics.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CanaryCodeOutput {
    /// <p>The ARN of the Lambda layer where Synthetics stores the canary script code.</p>
    #[doc(hidden)]
    pub source_location_arn: ::std::option::Option<::std::string::String>,
    /// <p>The entry point to use for the source code when running the canary.</p>
    #[doc(hidden)]
    pub handler: ::std::option::Option<::std::string::String>,
}
impl CanaryCodeOutput {
    /// <p>The ARN of the Lambda layer where Synthetics stores the canary script code.</p>
    pub fn source_location_arn(&self) -> ::std::option::Option<&str> {
        self.source_location_arn.as_deref()
    }
    /// <p>The entry point to use for the source code when running the canary.</p>
    pub fn handler(&self) -> ::std::option::Option<&str> {
        self.handler.as_deref()
    }
}
impl CanaryCodeOutput {
    /// Creates a new builder-style object to manufacture [`CanaryCodeOutput`](crate::types::CanaryCodeOutput).
    pub fn builder() -> crate::types::builders::CanaryCodeOutputBuilder {
        crate::types::builders::CanaryCodeOutputBuilder::default()
    }
}

/// A builder for [`CanaryCodeOutput`](crate::types::CanaryCodeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CanaryCodeOutputBuilder {
    pub(crate) source_location_arn: ::std::option::Option<::std::string::String>,
    pub(crate) handler: ::std::option::Option<::std::string::String>,
}
impl CanaryCodeOutputBuilder {
    /// <p>The ARN of the Lambda layer where Synthetics stores the canary script code.</p>
    pub fn source_location_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_location_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Lambda layer where Synthetics stores the canary script code.</p>
    pub fn set_source_location_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_location_arn = input;
        self
    }
    /// <p>The entry point to use for the source code when running the canary.</p>
    pub fn handler(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.handler = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The entry point to use for the source code when running the canary.</p>
    pub fn set_handler(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.handler = input;
        self
    }
    /// Consumes the builder and constructs a [`CanaryCodeOutput`](crate::types::CanaryCodeOutput).
    pub fn build(self) -> crate::types::CanaryCodeOutput {
        crate::types::CanaryCodeOutput {
            source_location_arn: self.source_location_arn,
            handler: self.handler,
        }
    }
}
