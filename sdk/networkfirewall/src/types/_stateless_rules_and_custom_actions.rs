// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Stateless inspection criteria. Each stateless rule group uses exactly one of these data types to define its stateless rules. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StatelessRulesAndCustomActions {
    /// <p>Defines the set of stateless rules for use in a stateless rule group. </p>
    #[doc(hidden)]
    pub stateless_rules: ::std::option::Option<::std::vec::Vec<crate::types::StatelessRule>>,
    /// <p>Defines an array of individual custom action definitions that are available for use by the stateless rules in this <code>StatelessRulesAndCustomActions</code> specification. You name each custom action that you define, and then you can use it by name in your <code>StatelessRule</code> <code>RuleDefinition</code> <code>Actions</code> specification.</p>
    #[doc(hidden)]
    pub custom_actions: ::std::option::Option<::std::vec::Vec<crate::types::CustomAction>>,
}
impl StatelessRulesAndCustomActions {
    /// <p>Defines the set of stateless rules for use in a stateless rule group. </p>
    pub fn stateless_rules(&self) -> ::std::option::Option<&[crate::types::StatelessRule]> {
        self.stateless_rules.as_deref()
    }
    /// <p>Defines an array of individual custom action definitions that are available for use by the stateless rules in this <code>StatelessRulesAndCustomActions</code> specification. You name each custom action that you define, and then you can use it by name in your <code>StatelessRule</code> <code>RuleDefinition</code> <code>Actions</code> specification.</p>
    pub fn custom_actions(&self) -> ::std::option::Option<&[crate::types::CustomAction]> {
        self.custom_actions.as_deref()
    }
}
impl StatelessRulesAndCustomActions {
    /// Creates a new builder-style object to manufacture [`StatelessRulesAndCustomActions`](crate::types::StatelessRulesAndCustomActions).
    pub fn builder() -> crate::types::builders::StatelessRulesAndCustomActionsBuilder {
        crate::types::builders::StatelessRulesAndCustomActionsBuilder::default()
    }
}

/// A builder for [`StatelessRulesAndCustomActions`](crate::types::StatelessRulesAndCustomActions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StatelessRulesAndCustomActionsBuilder {
    pub(crate) stateless_rules: ::std::option::Option<::std::vec::Vec<crate::types::StatelessRule>>,
    pub(crate) custom_actions: ::std::option::Option<::std::vec::Vec<crate::types::CustomAction>>,
}
impl StatelessRulesAndCustomActionsBuilder {
    /// Appends an item to `stateless_rules`.
    ///
    /// To override the contents of this collection use [`set_stateless_rules`](Self::set_stateless_rules).
    ///
    /// <p>Defines the set of stateless rules for use in a stateless rule group. </p>
    pub fn stateless_rules(mut self, input: crate::types::StatelessRule) -> Self {
        let mut v = self.stateless_rules.unwrap_or_default();
        v.push(input);
        self.stateless_rules = ::std::option::Option::Some(v);
        self
    }
    /// <p>Defines the set of stateless rules for use in a stateless rule group. </p>
    pub fn set_stateless_rules(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StatelessRule>>,
    ) -> Self {
        self.stateless_rules = input;
        self
    }
    /// Appends an item to `custom_actions`.
    ///
    /// To override the contents of this collection use [`set_custom_actions`](Self::set_custom_actions).
    ///
    /// <p>Defines an array of individual custom action definitions that are available for use by the stateless rules in this <code>StatelessRulesAndCustomActions</code> specification. You name each custom action that you define, and then you can use it by name in your <code>StatelessRule</code> <code>RuleDefinition</code> <code>Actions</code> specification.</p>
    pub fn custom_actions(mut self, input: crate::types::CustomAction) -> Self {
        let mut v = self.custom_actions.unwrap_or_default();
        v.push(input);
        self.custom_actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Defines an array of individual custom action definitions that are available for use by the stateless rules in this <code>StatelessRulesAndCustomActions</code> specification. You name each custom action that you define, and then you can use it by name in your <code>StatelessRule</code> <code>RuleDefinition</code> <code>Actions</code> specification.</p>
    pub fn set_custom_actions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CustomAction>>,
    ) -> Self {
        self.custom_actions = input;
        self
    }
    /// Consumes the builder and constructs a [`StatelessRulesAndCustomActions`](crate::types::StatelessRulesAndCustomActions).
    pub fn build(self) -> crate::types::StatelessRulesAndCustomActions {
        crate::types::StatelessRulesAndCustomActions {
            stateless_rules: self.stateless_rules,
            custom_actions: self.custom_actions,
        }
    }
}
