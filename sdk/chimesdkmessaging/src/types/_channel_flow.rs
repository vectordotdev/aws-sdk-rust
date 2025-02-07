// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of a channel flow.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ChannelFlow {
    /// <p>The ARN of the channel flow.</p>
    #[doc(hidden)]
    pub channel_flow_arn: ::std::option::Option<::std::string::String>,
    /// <p>Information about the processor Lambda functions.</p>
    #[doc(hidden)]
    pub processors: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    /// <p>The name of the channel flow.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The time at which the channel flow was created.</p>
    #[doc(hidden)]
    pub created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which a channel flow was updated.</p>
    #[doc(hidden)]
    pub last_updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ChannelFlow {
    /// <p>The ARN of the channel flow.</p>
    pub fn channel_flow_arn(&self) -> ::std::option::Option<&str> {
        self.channel_flow_arn.as_deref()
    }
    /// <p>Information about the processor Lambda functions.</p>
    pub fn processors(&self) -> ::std::option::Option<&[crate::types::Processor]> {
        self.processors.as_deref()
    }
    /// <p>The name of the channel flow.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The time at which the channel flow was created.</p>
    pub fn created_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_timestamp.as_ref()
    }
    /// <p>The time at which a channel flow was updated.</p>
    pub fn last_updated_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_timestamp.as_ref()
    }
}
impl ::std::fmt::Debug for ChannelFlow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ChannelFlow");
        formatter.field("channel_flow_arn", &self.channel_flow_arn);
        formatter.field("processors", &self.processors);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("last_updated_timestamp", &self.last_updated_timestamp);
        formatter.finish()
    }
}
impl ChannelFlow {
    /// Creates a new builder-style object to manufacture [`ChannelFlow`](crate::types::ChannelFlow).
    pub fn builder() -> crate::types::builders::ChannelFlowBuilder {
        crate::types::builders::ChannelFlowBuilder::default()
    }
}

/// A builder for [`ChannelFlow`](crate::types::ChannelFlow).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ChannelFlowBuilder {
    pub(crate) channel_flow_arn: ::std::option::Option<::std::string::String>,
    pub(crate) processors: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ChannelFlowBuilder {
    /// <p>The ARN of the channel flow.</p>
    pub fn channel_flow_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.channel_flow_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel flow.</p>
    pub fn set_channel_flow_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.channel_flow_arn = input;
        self
    }
    /// Appends an item to `processors`.
    ///
    /// To override the contents of this collection use [`set_processors`](Self::set_processors).
    ///
    /// <p>Information about the processor Lambda functions.</p>
    pub fn processors(mut self, input: crate::types::Processor) -> Self {
        let mut v = self.processors.unwrap_or_default();
        v.push(input);
        self.processors = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the processor Lambda functions.</p>
    pub fn set_processors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    ) -> Self {
        self.processors = input;
        self
    }
    /// <p>The name of the channel flow.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the channel flow.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The time at which the channel flow was created.</p>
    pub fn created_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the channel flow was created.</p>
    pub fn set_created_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_timestamp = input;
        self
    }
    /// <p>The time at which a channel flow was updated.</p>
    pub fn last_updated_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which a channel flow was updated.</p>
    pub fn set_last_updated_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`ChannelFlow`](crate::types::ChannelFlow).
    pub fn build(self) -> crate::types::ChannelFlow {
        crate::types::ChannelFlow {
            channel_flow_arn: self.channel_flow_arn,
            processors: self.processors,
            name: self.name,
            created_timestamp: self.created_timestamp,
            last_updated_timestamp: self.last_updated_timestamp,
        }
    }
}
impl ::std::fmt::Debug for ChannelFlowBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ChannelFlowBuilder");
        formatter.field("channel_flow_arn", &self.channel_flow_arn);
        formatter.field("processors", &self.processors);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("last_updated_timestamp", &self.last_updated_timestamp);
        formatter.finish()
    }
}
