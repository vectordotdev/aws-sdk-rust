// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateQueueInput {
    /// Optional. A description of the queue that you are creating.
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// The name of the queue that you are creating.
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    #[doc(hidden)]
    pub pricing_plan: ::std::option::Option<crate::types::PricingPlan>,
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    #[doc(hidden)]
    pub reservation_plan_settings: ::std::option::Option<crate::types::ReservationPlanSettings>,
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::QueueStatus>,
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateQueueInput {
    /// Optional. A description of the queue that you are creating.
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// The name of the queue that you are creating.
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    pub fn pricing_plan(&self) -> ::std::option::Option<&crate::types::PricingPlan> {
        self.pricing_plan.as_ref()
    }
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    pub fn reservation_plan_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::ReservationPlanSettings> {
        self.reservation_plan_settings.as_ref()
    }
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    pub fn status(&self) -> ::std::option::Option<&crate::types::QueueStatus> {
        self.status.as_ref()
    }
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl CreateQueueInput {
    /// Creates a new builder-style object to manufacture [`CreateQueueInput`](crate::operation::create_queue::CreateQueueInput).
    pub fn builder() -> crate::operation::create_queue::builders::CreateQueueInputBuilder {
        crate::operation::create_queue::builders::CreateQueueInputBuilder::default()
    }
}

/// A builder for [`CreateQueueInput`](crate::operation::create_queue::CreateQueueInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateQueueInputBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) pricing_plan: ::std::option::Option<crate::types::PricingPlan>,
    pub(crate) reservation_plan_settings:
        ::std::option::Option<crate::types::ReservationPlanSettings>,
    pub(crate) status: ::std::option::Option<crate::types::QueueStatus>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateQueueInputBuilder {
    /// Optional. A description of the queue that you are creating.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// Optional. A description of the queue that you are creating.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// The name of the queue that you are creating.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the queue that you are creating.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.pricing_plan = ::std::option::Option::Some(input);
        self
    }
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    pub fn set_pricing_plan(
        mut self,
        input: ::std::option::Option<crate::types::PricingPlan>,
    ) -> Self {
        self.pricing_plan = input;
        self
    }
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    pub fn reservation_plan_settings(
        mut self,
        input: crate::types::ReservationPlanSettings,
    ) -> Self {
        self.reservation_plan_settings = ::std::option::Option::Some(input);
        self
    }
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    pub fn set_reservation_plan_settings(
        mut self,
        input: ::std::option::Option<crate::types::ReservationPlanSettings>,
    ) -> Self {
        self.reservation_plan_settings = input;
        self
    }
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    pub fn status(mut self, input: crate::types::QueueStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::QueueStatus>) -> Self {
        self.status = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateQueueInput`](crate::operation::create_queue::CreateQueueInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_queue::CreateQueueInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_queue::CreateQueueInput {
            description: self.description,
            name: self.name,
            pricing_plan: self.pricing_plan,
            reservation_plan_settings: self.reservation_plan_settings,
            status: self.status,
            tags: self.tags,
        })
    }
}
