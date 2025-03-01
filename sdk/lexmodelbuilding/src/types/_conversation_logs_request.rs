// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the settings needed for conversation logs.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConversationLogsRequest {
    /// <p>The settings for your conversation logs. You can log the conversation text, conversation audio, or both.</p>
    #[doc(hidden)]
    pub log_settings: ::std::option::Option<::std::vec::Vec<crate::types::LogSettingsRequest>>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to write to your CloudWatch Logs for text logs and your S3 bucket for audio logs. If audio encryption is enabled, this role also provides access permission for the AWS KMS key used for encrypting audio logs. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/conversation-logs-role-and-policy.html">Creating an IAM Role and Policy for Conversation Logs</a>.</p>
    #[doc(hidden)]
    pub iam_role_arn: ::std::option::Option<::std::string::String>,
}
impl ConversationLogsRequest {
    /// <p>The settings for your conversation logs. You can log the conversation text, conversation audio, or both.</p>
    pub fn log_settings(&self) -> ::std::option::Option<&[crate::types::LogSettingsRequest]> {
        self.log_settings.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to write to your CloudWatch Logs for text logs and your S3 bucket for audio logs. If audio encryption is enabled, this role also provides access permission for the AWS KMS key used for encrypting audio logs. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/conversation-logs-role-and-policy.html">Creating an IAM Role and Policy for Conversation Logs</a>.</p>
    pub fn iam_role_arn(&self) -> ::std::option::Option<&str> {
        self.iam_role_arn.as_deref()
    }
}
impl ConversationLogsRequest {
    /// Creates a new builder-style object to manufacture [`ConversationLogsRequest`](crate::types::ConversationLogsRequest).
    pub fn builder() -> crate::types::builders::ConversationLogsRequestBuilder {
        crate::types::builders::ConversationLogsRequestBuilder::default()
    }
}

/// A builder for [`ConversationLogsRequest`](crate::types::ConversationLogsRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConversationLogsRequestBuilder {
    pub(crate) log_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::LogSettingsRequest>>,
    pub(crate) iam_role_arn: ::std::option::Option<::std::string::String>,
}
impl ConversationLogsRequestBuilder {
    /// Appends an item to `log_settings`.
    ///
    /// To override the contents of this collection use [`set_log_settings`](Self::set_log_settings).
    ///
    /// <p>The settings for your conversation logs. You can log the conversation text, conversation audio, or both.</p>
    pub fn log_settings(mut self, input: crate::types::LogSettingsRequest) -> Self {
        let mut v = self.log_settings.unwrap_or_default();
        v.push(input);
        self.log_settings = ::std::option::Option::Some(v);
        self
    }
    /// <p>The settings for your conversation logs. You can log the conversation text, conversation audio, or both.</p>
    pub fn set_log_settings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LogSettingsRequest>>,
    ) -> Self {
        self.log_settings = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to write to your CloudWatch Logs for text logs and your S3 bucket for audio logs. If audio encryption is enabled, this role also provides access permission for the AWS KMS key used for encrypting audio logs. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/conversation-logs-role-and-policy.html">Creating an IAM Role and Policy for Conversation Logs</a>.</p>
    pub fn iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.iam_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to write to your CloudWatch Logs for text logs and your S3 bucket for audio logs. If audio encryption is enabled, this role also provides access permission for the AWS KMS key used for encrypting audio logs. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/conversation-logs-role-and-policy.html">Creating an IAM Role and Policy for Conversation Logs</a>.</p>
    pub fn set_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.iam_role_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`ConversationLogsRequest`](crate::types::ConversationLogsRequest).
    pub fn build(self) -> crate::types::ConversationLogsRequest {
        crate::types::ConversationLogsRequest {
            log_settings: self.log_settings,
            iam_role_arn: self.iam_role_arn,
        }
    }
}
