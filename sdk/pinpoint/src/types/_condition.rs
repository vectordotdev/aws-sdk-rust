// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the conditions to evaluate for an activity in a journey, and how to evaluate those conditions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Condition {
    /// <p>The conditions to evaluate for the activity.</p>
    #[doc(hidden)]
    pub conditions: ::std::option::Option<::std::vec::Vec<crate::types::SimpleCondition>>,
    /// <p>Specifies how to handle multiple conditions for the activity. For example, if you specify two conditions for an activity, whether both or only one of the conditions must be met for the activity to be performed.</p>
    #[doc(hidden)]
    pub operator: ::std::option::Option<crate::types::Operator>,
}
impl Condition {
    /// <p>The conditions to evaluate for the activity.</p>
    pub fn conditions(&self) -> ::std::option::Option<&[crate::types::SimpleCondition]> {
        self.conditions.as_deref()
    }
    /// <p>Specifies how to handle multiple conditions for the activity. For example, if you specify two conditions for an activity, whether both or only one of the conditions must be met for the activity to be performed.</p>
    pub fn operator(&self) -> ::std::option::Option<&crate::types::Operator> {
        self.operator.as_ref()
    }
}
impl Condition {
    /// Creates a new builder-style object to manufacture [`Condition`](crate::types::Condition).
    pub fn builder() -> crate::types::builders::ConditionBuilder {
        crate::types::builders::ConditionBuilder::default()
    }
}

/// A builder for [`Condition`](crate::types::Condition).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConditionBuilder {
    pub(crate) conditions: ::std::option::Option<::std::vec::Vec<crate::types::SimpleCondition>>,
    pub(crate) operator: ::std::option::Option<crate::types::Operator>,
}
impl ConditionBuilder {
    /// Appends an item to `conditions`.
    ///
    /// To override the contents of this collection use [`set_conditions`](Self::set_conditions).
    ///
    /// <p>The conditions to evaluate for the activity.</p>
    pub fn conditions(mut self, input: crate::types::SimpleCondition) -> Self {
        let mut v = self.conditions.unwrap_or_default();
        v.push(input);
        self.conditions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The conditions to evaluate for the activity.</p>
    pub fn set_conditions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SimpleCondition>>,
    ) -> Self {
        self.conditions = input;
        self
    }
    /// <p>Specifies how to handle multiple conditions for the activity. For example, if you specify two conditions for an activity, whether both or only one of the conditions must be met for the activity to be performed.</p>
    pub fn operator(mut self, input: crate::types::Operator) -> Self {
        self.operator = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how to handle multiple conditions for the activity. For example, if you specify two conditions for an activity, whether both or only one of the conditions must be met for the activity to be performed.</p>
    pub fn set_operator(mut self, input: ::std::option::Option<crate::types::Operator>) -> Self {
        self.operator = input;
        self
    }
    /// Consumes the builder and constructs a [`Condition`](crate::types::Condition).
    pub fn build(self) -> crate::types::Condition {
        crate::types::Condition {
            conditions: self.conditions,
            operator: self.operator,
        }
    }
}
