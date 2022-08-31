// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This object is used in the ValidationException error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationExceptionField {
    /// <p>Name of the field which failed validation.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>Explanation of the reason for the validation error.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ValidationExceptionField {
    /// <p>Name of the field which failed validation.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Explanation of the reason for the validation error.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Debug for ValidationExceptionField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationExceptionField");
        formatter.field("name", &self.name);
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
/// See [`ValidationExceptionField`](crate::model::ValidationExceptionField).
pub mod validation_exception_field {

    /// A builder for [`ValidationExceptionField`](crate::model::ValidationExceptionField).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Name of the field which failed validation.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>Name of the field which failed validation.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>Explanation of the reason for the validation error.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>Explanation of the reason for the validation error.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ValidationExceptionField`](crate::model::ValidationExceptionField).
        pub fn build(self) -> crate::model::ValidationExceptionField {
            crate::model::ValidationExceptionField {
                name: self.name,
                message: self.message,
            }
        }
    }
}
impl ValidationExceptionField {
    /// Creates a new builder-style object to manufacture [`ValidationExceptionField`](crate::model::ValidationExceptionField).
    pub fn builder() -> crate::model::validation_exception_field::Builder {
        crate::model::validation_exception_field::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ValidationExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    FieldValidationFailed,
    #[allow(missing_docs)] // documentation missing in model
    Other,
    #[allow(missing_docs)] // documentation missing in model
    UnknownOperation,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ValidationExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "FIELD_VALIDATION_FAILED" => ValidationExceptionReason::FieldValidationFailed,
            "OTHER" => ValidationExceptionReason::Other,
            "UNKNOWN_OPERATION" => ValidationExceptionReason::UnknownOperation,
            other => ValidationExceptionReason::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ValidationExceptionReason {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ValidationExceptionReason::from(s))
    }
}
impl ValidationExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ValidationExceptionReason::FieldValidationFailed => "FIELD_VALIDATION_FAILED",
            ValidationExceptionReason::Other => "OTHER",
            ValidationExceptionReason::UnknownOperation => "UNKNOWN_OPERATION",
            ValidationExceptionReason::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["FIELD_VALIDATION_FAILED", "OTHER", "UNKNOWN_OPERATION"]
    }
}
impl AsRef<str> for ValidationExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ResourceType {
    #[allow(missing_docs)] // documentation missing in model
    Room,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ResourceType {
    fn from(s: &str) -> Self {
        match s {
            "ROOM" => ResourceType::Room,
            other => ResourceType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ResourceType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceType::from(s))
    }
}
impl ResourceType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ResourceType::Room => "ROOM",
            ResourceType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ROOM"]
    }
}
impl AsRef<str> for ResourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Configuration information for optional message review.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MessageReviewHandler {
    /// <p>Identifier of the message review handler. Currently this must be an ARN of a lambda function.</p>
    #[doc(hidden)]
    pub uri: std::option::Option<std::string::String>,
    /// <p>Specifies the fallback behavior (whether the message is allowed or denied) if the handler does not return a valid response, encounters an error, or times out. (For the timeout period, see <a href="https://docs.aws.amazon.com/ivs/latest/userguide/service-quotas.html"> Service Quotas</a>.) If allowed, the message is delivered with returned content to all users connected to the room. If denied, the message is not delivered to any user. Default: <code>ALLOW</code>.</p>
    #[doc(hidden)]
    pub fallback_result: std::option::Option<crate::model::FallbackResult>,
}
impl MessageReviewHandler {
    /// <p>Identifier of the message review handler. Currently this must be an ARN of a lambda function.</p>
    pub fn uri(&self) -> std::option::Option<&str> {
        self.uri.as_deref()
    }
    /// <p>Specifies the fallback behavior (whether the message is allowed or denied) if the handler does not return a valid response, encounters an error, or times out. (For the timeout period, see <a href="https://docs.aws.amazon.com/ivs/latest/userguide/service-quotas.html"> Service Quotas</a>.) If allowed, the message is delivered with returned content to all users connected to the room. If denied, the message is not delivered to any user. Default: <code>ALLOW</code>.</p>
    pub fn fallback_result(&self) -> std::option::Option<&crate::model::FallbackResult> {
        self.fallback_result.as_ref()
    }
}
impl std::fmt::Debug for MessageReviewHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MessageReviewHandler");
        formatter.field("uri", &self.uri);
        formatter.field("fallback_result", &self.fallback_result);
        formatter.finish()
    }
}
/// See [`MessageReviewHandler`](crate::model::MessageReviewHandler).
pub mod message_review_handler {

    /// A builder for [`MessageReviewHandler`](crate::model::MessageReviewHandler).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) uri: std::option::Option<std::string::String>,
        pub(crate) fallback_result: std::option::Option<crate::model::FallbackResult>,
    }
    impl Builder {
        /// <p>Identifier of the message review handler. Currently this must be an ARN of a lambda function.</p>
        pub fn uri(mut self, input: impl Into<std::string::String>) -> Self {
            self.uri = Some(input.into());
            self
        }
        /// <p>Identifier of the message review handler. Currently this must be an ARN of a lambda function.</p>
        pub fn set_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.uri = input;
            self
        }
        /// <p>Specifies the fallback behavior (whether the message is allowed or denied) if the handler does not return a valid response, encounters an error, or times out. (For the timeout period, see <a href="https://docs.aws.amazon.com/ivs/latest/userguide/service-quotas.html"> Service Quotas</a>.) If allowed, the message is delivered with returned content to all users connected to the room. If denied, the message is not delivered to any user. Default: <code>ALLOW</code>.</p>
        pub fn fallback_result(mut self, input: crate::model::FallbackResult) -> Self {
            self.fallback_result = Some(input);
            self
        }
        /// <p>Specifies the fallback behavior (whether the message is allowed or denied) if the handler does not return a valid response, encounters an error, or times out. (For the timeout period, see <a href="https://docs.aws.amazon.com/ivs/latest/userguide/service-quotas.html"> Service Quotas</a>.) If allowed, the message is delivered with returned content to all users connected to the room. If denied, the message is not delivered to any user. Default: <code>ALLOW</code>.</p>
        pub fn set_fallback_result(
            mut self,
            input: std::option::Option<crate::model::FallbackResult>,
        ) -> Self {
            self.fallback_result = input;
            self
        }
        /// Consumes the builder and constructs a [`MessageReviewHandler`](crate::model::MessageReviewHandler).
        pub fn build(self) -> crate::model::MessageReviewHandler {
            crate::model::MessageReviewHandler {
                uri: self.uri,
                fallback_result: self.fallback_result,
            }
        }
    }
}
impl MessageReviewHandler {
    /// Creates a new builder-style object to manufacture [`MessageReviewHandler`](crate::model::MessageReviewHandler).
    pub fn builder() -> crate::model::message_review_handler::Builder {
        crate::model::message_review_handler::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum FallbackResult {
    #[allow(missing_docs)] // documentation missing in model
    Allow,
    #[allow(missing_docs)] // documentation missing in model
    Deny,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for FallbackResult {
    fn from(s: &str) -> Self {
        match s {
            "ALLOW" => FallbackResult::Allow,
            "DENY" => FallbackResult::Deny,
            other => FallbackResult::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for FallbackResult {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(FallbackResult::from(s))
    }
}
impl FallbackResult {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            FallbackResult::Allow => "ALLOW",
            FallbackResult::Deny => "DENY",
            FallbackResult::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ALLOW", "DENY"]
    }
}
impl AsRef<str> for FallbackResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Summary information about a room.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RoomSummary {
    /// <p>Room ARN.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>Room ID, generated by the system. This is a relative identifier, the part of the ARN that uniquely identifies the room.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>Room name. The value does not need to be unique.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>Configuration information for optional review of messages.</p>
    #[doc(hidden)]
    pub message_review_handler: std::option::Option<crate::model::MessageReviewHandler>,
    /// <p>Time when the room was created. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
    #[doc(hidden)]
    pub create_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Time of the room’s last update. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
    #[doc(hidden)]
    pub update_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Tags attached to the resource. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS Chat has no constraints beyond what is documented there.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl RoomSummary {
    /// <p>Room ARN.</p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Room ID, generated by the system. This is a relative identifier, the part of the ARN that uniquely identifies the room.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Room name. The value does not need to be unique.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Configuration information for optional review of messages.</p>
    pub fn message_review_handler(
        &self,
    ) -> std::option::Option<&crate::model::MessageReviewHandler> {
        self.message_review_handler.as_ref()
    }
    /// <p>Time when the room was created. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
    pub fn create_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>Time of the room’s last update. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
    pub fn update_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
    /// <p>Tags attached to the resource. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS Chat has no constraints beyond what is documented there.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for RoomSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RoomSummary");
        formatter.field("arn", &self.arn);
        formatter.field("id", &self.id);
        formatter.field("name", &self.name);
        formatter.field("message_review_handler", &self.message_review_handler);
        formatter.field("create_time", &self.create_time);
        formatter.field("update_time", &self.update_time);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`RoomSummary`](crate::model::RoomSummary).
pub mod room_summary {

    /// A builder for [`RoomSummary`](crate::model::RoomSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) message_review_handler: std::option::Option<crate::model::MessageReviewHandler>,
        pub(crate) create_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) update_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>Room ARN.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>Room ARN.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// <p>Room ID, generated by the system. This is a relative identifier, the part of the ARN that uniquely identifies the room.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>Room ID, generated by the system. This is a relative identifier, the part of the ARN that uniquely identifies the room.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// <p>Room name. The value does not need to be unique.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>Room name. The value does not need to be unique.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>Configuration information for optional review of messages.</p>
        pub fn message_review_handler(mut self, input: crate::model::MessageReviewHandler) -> Self {
            self.message_review_handler = Some(input);
            self
        }
        /// <p>Configuration information for optional review of messages.</p>
        pub fn set_message_review_handler(
            mut self,
            input: std::option::Option<crate::model::MessageReviewHandler>,
        ) -> Self {
            self.message_review_handler = input;
            self
        }
        /// <p>Time when the room was created. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
        pub fn create_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.create_time = Some(input);
            self
        }
        /// <p>Time when the room was created. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
        pub fn set_create_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.create_time = input;
            self
        }
        /// <p>Time of the room’s last update. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
        pub fn update_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.update_time = Some(input);
            self
        }
        /// <p>Time of the room’s last update. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>. </p>
        pub fn set_update_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.update_time = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Tags attached to the resource. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS Chat has no constraints beyond what is documented there.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>Tags attached to the resource. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS Chat has no constraints beyond what is documented there.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`RoomSummary`](crate::model::RoomSummary).
        pub fn build(self) -> crate::model::RoomSummary {
            crate::model::RoomSummary {
                arn: self.arn,
                id: self.id,
                name: self.name,
                message_review_handler: self.message_review_handler,
                create_time: self.create_time,
                update_time: self.update_time,
                tags: self.tags,
            }
        }
    }
}
impl RoomSummary {
    /// Creates a new builder-style object to manufacture [`RoomSummary`](crate::model::RoomSummary).
    pub fn builder() -> crate::model::room_summary::Builder {
        crate::model::room_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ChatTokenCapability {
    #[allow(missing_docs)] // documentation missing in model
    DeleteMessage,
    #[allow(missing_docs)] // documentation missing in model
    DisconnectUser,
    #[allow(missing_docs)] // documentation missing in model
    SendMessage,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ChatTokenCapability {
    fn from(s: &str) -> Self {
        match s {
            "DELETE_MESSAGE" => ChatTokenCapability::DeleteMessage,
            "DISCONNECT_USER" => ChatTokenCapability::DisconnectUser,
            "SEND_MESSAGE" => ChatTokenCapability::SendMessage,
            other => ChatTokenCapability::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ChatTokenCapability {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ChatTokenCapability::from(s))
    }
}
impl ChatTokenCapability {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ChatTokenCapability::DeleteMessage => "DELETE_MESSAGE",
            ChatTokenCapability::DisconnectUser => "DISCONNECT_USER",
            ChatTokenCapability::SendMessage => "SEND_MESSAGE",
            ChatTokenCapability::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["DELETE_MESSAGE", "DISCONNECT_USER", "SEND_MESSAGE"]
    }
}
impl AsRef<str> for ChatTokenCapability {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
