// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartConfigRulesEvaluationInput {
    /// <p>The list of names of Config rules that you want to run evaluations for.</p>
    #[doc(hidden)]
    pub config_rule_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl StartConfigRulesEvaluationInput {
    /// <p>The list of names of Config rules that you want to run evaluations for.</p>
    pub fn config_rule_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.config_rule_names.as_deref()
    }
}
impl StartConfigRulesEvaluationInput {
    /// Creates a new builder-style object to manufacture [`StartConfigRulesEvaluationInput`](crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput).
    pub fn builder() -> crate::operation::start_config_rules_evaluation::builders::StartConfigRulesEvaluationInputBuilder{
        crate::operation::start_config_rules_evaluation::builders::StartConfigRulesEvaluationInputBuilder::default()
    }
}

/// A builder for [`StartConfigRulesEvaluationInput`](crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartConfigRulesEvaluationInputBuilder {
    pub(crate) config_rule_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl StartConfigRulesEvaluationInputBuilder {
    /// Appends an item to `config_rule_names`.
    ///
    /// To override the contents of this collection use [`set_config_rule_names`](Self::set_config_rule_names).
    ///
    /// <p>The list of names of Config rules that you want to run evaluations for.</p>
    pub fn config_rule_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.config_rule_names.unwrap_or_default();
        v.push(input.into());
        self.config_rule_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of names of Config rules that you want to run evaluations for.</p>
    pub fn set_config_rule_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.config_rule_names = input;
        self
    }
    /// Consumes the builder and constructs a [`StartConfigRulesEvaluationInput`](crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput {
                config_rule_names: self.config_rule_names,
            },
        )
    }
}
