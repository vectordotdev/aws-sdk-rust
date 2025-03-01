// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the automation option based on a rule category for a single select question.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SingleSelectQuestionRuleCategoryAutomation {
    /// <p> The category name, as defined in Rules.</p>
    #[doc(hidden)]
    pub category: ::std::option::Option<::std::string::String>,
    /// <p>The condition to apply for the automation option. If the condition is <code>PRESENT</code>, then the option is applied when the contact data includes the category. Similarly, if the condition is <code>NOT_PRESENT</code>, then the option is applied when the contact data does not include the category.</p>
    #[doc(hidden)]
    pub condition:
        ::std::option::Option<crate::types::SingleSelectQuestionRuleCategoryAutomationCondition>,
    /// <p>The identifier of the answer option.</p>
    #[doc(hidden)]
    pub option_ref_id: ::std::option::Option<::std::string::String>,
}
impl SingleSelectQuestionRuleCategoryAutomation {
    /// <p> The category name, as defined in Rules.</p>
    pub fn category(&self) -> ::std::option::Option<&str> {
        self.category.as_deref()
    }
    /// <p>The condition to apply for the automation option. If the condition is <code>PRESENT</code>, then the option is applied when the contact data includes the category. Similarly, if the condition is <code>NOT_PRESENT</code>, then the option is applied when the contact data does not include the category.</p>
    pub fn condition(
        &self,
    ) -> ::std::option::Option<&crate::types::SingleSelectQuestionRuleCategoryAutomationCondition>
    {
        self.condition.as_ref()
    }
    /// <p>The identifier of the answer option.</p>
    pub fn option_ref_id(&self) -> ::std::option::Option<&str> {
        self.option_ref_id.as_deref()
    }
}
impl SingleSelectQuestionRuleCategoryAutomation {
    /// Creates a new builder-style object to manufacture [`SingleSelectQuestionRuleCategoryAutomation`](crate::types::SingleSelectQuestionRuleCategoryAutomation).
    pub fn builder() -> crate::types::builders::SingleSelectQuestionRuleCategoryAutomationBuilder {
        crate::types::builders::SingleSelectQuestionRuleCategoryAutomationBuilder::default()
    }
}

/// A builder for [`SingleSelectQuestionRuleCategoryAutomation`](crate::types::SingleSelectQuestionRuleCategoryAutomation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SingleSelectQuestionRuleCategoryAutomationBuilder {
    pub(crate) category: ::std::option::Option<::std::string::String>,
    pub(crate) condition:
        ::std::option::Option<crate::types::SingleSelectQuestionRuleCategoryAutomationCondition>,
    pub(crate) option_ref_id: ::std::option::Option<::std::string::String>,
}
impl SingleSelectQuestionRuleCategoryAutomationBuilder {
    /// <p> The category name, as defined in Rules.</p>
    pub fn category(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.category = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The category name, as defined in Rules.</p>
    pub fn set_category(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.category = input;
        self
    }
    /// <p>The condition to apply for the automation option. If the condition is <code>PRESENT</code>, then the option is applied when the contact data includes the category. Similarly, if the condition is <code>NOT_PRESENT</code>, then the option is applied when the contact data does not include the category.</p>
    pub fn condition(
        mut self,
        input: crate::types::SingleSelectQuestionRuleCategoryAutomationCondition,
    ) -> Self {
        self.condition = ::std::option::Option::Some(input);
        self
    }
    /// <p>The condition to apply for the automation option. If the condition is <code>PRESENT</code>, then the option is applied when the contact data includes the category. Similarly, if the condition is <code>NOT_PRESENT</code>, then the option is applied when the contact data does not include the category.</p>
    pub fn set_condition(
        mut self,
        input: ::std::option::Option<
            crate::types::SingleSelectQuestionRuleCategoryAutomationCondition,
        >,
    ) -> Self {
        self.condition = input;
        self
    }
    /// <p>The identifier of the answer option.</p>
    pub fn option_ref_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.option_ref_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the answer option.</p>
    pub fn set_option_ref_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.option_ref_id = input;
        self
    }
    /// Consumes the builder and constructs a [`SingleSelectQuestionRuleCategoryAutomation`](crate::types::SingleSelectQuestionRuleCategoryAutomation).
    pub fn build(self) -> crate::types::SingleSelectQuestionRuleCategoryAutomation {
        crate::types::SingleSelectQuestionRuleCategoryAutomation {
            category: self.category,
            condition: self.condition,
            option_ref_id: self.option_ref_id,
        }
    }
}
