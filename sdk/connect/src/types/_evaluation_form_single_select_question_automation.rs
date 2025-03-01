// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the automation configuration in single select questions. Automation options are evaluated in order, and the first matched option is applied. If no automation option matches, and there is a default option, then the default option is applied.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EvaluationFormSingleSelectQuestionAutomation {
    /// <p>The automation options of the single select question.</p>
    #[doc(hidden)]
    pub options: ::std::option::Option<
        ::std::vec::Vec<crate::types::EvaluationFormSingleSelectQuestionAutomationOption>,
    >,
    /// <p>The identifier of the default answer option, when none of the automation options match the criteria.</p>
    #[doc(hidden)]
    pub default_option_ref_id: ::std::option::Option<::std::string::String>,
}
impl EvaluationFormSingleSelectQuestionAutomation {
    /// <p>The automation options of the single select question.</p>
    pub fn options(
        &self,
    ) -> ::std::option::Option<&[crate::types::EvaluationFormSingleSelectQuestionAutomationOption]>
    {
        self.options.as_deref()
    }
    /// <p>The identifier of the default answer option, when none of the automation options match the criteria.</p>
    pub fn default_option_ref_id(&self) -> ::std::option::Option<&str> {
        self.default_option_ref_id.as_deref()
    }
}
impl EvaluationFormSingleSelectQuestionAutomation {
    /// Creates a new builder-style object to manufacture [`EvaluationFormSingleSelectQuestionAutomation`](crate::types::EvaluationFormSingleSelectQuestionAutomation).
    pub fn builder() -> crate::types::builders::EvaluationFormSingleSelectQuestionAutomationBuilder
    {
        crate::types::builders::EvaluationFormSingleSelectQuestionAutomationBuilder::default()
    }
}

/// A builder for [`EvaluationFormSingleSelectQuestionAutomation`](crate::types::EvaluationFormSingleSelectQuestionAutomation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EvaluationFormSingleSelectQuestionAutomationBuilder {
    pub(crate) options: ::std::option::Option<
        ::std::vec::Vec<crate::types::EvaluationFormSingleSelectQuestionAutomationOption>,
    >,
    pub(crate) default_option_ref_id: ::std::option::Option<::std::string::String>,
}
impl EvaluationFormSingleSelectQuestionAutomationBuilder {
    /// Appends an item to `options`.
    ///
    /// To override the contents of this collection use [`set_options`](Self::set_options).
    ///
    /// <p>The automation options of the single select question.</p>
    pub fn options(
        mut self,
        input: crate::types::EvaluationFormSingleSelectQuestionAutomationOption,
    ) -> Self {
        let mut v = self.options.unwrap_or_default();
        v.push(input);
        self.options = ::std::option::Option::Some(v);
        self
    }
    /// <p>The automation options of the single select question.</p>
    pub fn set_options(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::EvaluationFormSingleSelectQuestionAutomationOption>,
        >,
    ) -> Self {
        self.options = input;
        self
    }
    /// <p>The identifier of the default answer option, when none of the automation options match the criteria.</p>
    pub fn default_option_ref_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.default_option_ref_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the default answer option, when none of the automation options match the criteria.</p>
    pub fn set_default_option_ref_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.default_option_ref_id = input;
        self
    }
    /// Consumes the builder and constructs a [`EvaluationFormSingleSelectQuestionAutomation`](crate::types::EvaluationFormSingleSelectQuestionAutomation).
    pub fn build(self) -> crate::types::EvaluationFormSingleSelectQuestionAutomation {
        crate::types::EvaluationFormSingleSelectQuestionAutomation {
            options: self.options,
            default_option_ref_id: self.default_option_ref_id,
        }
    }
}
