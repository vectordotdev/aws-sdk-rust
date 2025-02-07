// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>When you configure a SQL-based Kinesis Data Analytics application's output, identifies a Kinesis data stream as the destination. You provide the stream Amazon Resource Name (ARN). </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KinesisStreamsOutput {
    /// <p>The ARN of the destination Kinesis data stream to write to.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl KinesisStreamsOutput {
    /// <p>The ARN of the destination Kinesis data stream to write to.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl KinesisStreamsOutput {
    /// Creates a new builder-style object to manufacture [`KinesisStreamsOutput`](crate::types::KinesisStreamsOutput).
    pub fn builder() -> crate::types::builders::KinesisStreamsOutputBuilder {
        crate::types::builders::KinesisStreamsOutputBuilder::default()
    }
}

/// A builder for [`KinesisStreamsOutput`](crate::types::KinesisStreamsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KinesisStreamsOutputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl KinesisStreamsOutputBuilder {
    /// <p>The ARN of the destination Kinesis data stream to write to.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the destination Kinesis data stream to write to.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`KinesisStreamsOutput`](crate::types::KinesisStreamsOutput).
    pub fn build(self) -> crate::types::KinesisStreamsOutput {
        crate::types::KinesisStreamsOutput {
            resource_arn: self.resource_arn,
        }
    }
}
