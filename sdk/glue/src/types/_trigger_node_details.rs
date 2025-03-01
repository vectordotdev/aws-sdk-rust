// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of a Trigger node present in the workflow.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TriggerNodeDetails {
    /// <p>The information of the trigger represented by the trigger node.</p>
    #[doc(hidden)]
    pub trigger: ::std::option::Option<crate::types::Trigger>,
}
impl TriggerNodeDetails {
    /// <p>The information of the trigger represented by the trigger node.</p>
    pub fn trigger(&self) -> ::std::option::Option<&crate::types::Trigger> {
        self.trigger.as_ref()
    }
}
impl TriggerNodeDetails {
    /// Creates a new builder-style object to manufacture [`TriggerNodeDetails`](crate::types::TriggerNodeDetails).
    pub fn builder() -> crate::types::builders::TriggerNodeDetailsBuilder {
        crate::types::builders::TriggerNodeDetailsBuilder::default()
    }
}

/// A builder for [`TriggerNodeDetails`](crate::types::TriggerNodeDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TriggerNodeDetailsBuilder {
    pub(crate) trigger: ::std::option::Option<crate::types::Trigger>,
}
impl TriggerNodeDetailsBuilder {
    /// <p>The information of the trigger represented by the trigger node.</p>
    pub fn trigger(mut self, input: crate::types::Trigger) -> Self {
        self.trigger = ::std::option::Option::Some(input);
        self
    }
    /// <p>The information of the trigger represented by the trigger node.</p>
    pub fn set_trigger(mut self, input: ::std::option::Option<crate::types::Trigger>) -> Self {
        self.trigger = input;
        self
    }
    /// Consumes the builder and constructs a [`TriggerNodeDetails`](crate::types::TriggerNodeDetails).
    pub fn build(self) -> crate::types::TriggerNodeDetails {
        crate::types::TriggerNodeDetails {
            trigger: self.trigger,
        }
    }
}
