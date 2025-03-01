// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRateBasedRuleManagedKeysInput {
    /// <p>The <code>RuleId</code> of the <code>RateBasedRule</code> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <code>ListRateBasedRules</code>.</p>
    #[doc(hidden)]
    pub rule_id: ::std::option::Option<::std::string::String>,
    /// <p>A null value and not currently used. Do not include this in your request.</p>
    #[doc(hidden)]
    pub next_marker: ::std::option::Option<::std::string::String>,
}
impl GetRateBasedRuleManagedKeysInput {
    /// <p>The <code>RuleId</code> of the <code>RateBasedRule</code> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <code>ListRateBasedRules</code>.</p>
    pub fn rule_id(&self) -> ::std::option::Option<&str> {
        self.rule_id.as_deref()
    }
    /// <p>A null value and not currently used. Do not include this in your request.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
}
impl GetRateBasedRuleManagedKeysInput {
    /// Creates a new builder-style object to manufacture [`GetRateBasedRuleManagedKeysInput`](crate::operation::get_rate_based_rule_managed_keys::GetRateBasedRuleManagedKeysInput).
    pub fn builder() -> crate::operation::get_rate_based_rule_managed_keys::builders::GetRateBasedRuleManagedKeysInputBuilder{
        crate::operation::get_rate_based_rule_managed_keys::builders::GetRateBasedRuleManagedKeysInputBuilder::default()
    }
}

/// A builder for [`GetRateBasedRuleManagedKeysInput`](crate::operation::get_rate_based_rule_managed_keys::GetRateBasedRuleManagedKeysInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRateBasedRuleManagedKeysInputBuilder {
    pub(crate) rule_id: ::std::option::Option<::std::string::String>,
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
}
impl GetRateBasedRuleManagedKeysInputBuilder {
    /// <p>The <code>RuleId</code> of the <code>RateBasedRule</code> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <code>ListRateBasedRules</code>.</p>
    pub fn rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rule_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>RuleId</code> of the <code>RateBasedRule</code> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <code>ListRateBasedRules</code>.</p>
    pub fn set_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rule_id = input;
        self
    }
    /// <p>A null value and not currently used. Do not include this in your request.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A null value and not currently used. Do not include this in your request.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// Consumes the builder and constructs a [`GetRateBasedRuleManagedKeysInput`](crate::operation::get_rate_based_rule_managed_keys::GetRateBasedRuleManagedKeysInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_rate_based_rule_managed_keys::GetRateBasedRuleManagedKeysInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_rate_based_rule_managed_keys::GetRateBasedRuleManagedKeysInput {
                rule_id: self.rule_id,
                next_marker: self.next_marker,
            },
        )
    }
}
