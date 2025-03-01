// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateSubscriptionNotificationConfigurationOutput {
    /// <p>Returns the Amazon Resource Name (ARN) of the queue.</p>
    #[doc(hidden)]
    pub queue_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateSubscriptionNotificationConfigurationOutput {
    /// <p>Returns the Amazon Resource Name (ARN) of the queue.</p>
    pub fn queue_arn(&self) -> ::std::option::Option<&str> {
        self.queue_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateSubscriptionNotificationConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateSubscriptionNotificationConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`CreateSubscriptionNotificationConfigurationOutput`](crate::operation::create_subscription_notification_configuration::CreateSubscriptionNotificationConfigurationOutput).
    pub fn builder() -> crate::operation::create_subscription_notification_configuration::builders::CreateSubscriptionNotificationConfigurationOutputBuilder{
        crate::operation::create_subscription_notification_configuration::builders::CreateSubscriptionNotificationConfigurationOutputBuilder::default()
    }
}

/// A builder for [`CreateSubscriptionNotificationConfigurationOutput`](crate::operation::create_subscription_notification_configuration::CreateSubscriptionNotificationConfigurationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateSubscriptionNotificationConfigurationOutputBuilder {
    pub(crate) queue_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateSubscriptionNotificationConfigurationOutputBuilder {
    /// <p>Returns the Amazon Resource Name (ARN) of the queue.</p>
    pub fn queue_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns the Amazon Resource Name (ARN) of the queue.</p>
    pub fn set_queue_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_arn = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateSubscriptionNotificationConfigurationOutput`](crate::operation::create_subscription_notification_configuration::CreateSubscriptionNotificationConfigurationOutput).
    pub fn build(self) -> crate::operation::create_subscription_notification_configuration::CreateSubscriptionNotificationConfigurationOutput{
        crate::operation::create_subscription_notification_configuration::CreateSubscriptionNotificationConfigurationOutput {
            queue_arn: self.queue_arn
            ,
            _request_id: self._request_id,
        }
    }
}
