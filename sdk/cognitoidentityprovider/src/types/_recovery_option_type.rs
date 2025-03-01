// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A map containing a priority as a key, and recovery method name as a value.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecoveryOptionType {
    /// <p>A positive integer specifying priority of a method with 1 being the highest priority.</p>
    #[doc(hidden)]
    pub priority: i32,
    /// <p>The recovery method for a user.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::RecoveryOptionNameType>,
}
impl RecoveryOptionType {
    /// <p>A positive integer specifying priority of a method with 1 being the highest priority.</p>
    pub fn priority(&self) -> i32 {
        self.priority
    }
    /// <p>The recovery method for a user.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::RecoveryOptionNameType> {
        self.name.as_ref()
    }
}
impl RecoveryOptionType {
    /// Creates a new builder-style object to manufacture [`RecoveryOptionType`](crate::types::RecoveryOptionType).
    pub fn builder() -> crate::types::builders::RecoveryOptionTypeBuilder {
        crate::types::builders::RecoveryOptionTypeBuilder::default()
    }
}

/// A builder for [`RecoveryOptionType`](crate::types::RecoveryOptionType).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecoveryOptionTypeBuilder {
    pub(crate) priority: ::std::option::Option<i32>,
    pub(crate) name: ::std::option::Option<crate::types::RecoveryOptionNameType>,
}
impl RecoveryOptionTypeBuilder {
    /// <p>A positive integer specifying priority of a method with 1 being the highest priority.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = ::std::option::Option::Some(input);
        self
    }
    /// <p>A positive integer specifying priority of a method with 1 being the highest priority.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// <p>The recovery method for a user.</p>
    pub fn name(mut self, input: crate::types::RecoveryOptionNameType) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The recovery method for a user.</p>
    pub fn set_name(
        mut self,
        input: ::std::option::Option<crate::types::RecoveryOptionNameType>,
    ) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`RecoveryOptionType`](crate::types::RecoveryOptionType).
    pub fn build(self) -> crate::types::RecoveryOptionType {
        crate::types::RecoveryOptionType {
            priority: self.priority.unwrap_or_default(),
            name: self.name,
        }
    }
}
