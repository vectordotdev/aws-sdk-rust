// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateEventSubscriptionInput {
    /// <p>The name of the DMS event notification subscription. This name must be less than 255 characters.</p>
    #[doc(hidden)]
    pub subscription_name: ::std::option::Option<::std::string::String>,
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    #[doc(hidden)]
    pub sns_topic_arn: ::std::option::Option<::std::string::String>,
    /// <p> The type of DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value isn't specified, all events are returned. </p>
    /// <p>Valid values: <code>replication-instance</code> | <code>replication-task</code> </p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<::std::string::String>,
    /// <p>A list of event categories for a source type that you want to subscribe to. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>Database Migration Service User Guide.</i> </p>
    #[doc(hidden)]
    pub event_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of identifiers for which DMS provides notification events.</p>
    /// <p>If you don't specify a value, notifications are provided for all sources.</p>
    /// <p>If you specify multiple values, they must be of the same type. For example, if you specify a database instance ID, then all of the other values must be database instance IDs.</p>
    #[doc(hidden)]
    pub source_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to <code>false</code> to create the subscription but not activate it. </p>
    #[doc(hidden)]
    pub enabled: ::std::option::Option<bool>,
    /// <p>One or more tags to be assigned to the event subscription.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateEventSubscriptionInput {
    /// <p>The name of the DMS event notification subscription. This name must be less than 255 characters.</p>
    pub fn subscription_name(&self) -> ::std::option::Option<&str> {
        self.subscription_name.as_deref()
    }
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    pub fn sns_topic_arn(&self) -> ::std::option::Option<&str> {
        self.sns_topic_arn.as_deref()
    }
    /// <p> The type of DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value isn't specified, all events are returned. </p>
    /// <p>Valid values: <code>replication-instance</code> | <code>replication-task</code> </p>
    pub fn source_type(&self) -> ::std::option::Option<&str> {
        self.source_type.as_deref()
    }
    /// <p>A list of event categories for a source type that you want to subscribe to. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>Database Migration Service User Guide.</i> </p>
    pub fn event_categories(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.event_categories.as_deref()
    }
    /// <p>A list of identifiers for which DMS provides notification events.</p>
    /// <p>If you don't specify a value, notifications are provided for all sources.</p>
    /// <p>If you specify multiple values, they must be of the same type. For example, if you specify a database instance ID, then all of the other values must be database instance IDs.</p>
    pub fn source_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.source_ids.as_deref()
    }
    /// <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to <code>false</code> to create the subscription but not activate it. </p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
    /// <p>One or more tags to be assigned to the event subscription.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateEventSubscriptionInput {
    /// Creates a new builder-style object to manufacture [`CreateEventSubscriptionInput`](crate::operation::create_event_subscription::CreateEventSubscriptionInput).
    pub fn builder(
    ) -> crate::operation::create_event_subscription::builders::CreateEventSubscriptionInputBuilder
    {
        crate::operation::create_event_subscription::builders::CreateEventSubscriptionInputBuilder::default()
    }
}

/// A builder for [`CreateEventSubscriptionInput`](crate::operation::create_event_subscription::CreateEventSubscriptionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateEventSubscriptionInputBuilder {
    pub(crate) subscription_name: ::std::option::Option<::std::string::String>,
    pub(crate) sns_topic_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source_type: ::std::option::Option<::std::string::String>,
    pub(crate) event_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) source_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateEventSubscriptionInputBuilder {
    /// <p>The name of the DMS event notification subscription. This name must be less than 255 characters.</p>
    pub fn subscription_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.subscription_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the DMS event notification subscription. This name must be less than 255 characters.</p>
    pub fn set_subscription_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.subscription_name = input;
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    pub fn sns_topic_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sns_topic_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    pub fn set_sns_topic_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sns_topic_arn = input;
        self
    }
    /// <p> The type of DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value isn't specified, all events are returned. </p>
    /// <p>Valid values: <code>replication-instance</code> | <code>replication-task</code> </p>
    pub fn source_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The type of DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value isn't specified, all events are returned. </p>
    /// <p>Valid values: <code>replication-instance</code> | <code>replication-task</code> </p>
    pub fn set_source_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_type = input;
        self
    }
    /// Appends an item to `event_categories`.
    ///
    /// To override the contents of this collection use [`set_event_categories`](Self::set_event_categories).
    ///
    /// <p>A list of event categories for a source type that you want to subscribe to. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>Database Migration Service User Guide.</i> </p>
    pub fn event_categories(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.event_categories.unwrap_or_default();
        v.push(input.into());
        self.event_categories = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of event categories for a source type that you want to subscribe to. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>Database Migration Service User Guide.</i> </p>
    pub fn set_event_categories(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.event_categories = input;
        self
    }
    /// Appends an item to `source_ids`.
    ///
    /// To override the contents of this collection use [`set_source_ids`](Self::set_source_ids).
    ///
    /// <p>A list of identifiers for which DMS provides notification events.</p>
    /// <p>If you don't specify a value, notifications are provided for all sources.</p>
    /// <p>If you specify multiple values, they must be of the same type. For example, if you specify a database instance ID, then all of the other values must be database instance IDs.</p>
    pub fn source_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.source_ids.unwrap_or_default();
        v.push(input.into());
        self.source_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of identifiers for which DMS provides notification events.</p>
    /// <p>If you don't specify a value, notifications are provided for all sources.</p>
    /// <p>If you specify multiple values, they must be of the same type. For example, if you specify a database instance ID, then all of the other values must be database instance IDs.</p>
    pub fn set_source_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.source_ids = input;
        self
    }
    /// <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to <code>false</code> to create the subscription but not activate it. </p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to <code>false</code> to create the subscription but not activate it. </p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>One or more tags to be assigned to the event subscription.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more tags to be assigned to the event subscription.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateEventSubscriptionInput`](crate::operation::create_event_subscription::CreateEventSubscriptionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_event_subscription::CreateEventSubscriptionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_event_subscription::CreateEventSubscriptionInput {
                subscription_name: self.subscription_name,
                sns_topic_arn: self.sns_topic_arn,
                source_type: self.source_type,
                event_categories: self.event_categories,
                source_ids: self.source_ids,
                enabled: self.enabled,
                tags: self.tags,
            },
        )
    }
}
