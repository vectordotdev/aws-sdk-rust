// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateChannelFlowInput {
    /// <p>The ARN of the channel flow request.</p>
    #[doc(hidden)]
    pub app_instance_arn: ::std::option::Option<::std::string::String>,
    /// <p>Information about the processor Lambda functions.</p>
    #[doc(hidden)]
    pub processors: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    /// <p>The name of the channel flow.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The tags for the creation request.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The client token for the request. An Idempotency token.</p>
    #[doc(hidden)]
    pub client_request_token: ::std::option::Option<::std::string::String>,
}
impl CreateChannelFlowInput {
    /// <p>The ARN of the channel flow request.</p>
    pub fn app_instance_arn(&self) -> ::std::option::Option<&str> {
        self.app_instance_arn.as_deref()
    }
    /// <p>Information about the processor Lambda functions.</p>
    pub fn processors(&self) -> ::std::option::Option<&[crate::types::Processor]> {
        self.processors.as_deref()
    }
    /// <p>The name of the channel flow.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The tags for the creation request.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The client token for the request. An Idempotency token.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
}
impl ::std::fmt::Debug for CreateChannelFlowInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateChannelFlowInput");
        formatter.field("app_instance_arn", &self.app_instance_arn);
        formatter.field("processors", &self.processors);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("tags", &self.tags);
        formatter.field("client_request_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CreateChannelFlowInput {
    /// Creates a new builder-style object to manufacture [`CreateChannelFlowInput`](crate::operation::create_channel_flow::CreateChannelFlowInput).
    pub fn builder(
    ) -> crate::operation::create_channel_flow::builders::CreateChannelFlowInputBuilder {
        crate::operation::create_channel_flow::builders::CreateChannelFlowInputBuilder::default()
    }
}

/// A builder for [`CreateChannelFlowInput`](crate::operation::create_channel_flow::CreateChannelFlowInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CreateChannelFlowInputBuilder {
    pub(crate) app_instance_arn: ::std::option::Option<::std::string::String>,
    pub(crate) processors: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
}
impl CreateChannelFlowInputBuilder {
    /// <p>The ARN of the channel flow request.</p>
    pub fn app_instance_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_instance_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel flow request.</p>
    pub fn set_app_instance_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_instance_arn = input;
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
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the creation request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags for the creation request.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The client token for the request. An Idempotency token.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The client token for the request. An Idempotency token.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.client_request_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateChannelFlowInput`](crate::operation::create_channel_flow::CreateChannelFlowInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_channel_flow::CreateChannelFlowInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_channel_flow::CreateChannelFlowInput {
                app_instance_arn: self.app_instance_arn,
                processors: self.processors,
                name: self.name,
                tags: self.tags,
                client_request_token: self.client_request_token,
            },
        )
    }
}
impl ::std::fmt::Debug for CreateChannelFlowInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateChannelFlowInputBuilder");
        formatter.field("app_instance_arn", &self.app_instance_arn);
        formatter.field("processors", &self.processors);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("tags", &self.tags);
        formatter.field("client_request_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
