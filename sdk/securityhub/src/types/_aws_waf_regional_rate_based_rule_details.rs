// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>contains details about a rate-based rule for Regional resources. A rate-based rule provides settings to indicate when to allow, block, or count a request. Rate-based rules include the number of requests that arrive over a specified period of time.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsWafRegionalRateBasedRuleDetails {
    /// <p>The name of the metrics for the rate-based rule.</p>
    #[doc(hidden)]
    pub metric_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the rate-based rule.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The field that WAF uses to determine whether requests are likely arriving from single source and are subject to rate monitoring.</p>
    #[doc(hidden)]
    pub rate_key: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of requests that have an identical value for the field specified in <code>RateKey</code> that are allowed within a five-minute period. If the number of requests exceeds <code>RateLimit</code> and the other predicates specified in the rule are met, WAF triggers the action for the rule.</p>
    #[doc(hidden)]
    pub rate_limit: i64,
    /// <p>The unique identifier for the rate-based rule.</p>
    #[doc(hidden)]
    pub rule_id: ::std::option::Option<::std::string::String>,
    /// <p>The predicates to include in the rate-based rule.</p>
    #[doc(hidden)]
    pub match_predicates: ::std::option::Option<
        ::std::vec::Vec<crate::types::AwsWafRegionalRateBasedRuleMatchPredicate>,
    >,
}
impl AwsWafRegionalRateBasedRuleDetails {
    /// <p>The name of the metrics for the rate-based rule.</p>
    pub fn metric_name(&self) -> ::std::option::Option<&str> {
        self.metric_name.as_deref()
    }
    /// <p>The name of the rate-based rule.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The field that WAF uses to determine whether requests are likely arriving from single source and are subject to rate monitoring.</p>
    pub fn rate_key(&self) -> ::std::option::Option<&str> {
        self.rate_key.as_deref()
    }
    /// <p>The maximum number of requests that have an identical value for the field specified in <code>RateKey</code> that are allowed within a five-minute period. If the number of requests exceeds <code>RateLimit</code> and the other predicates specified in the rule are met, WAF triggers the action for the rule.</p>
    pub fn rate_limit(&self) -> i64 {
        self.rate_limit
    }
    /// <p>The unique identifier for the rate-based rule.</p>
    pub fn rule_id(&self) -> ::std::option::Option<&str> {
        self.rule_id.as_deref()
    }
    /// <p>The predicates to include in the rate-based rule.</p>
    pub fn match_predicates(
        &self,
    ) -> ::std::option::Option<&[crate::types::AwsWafRegionalRateBasedRuleMatchPredicate]> {
        self.match_predicates.as_deref()
    }
}
impl AwsWafRegionalRateBasedRuleDetails {
    /// Creates a new builder-style object to manufacture [`AwsWafRegionalRateBasedRuleDetails`](crate::types::AwsWafRegionalRateBasedRuleDetails).
    pub fn builder() -> crate::types::builders::AwsWafRegionalRateBasedRuleDetailsBuilder {
        crate::types::builders::AwsWafRegionalRateBasedRuleDetailsBuilder::default()
    }
}

/// A builder for [`AwsWafRegionalRateBasedRuleDetails`](crate::types::AwsWafRegionalRateBasedRuleDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsWafRegionalRateBasedRuleDetailsBuilder {
    pub(crate) metric_name: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) rate_key: ::std::option::Option<::std::string::String>,
    pub(crate) rate_limit: ::std::option::Option<i64>,
    pub(crate) rule_id: ::std::option::Option<::std::string::String>,
    pub(crate) match_predicates: ::std::option::Option<
        ::std::vec::Vec<crate::types::AwsWafRegionalRateBasedRuleMatchPredicate>,
    >,
}
impl AwsWafRegionalRateBasedRuleDetailsBuilder {
    /// <p>The name of the metrics for the rate-based rule.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the metrics for the rate-based rule.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric_name = input;
        self
    }
    /// <p>The name of the rate-based rule.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the rate-based rule.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The field that WAF uses to determine whether requests are likely arriving from single source and are subject to rate monitoring.</p>
    pub fn rate_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rate_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The field that WAF uses to determine whether requests are likely arriving from single source and are subject to rate monitoring.</p>
    pub fn set_rate_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rate_key = input;
        self
    }
    /// <p>The maximum number of requests that have an identical value for the field specified in <code>RateKey</code> that are allowed within a five-minute period. If the number of requests exceeds <code>RateLimit</code> and the other predicates specified in the rule are met, WAF triggers the action for the rule.</p>
    pub fn rate_limit(mut self, input: i64) -> Self {
        self.rate_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of requests that have an identical value for the field specified in <code>RateKey</code> that are allowed within a five-minute period. If the number of requests exceeds <code>RateLimit</code> and the other predicates specified in the rule are met, WAF triggers the action for the rule.</p>
    pub fn set_rate_limit(mut self, input: ::std::option::Option<i64>) -> Self {
        self.rate_limit = input;
        self
    }
    /// <p>The unique identifier for the rate-based rule.</p>
    pub fn rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rule_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the rate-based rule.</p>
    pub fn set_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rule_id = input;
        self
    }
    /// Appends an item to `match_predicates`.
    ///
    /// To override the contents of this collection use [`set_match_predicates`](Self::set_match_predicates).
    ///
    /// <p>The predicates to include in the rate-based rule.</p>
    pub fn match_predicates(
        mut self,
        input: crate::types::AwsWafRegionalRateBasedRuleMatchPredicate,
    ) -> Self {
        let mut v = self.match_predicates.unwrap_or_default();
        v.push(input);
        self.match_predicates = ::std::option::Option::Some(v);
        self
    }
    /// <p>The predicates to include in the rate-based rule.</p>
    pub fn set_match_predicates(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::AwsWafRegionalRateBasedRuleMatchPredicate>,
        >,
    ) -> Self {
        self.match_predicates = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsWafRegionalRateBasedRuleDetails`](crate::types::AwsWafRegionalRateBasedRuleDetails).
    pub fn build(self) -> crate::types::AwsWafRegionalRateBasedRuleDetails {
        crate::types::AwsWafRegionalRateBasedRuleDetails {
            metric_name: self.metric_name,
            name: self.name,
            rate_key: self.rate_key,
            rate_limit: self.rate_limit.unwrap_or_default(),
            rule_id: self.rule_id,
            match_predicates: self.match_predicates,
        }
    }
}
