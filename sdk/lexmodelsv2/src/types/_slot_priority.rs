// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Sets the priority that Amazon Lex should use when eliciting slot values from a user.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SlotPriority {
    /// <p>The priority that a slot should be elicited.</p>
    #[doc(hidden)]
    pub priority: ::std::option::Option<i32>,
    /// <p>The unique identifier of the slot.</p>
    #[doc(hidden)]
    pub slot_id: ::std::option::Option<::std::string::String>,
}
impl SlotPriority {
    /// <p>The priority that a slot should be elicited.</p>
    pub fn priority(&self) -> ::std::option::Option<i32> {
        self.priority
    }
    /// <p>The unique identifier of the slot.</p>
    pub fn slot_id(&self) -> ::std::option::Option<&str> {
        self.slot_id.as_deref()
    }
}
impl SlotPriority {
    /// Creates a new builder-style object to manufacture [`SlotPriority`](crate::types::SlotPriority).
    pub fn builder() -> crate::types::builders::SlotPriorityBuilder {
        crate::types::builders::SlotPriorityBuilder::default()
    }
}

/// A builder for [`SlotPriority`](crate::types::SlotPriority).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SlotPriorityBuilder {
    pub(crate) priority: ::std::option::Option<i32>,
    pub(crate) slot_id: ::std::option::Option<::std::string::String>,
}
impl SlotPriorityBuilder {
    /// <p>The priority that a slot should be elicited.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = ::std::option::Option::Some(input);
        self
    }
    /// <p>The priority that a slot should be elicited.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// <p>The unique identifier of the slot.</p>
    pub fn slot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.slot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the slot.</p>
    pub fn set_slot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.slot_id = input;
        self
    }
    /// Consumes the builder and constructs a [`SlotPriority`](crate::types::SlotPriority).
    pub fn build(self) -> crate::types::SlotPriority {
        crate::types::SlotPriority {
            priority: self.priority,
            slot_id: self.slot_id,
        }
    }
}
