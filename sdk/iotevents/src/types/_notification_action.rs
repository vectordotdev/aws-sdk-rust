// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the notification settings of an alarm model. The settings apply to all alarms that were created based on this alarm model.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NotificationAction {
    /// <p>Specifies an AWS Lambda function to manage alarm notifications. You can create one or use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">AWS Lambda function provided by AWS IoT Events</a>.</p>
    #[doc(hidden)]
    pub action: ::std::option::Option<crate::types::NotificationTargetActions>,
    /// <p>Contains the configuration information of SMS notifications.</p>
    #[doc(hidden)]
    pub sms_configurations: ::std::option::Option<::std::vec::Vec<crate::types::SmsConfiguration>>,
    /// <p>Contains the configuration information of email notifications.</p>
    #[doc(hidden)]
    pub email_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::EmailConfiguration>>,
}
impl NotificationAction {
    /// <p>Specifies an AWS Lambda function to manage alarm notifications. You can create one or use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">AWS Lambda function provided by AWS IoT Events</a>.</p>
    pub fn action(&self) -> ::std::option::Option<&crate::types::NotificationTargetActions> {
        self.action.as_ref()
    }
    /// <p>Contains the configuration information of SMS notifications.</p>
    pub fn sms_configurations(&self) -> ::std::option::Option<&[crate::types::SmsConfiguration]> {
        self.sms_configurations.as_deref()
    }
    /// <p>Contains the configuration information of email notifications.</p>
    pub fn email_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::EmailConfiguration]> {
        self.email_configurations.as_deref()
    }
}
impl NotificationAction {
    /// Creates a new builder-style object to manufacture [`NotificationAction`](crate::types::NotificationAction).
    pub fn builder() -> crate::types::builders::NotificationActionBuilder {
        crate::types::builders::NotificationActionBuilder::default()
    }
}

/// A builder for [`NotificationAction`](crate::types::NotificationAction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NotificationActionBuilder {
    pub(crate) action: ::std::option::Option<crate::types::NotificationTargetActions>,
    pub(crate) sms_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::SmsConfiguration>>,
    pub(crate) email_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::EmailConfiguration>>,
}
impl NotificationActionBuilder {
    /// <p>Specifies an AWS Lambda function to manage alarm notifications. You can create one or use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">AWS Lambda function provided by AWS IoT Events</a>.</p>
    pub fn action(mut self, input: crate::types::NotificationTargetActions) -> Self {
        self.action = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies an AWS Lambda function to manage alarm notifications. You can create one or use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">AWS Lambda function provided by AWS IoT Events</a>.</p>
    pub fn set_action(
        mut self,
        input: ::std::option::Option<crate::types::NotificationTargetActions>,
    ) -> Self {
        self.action = input;
        self
    }
    /// Appends an item to `sms_configurations`.
    ///
    /// To override the contents of this collection use [`set_sms_configurations`](Self::set_sms_configurations).
    ///
    /// <p>Contains the configuration information of SMS notifications.</p>
    pub fn sms_configurations(mut self, input: crate::types::SmsConfiguration) -> Self {
        let mut v = self.sms_configurations.unwrap_or_default();
        v.push(input);
        self.sms_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains the configuration information of SMS notifications.</p>
    pub fn set_sms_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SmsConfiguration>>,
    ) -> Self {
        self.sms_configurations = input;
        self
    }
    /// Appends an item to `email_configurations`.
    ///
    /// To override the contents of this collection use [`set_email_configurations`](Self::set_email_configurations).
    ///
    /// <p>Contains the configuration information of email notifications.</p>
    pub fn email_configurations(mut self, input: crate::types::EmailConfiguration) -> Self {
        let mut v = self.email_configurations.unwrap_or_default();
        v.push(input);
        self.email_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains the configuration information of email notifications.</p>
    pub fn set_email_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EmailConfiguration>>,
    ) -> Self {
        self.email_configurations = input;
        self
    }
    /// Consumes the builder and constructs a [`NotificationAction`](crate::types::NotificationAction).
    pub fn build(self) -> crate::types::NotificationAction {
        crate::types::NotificationAction {
            action: self.action,
            sms_configurations: self.sms_configurations,
            email_configurations: self.email_configurations,
        }
    }
}
