// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides a description of Amazon CloudWatch logging options, including the log stream Amazon Resource Name (ARN). </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudWatchLoggingOption {
    /// <p>The ARN of the CloudWatch log to receive application messages.</p>
    #[doc(hidden)]
    pub log_stream_arn: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLoggingOption {
    /// <p>The ARN of the CloudWatch log to receive application messages.</p>
    pub fn log_stream_arn(&self) -> ::std::option::Option<&str> {
        self.log_stream_arn.as_deref()
    }
}
impl CloudWatchLoggingOption {
    /// Creates a new builder-style object to manufacture [`CloudWatchLoggingOption`](crate::types::CloudWatchLoggingOption).
    pub fn builder() -> crate::types::builders::CloudWatchLoggingOptionBuilder {
        crate::types::builders::CloudWatchLoggingOptionBuilder::default()
    }
}

/// A builder for [`CloudWatchLoggingOption`](crate::types::CloudWatchLoggingOption).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CloudWatchLoggingOptionBuilder {
    pub(crate) log_stream_arn: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLoggingOptionBuilder {
    /// <p>The ARN of the CloudWatch log to receive application messages.</p>
    pub fn log_stream_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.log_stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the CloudWatch log to receive application messages.</p>
    pub fn set_log_stream_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.log_stream_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`CloudWatchLoggingOption`](crate::types::CloudWatchLoggingOption).
    pub fn build(self) -> crate::types::CloudWatchLoggingOption {
        crate::types::CloudWatchLoggingOption {
            log_stream_arn: self.log_stream_arn,
        }
    }
}
